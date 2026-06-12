//! Shared helper for notification atoms that are pure renderer-event
//! relays.
//!
//! Many Extension Host → Mountain notification atoms do exactly two things:
//!
//! 1. `Host.EmitToRenderer(SkyEvent, Parameter)`
//! 2. `dev_log!(tag, "...")`
//!
//! This function collapses that pair so each such atom is a one-liner. It
//! takes `&dyn VineHost`, decoupling the handler tree from any specific
//! embedder runtime.

use serde_json::Value;

use crate::{Host::VineHost, dev_log};

/// Collapses `EmitToRenderer` + `dev_log` into a single call for pure-relay
/// notification atoms.
///
/// # Parameters
///
/// * `Host` - the embedder host interface.
/// * `SkyEvent` - the `sky://…` renderer event name.
/// * `Parameter` - JSON payload to emit.
/// * `LogTag` - dev-log tag (`"grpc"`, `"output-verbose"`, …).
/// * `LogLine` - pre-formatted message; skipped when empty.
pub fn Fn(Host:&dyn VineHost, SkyEvent:&str, Parameter:&Value, LogTag:&str, LogLine:&str) {
	Host.EmitToRenderer(SkyEvent, Parameter.clone());

	if !LogLine.is_empty() {
		dev_log!(LogTag, "{}", LogLine);
	}
}
