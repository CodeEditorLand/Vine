//! Await Cocoon's gRPC connection without polling. `GetConnectionNotify`
//! returns a shared `tokio::sync::Notify` that `ConnectToSideCar` fires
//! once the handshake succeeds; `WaitForClientConnection` awaits it in
//! short slices and re-checks `IsClientConnected` between them. The
//! re-check closes the lost-wakeup window: `notify_waiters` only wakes
//! waiters already registered, so a waiter that subscribes after the
//! handshake fired would otherwise block for the whole budget even
//! though the client is in the pool.
//!
//! `BudgetMilliseconds` remains the hard cap so call sites keep their
//! existing behaviour for the pathological "Cocoon never starts" case.

use std::time::Duration;

use crate::Client::{IsClientConnected, IsShuttingDown, Shared::GetConnectionNotify};

/// Upper bound on a single `notified()` slice before re-checking the
/// connected flag. Wakeups via `FireConnectionNotify` still land
/// immediately; the slice only bounds the lost-wakeup worst case.
const RECHECK_INTERVAL_MS:u64 = 100;

/// Await Cocoon's gRPC connection without polling. `GetConnectionNotify` returns a shared `tokio::sync::Notify` that `Conne
///
pub async fn Fn(SideCarIdentifier:&str, BudgetMilliseconds:u64) -> bool {
	if IsShuttingDown::Fn() {
		return false;
	}

	if IsClientConnected::Fn(SideCarIdentifier) {
		return true;
	}

	let Notifier = GetConnectionNotify(SideCarIdentifier);

	let Deadline = tokio::time::Instant::now() + Duration::from_millis(BudgetMilliseconds);

	loop {
		if IsClientConnected::Fn(SideCarIdentifier) {
			return true;
		}

		if IsShuttingDown::Fn() {
			return false;
		}

		let Remaining = Deadline.saturating_duration_since(tokio::time::Instant::now());

		if Remaining.is_zero() {
			// Budget expired - do a final check in case the connection
			// landed in the window since the last re-check.
			return IsClientConnected::Fn(SideCarIdentifier);
		}

		let Slice = Remaining.min(Duration::from_millis(RECHECK_INTERVAL_MS));

		let _ = tokio::time::timeout(Slice, Notifier.notified()).await;
	}
}
