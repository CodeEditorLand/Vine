//! Extension Host → `statusBar.update` / `statusBar.dispose` notifications.
//! Derives the Sky event name from the wire method suffix and emits on
//! `sky://statusbar/<suffix>`. Canonical prefix is `sky://statusbar/` (no
//! hyphen) to match every other emit site in the statusbar group.

use serde_json::Value;

use crate::{Host::VineHost, dev_log};

/// Handles : → `statusBar.update` / `statusBar.dispose` notifications. Derives the Sky event name from the wire method suffix and emits on `sky://statusbar/<suffix>`. Canonical prefix is `sky://statusbar/` (no hyphen) to match every other emit site in the statusbar group..
pub async fn StatusBarLifecycle(Host:&dyn VineHost, MethodName:&str, Parameter:&Value) {
	let EventName = format!("sky://statusbar/{}", &MethodName["statusBar.".len()..]);

	dev_log!("grpc", "[StatusBarLifecycle] emit {}", EventName);

	Host.EmitToRenderer(&EventName, Parameter.clone());
}
