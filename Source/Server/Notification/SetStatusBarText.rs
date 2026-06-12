//! Extension Host → `setStatusBarText` notification.
//! Pure text-only fast path for `vscode.window.setStatusBarMessage(...)`.
//! Distinct from the typed `statusBar.update` notification (which carries
//! colour/tooltip/command fields). Forwards onto `sky://statusbar/set-entry`.

use serde_json::{Value, json};

use crate::{Host::VineHost, dev_log};

/// Handles : → `setStatusBarText` Pure text-only fast path for
/// `vscode.window.setStatusBarMessage(...)`. Distinct from the typed
/// `statusBar.update` notification (which carries colour/tooltip/command
/// fields). Forwards onto `sky://statusbar/set-entry`..
pub async fn SetStatusBarText(Host:&dyn VineHost, Parameter:&Value) {
	let Id = Parameter.get("id").and_then(Value::as_str).unwrap_or("");

	let Text = Parameter.get("text").and_then(Value::as_str).unwrap_or("");

	let Tooltip = Parameter.get("tooltip").and_then(Value::as_str).unwrap_or("");

	Host.EmitToRenderer(
		"sky://statusbar/set-entry",
		json!({
			"id": Id,
			"text": Text,
			"tooltip": Tooltip,
		}),
	);

	dev_log!("grpc", "[StatusBar] set-text id={} len={}", Id, Text.len());
}
