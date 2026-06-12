//! One observed notification frame fanned out from `SendNotification`
//! (or, once the streaming-channel multiplexer is live, from
//! `Multiplexer`). Subscribers consume frames from the broadcast channel
//! managed by `Shared::NOTIFICATION_BROADCAST`.

use serde_json::Value;

/// A single notification frame observed by a broadcast subscriber.
#[derive(Debug, Clone)]
pub struct Struct {
	/// Identifies the originating sidecar (e.g. `"cocoon-main"`).
	pub SideCarIdentifier:String,

	/// gRPC method name for this notification (e.g.
	/// `"$onDidChangeTextDocument"`).
	pub Method:String,

	/// JSON payload deserialised from the wire.
	pub Parameters:Value,

	/// Monotonic process-relative nanosecond timestamp at fan-out time.
	///
	/// Useful for OTel span correlation without burning a
	/// `SystemTime::now()` per frame.
	pub TimestampNanos:u64,
}
