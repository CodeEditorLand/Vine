//! Extension Host `progress.end` notification. Fires once per
//! `vscode.window.withProgress(...)` call when the task settles.
//! Forwarded onto `sky://notification/progress-end`.

use serde_json::{Value, json};

use crate::Host::VineHost;

/// Handles : `progress.end` Fires once per `vscode.window.withProgress(...)` call when the task settles. Forwarded onto `sky://notification/progress-end`..
pub async fn ProgressEnd(Host:&dyn VineHost, Parameter:&Value) {
	let Handle = Parameter.get("handle").and_then(Value::as_str).unwrap_or("");

	Host.EmitToRenderer("sky://notification/progress-end", json!({ "id": Handle }));
}
