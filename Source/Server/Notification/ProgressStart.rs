//! Extension Host `progress.start` notification. Fires at the top of every
//! `vscode.window.withProgress(...)` call. Normalised onto
//! `sky://notification/progress-begin`.

use serde_json::{Value, json};

use crate::Host::VineHost;

/// Handles : `progress.start` Fires at the top of every `vscode.window.withProgress(...)` call. Normalised onto `sky://notification/progress-begin`..
pub async fn ProgressStart(Host:&dyn VineHost, Parameter:&Value) {
	let Handle = Parameter.get("handle").and_then(Value::as_str).unwrap_or("");

	let Title = Parameter.get("title").and_then(Value::as_str).unwrap_or("");

	let Cancellable = Parameter.get("cancellable").and_then(Value::as_bool).unwrap_or(false);

	Host.EmitToRenderer(
		"sky://notification/progress-begin",
		json!({
			"id": Handle,
			"title": Title,
			"cancellable": Cancellable,
		}),
	);
}
