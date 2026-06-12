//! # Vine::Client::Shared
//!
//! Module-private state for the Vine client: connection pool, per-
//! connection metadata, the broadcast fan-out, the shutdown flag, plus
//! the constants and message-size validator every entry-point shares.

use std::{
	collections::HashMap,
	sync::{
		Arc,
		OnceLock,
		atomic::{AtomicBool, Ordering},
	},
	time::Instant,
};

use lazy_static::lazy_static;
use parking_lot::Mutex;
use tokio::sync::Notify;

use crate::{Client::NotificationFrame, Error::VineError, Generated::cocoon_service_client::CocoonServiceClient};

/// Cocoon gRPC client over a tonic transport channel.
pub type CocoonClient = CocoonServiceClient<tonic::transport::Channel>;

/// Default timeout for RPC calls, in milliseconds.
pub const DEFAULT_TIMEOUT_MS:u64 = 5000;

/// Maximum number of retry attempts for failed connections.
///
/// Air's gRPC server takes ~150-500 ms to bind after the process spawns,
/// so 10 attempts at 200 ms base gives a ~5 s window (2^0+…+2^9 × 200 ms,
/// capped by the Vine client's exponential backoff).
pub const MAX_RETRY_ATTEMPTS:usize = 10;

/// Base delay between retry attempts, in milliseconds.
pub const RETRY_BASE_DELAY_MS:u64 = 200;

/// Ceiling on the exponential retry backoff, in milliseconds.
///
/// Without this clamp the doubling series reaches ~102 s by attempt 9
/// and holds boot for >3 min on a dead sidecar.
pub const MAX_BACKOFF_MS:u64 = 5000;

/// Maximum message size for validation (4 MB to match the tonic default).
pub const MAX_MESSAGE_SIZE_BYTES:usize = 4 * 1024 * 1024;

/// Health-check interval, in milliseconds.
pub const HEALTH_CHECK_INTERVAL_MS:u64 = 30000;

/// Connection timeout (currently unused — kept for the streaming variant).
pub const CONNECTION_TIMEOUT_MS:u64 = 10000;

/// Notification broadcast capacity (drop-oldest when full).
///
/// 4096 covers the worst-case storms (sky://diagnostics/changed at
/// 50-200/s during rust-analyzer cargo-check) with margin.
pub const NOTIFICATION_BROADCAST_CAPACITY:usize = 4096;

/// Per-sidecar connection metadata tracking health and last activity.
pub struct ConnectionMetadata {
	/// Instant of the most recent successful RPC activity.
	pub LastActivity:Instant,

	/// Consecutive failure count since last success.
	pub FailureCount:usize,

	/// Whether the connection is currently considered healthy.
	pub IsHealthy:bool,
}

lazy_static! {
	/// Connection pool mapping sidecar identifiers to their gRPC clients.
	///
	/// Populated by `ConnectToSideCar` on success; consumed by
	/// `SendRequest`, `SendNotification`, and `DisconnectFromSideCar`.
	pub static ref SIDECAR_CLIENTS: Arc<Mutex<HashMap<String, CocoonClient>>> = Arc::new(Mutex::new(HashMap::new()));

	/// Per-connection metadata keyed by sidecar identifier.
	///
	/// Tracks activity timestamps, failure counts, and health flags
	/// used by `CheckSideCarHealth` and the failure-eviction logic in
	/// `RecordSideCarFailure`.
	pub static ref CONNECTION_METADATA: Arc<Mutex<HashMap<String, ConnectionMetadata>>> =
		Arc::new(Mutex::new(HashMap::new()));

	/// Global notification broadcast channel.
	///
	/// Every successful wire send publishes a `NotificationFrame` here
	/// so broadcast subscribers (Effect-TS fibers, OTel emitters, future
	/// Mist-WS bridge, dev log) observe the flow concurrently.
	pub static ref NOTIFICATION_BROADCAST: tokio::sync::broadcast::Sender<NotificationFrame::Struct> = {
		let (Sender, _) = tokio::sync::broadcast::channel(NOTIFICATION_BROADCAST_CAPACITY);

		Sender
	};
}

/// Per-sidecar connection-ready notifiers. Keyed by the sidecar identifier
/// (e.g. `"cocoon-main"`). Callers that need the connection before issuing an
/// RPC can `await` the `Notify` instead of polling `IsClientConnected`.
/// The `Notify` is created lazily on first `GetConnectionNotify` call and
/// fired (`notify_waiters`) once in `ConnectToSideCar` after a successful
/// handshake. Subsequent calls see a pre-fired notifier and wake immediately.
static CONNECTION_NOTIFIERS:OnceLock<Arc<parking_lot::RwLock<HashMap<String, Arc<Notify>>>>> = OnceLock::new();

/// Retrieves (or lazily creates) the connection-ready notifier for a sidecar.
///
/// # Parameters
///
/// * `SideCarIdentifier` — identifies the sidecar whose notifier to fetch.
pub fn GetConnectionNotify(SideCarIdentifier:&str) -> Arc<Notify> {
	let Map = CONNECTION_NOTIFIERS.get_or_init(|| Arc::new(parking_lot::RwLock::new(HashMap::new())));

	{
		let Read = Map.read();

		if let Some(Notify) = Read.get(SideCarIdentifier) {
			return Notify.clone();
		}
	}

	let mut Write = Map.write();

	Write
		.entry(SideCarIdentifier.to_string())
		.or_insert_with(|| Arc::new(Notify::new()))
		.clone()
}

/// Wakes all waiters on the connection-ready notifier for a sidecar.
///
/// Called by `ConnectToSideCar` after a successful handshake to unblock
/// any `WaitForClientConnection` callers.
///
/// # Parameters
///
/// * `SideCarIdentifier` — identifies the sidecar whose notifier to fire.
pub fn FireConnectionNotify(SideCarIdentifier:&str) {
	if let Some(Map) = CONNECTION_NOTIFIERS.get() {
		if let Some(Notifier) = Map.read().get(SideCarIdentifier) {
			Notifier.notify_waiters();
		}
	}
}

/// Process-wide shutdown flag.
///
/// Set to `true` once the embedder has issued `$shutdown` (or SIGKILL'd)
/// the sidecar. After that point all `SendNotification` / `SendRequest`
/// calls short-circuit.
pub static SHUTDOWN_FLAG:AtomicBool = AtomicBool::new(false);

/// Stores a value in the process-wide shutdown flag.
///
/// # Parameters
///
/// * `Value` — `true` to mark the client as shutting down.
pub fn ShutdownFlagStore(Value:bool) { SHUTDOWN_FLAG.store(Value, Ordering::Relaxed); }

/// Loads the current value of the process-wide shutdown flag.
pub fn ShutdownFlagLoad() -> bool { SHUTDOWN_FLAG.load(Ordering::Relaxed) }

/// Records a sidecar failure and evicts the pooled client after
/// `MAX_RETRY_ATTEMPTS` consecutive failures.
///
/// Increments the failure counter and marks the connection unhealthy.
/// Once the counter passes `MAX_RETRY_ATTEMPTS` the pooled client is
/// evicted from `SIDECAR_CLIENTS` so the next caller gets
/// `ClientNotConnected` and triggers a fresh reconnect instead of
/// hammering a dead channel.
///
/// # Parameters
///
/// * `SideCarIdentifier` — identifies the sidecar that experienced a failure.
pub fn RecordSideCarFailure(SideCarIdentifier:&str) {
	let ShouldEvict = {
		let mut Metadata = CONNECTION_METADATA.lock();

		if let Some(Connection) = Metadata.get_mut(SideCarIdentifier) {
			Connection.FailureCount += 1;

			Connection.IsHealthy = false;

			Connection.FailureCount > MAX_RETRY_ATTEMPTS
		} else {
			false
		}
	};

	if ShouldEvict && SIDECAR_CLIENTS.lock().remove(SideCarIdentifier).is_some() {
		crate::dev_log!(
			"grpc",
			"warn: [VineClient] evicting pooled client for sidecar '{}' after {} consecutive failures",
			SideCarIdentifier,
			MAX_RETRY_ATTEMPTS
		);
	}
}

/// Refreshes the last-activity timestamp and resets the failure counter
/// for a sidecar.
///
/// # Parameters
///
/// * `SideCarIdentifier` — identifies the sidecar whose metadata to update.
pub fn UpdateSideCarActivity(SideCarIdentifier:&str) {
	let mut Metadata = CONNECTION_METADATA.lock();

	if let Some(Connection) = Metadata.get_mut(SideCarIdentifier) {
		Connection.LastActivity = Instant::now();

		Connection.FailureCount = 0;

		Connection.IsHealthy = true;
	}
}

/// Reports a notification subscriber falling behind the broadcast channel.
///
/// `NOTIFICATION_BROADCAST` drops the oldest frames when a subscriber lags
/// (`broadcast::error::RecvError::Lagged`); recv loops call this with the
/// skipped-frame count so the loss is visible instead of silent.
///
/// # Parameters
///
/// * `SubscriberIdentity` — label identifying the lagging subscriber.
/// * `SkippedFrames` — number of frames that were dropped.
pub fn ReportNotificationLag(SubscriberIdentity:&str, SkippedFrames:u64) {
	crate::dev_log!(
		"grpc",
		"warn: [VineClient] notification subscriber '{}' lagged; {} frame(s) dropped by the broadcast channel",
		SubscriberIdentity,
		SkippedFrames
	);
}

/// Validates that a byte payload does not exceed the maximum message size.
///
/// Rejects messages above `MAX_MESSAGE_SIZE_BYTES` to bound the worst-case
/// gRPC frame. Mirrors tonic's own check so callers don't pay the codec
/// round-trip for an oversize payload.
///
/// # Parameters
///
/// * `Data` — byte slice to validate.
///
/// # Errors
///
/// Returns `VineError::MessageTooLarge` when the payload exceeds the limit.
pub fn ValidateMessageSize(Data:&[u8]) -> Result<(), VineError> {
	if Data.len() > MAX_MESSAGE_SIZE_BYTES {
		Err(VineError::MessageTooLarge { ActualSize:Data.len(), MaxSize:MAX_MESSAGE_SIZE_BYTES })
	} else {
		Ok(())
	}
}
