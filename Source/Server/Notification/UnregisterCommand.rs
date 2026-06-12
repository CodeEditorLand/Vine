//! Extension Host → `unregisterCommand` notification.
//! Removes the proxied `CommandHandler` from the embedder's dispatch registry
//! so subsequent `commands.executeCommand` no longer routes back to the
//! extension. Also notifies Sky so the workbench command-service view and
//! Mountain's registry stay in sync when an extension disposes a command.

use serde_json::{Value, json};

use crate::{Host::VineHost, dev_log};

/// Handles : → `unregisterCommand` Removes the proxied `CommandHandler` from
/// the embedder's dispatch registry so subsequent `commands.executeCommand` no
/// longer routes back to the extension. Also notifies Sky so the workbench
/// command-service view and Mountain's registry stay in sync when an extension
/// disposes a command..
pub async fn UnregisterCommand(Host:&dyn VineHost, Parameter:&Value) {
	let CommandId = Parameter.get("commandId").and_then(Value::as_str).unwrap_or("");

	if CommandId.is_empty() {
		return;
	}

	Host.UnregisterCommandInRegistry(CommandId);

	dev_log!("command-register", "[UnregisterCommand] id={}", CommandId);

	Host.EmitToRenderer("sky://command/unregister", json!({ "id": CommandId, "commandId": CommandId }));
}
