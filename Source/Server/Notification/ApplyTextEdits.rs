//! Extension Host → `window.applyTextEdits` notification.
//! Emitted when an extension calls `editor.edit(editBuilder => {...})`.
//! Cocoon's TextEditor shim collects the edits and sends them here.
//! Mountain relays `sky://editor/apply-text-edits` so Sky can apply them
//! via `ICodeEditorService.listCodeEditors()` → `editor.executeEdits(...)`.

use serde_json::Value;

use crate::{Host::VineHost, dev_log};

/// Handles : → `window.applyTextEdits` Emitted when an extension calls `editor.edit(editBuilder => {...})`. Cocoon's TextEditor shim collects the edits and sends them here. Mountain relays `sky://editor/apply-text-edits` so Sky can apply them via `ICodeEditorService.listCodeEditors()` → `editor.executeEdits(...)`..
pub async fn ApplyTextEdits(Host:&dyn VineHost, Parameter:&Value) {
	let Uri = Parameter.get("uri").and_then(Value::as_str).unwrap_or("").to_string();

	let EditCount = Parameter.get("edits").and_then(Value::as_array).map(|A| A.len()).unwrap_or(0);

	dev_log!("model", "[ApplyTextEdits] uri={} edits={}", Uri, EditCount);

	if Uri.is_empty() || EditCount == 0 {
		return;
	}

	Host.EmitToRenderer("sky://editor/apply-text-edits", Parameter.clone());
}
