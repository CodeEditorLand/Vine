//! Extension Host → `statusBar.message` notification.
//! Emitted when an extension calls `vscode.window.setStatusBarMessage`
//! (one-shot text, optional auto-hide). Canonical channel is
//! `sky://statusbar/set-message`.

use serde_json::{Value, json};

use crate::{Host::VineHost, dev_log};

/// Handles : → `statusBar.message` Emitted when an extension calls `vscode.window.setStatusBarMessage` (one-shot text, optional auto-hide). Canonical channel is `sky://statusbar/set-message`..
pub async fn StatusBarMessage(Host:&dyn VineHost, Parameter:&Value) {
	let Text = Parameter.get("text").and_then(Value::as_str).unwrap_or("");

	let HideAfter = Parameter.get("hideAfter").and_then(Value::as_u64);

	Host.EmitToRenderer(
		"sky://statusbar/set-message",
		json!({
			"text": Text,
			"hideAfter": HideAfter,
		}),
	);

	dev_log!("grpc", "[StatusBar] message len={}", Text.len());
}
