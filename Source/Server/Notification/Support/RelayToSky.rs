//! Shared helper for notification atoms that are pure renderer-event
//! relays.
//!
//! Many Cocoon → Mountain notification atoms do exactly two things:
//!
//! 1. `Host.EmitToRenderer(SkyEvent, Parameter)`
//! 2. `dev_log!(tag, "...")`
//!
//! This function collapses that pair so each such atom is a one-liner. It
//! takes `&dyn VineHost`, decoupling the handler tree from any specific
//! embedder runtime.
//!
//! - `SkyEvent` - the `sky://…` renderer event name.
//! - `LogTag`   - dev-log tag (`"grpc"`, `"output-verbose"`, …).
//! - `LogLine`  - pre-formatted message; skipped when empty.

use serde_json::Value;

use crate::{Host::VineHost, dev_log};

pub fn Fn(Host:&dyn VineHost, SkyEvent:&str, Parameter:&Value, LogTag:&str, LogLine:&str) {
	Host.EmitToRenderer(SkyEvent, Parameter.clone());

	if !LogLine.is_empty() {
		dev_log!(LogTag, "{}", LogLine);
	}
}
