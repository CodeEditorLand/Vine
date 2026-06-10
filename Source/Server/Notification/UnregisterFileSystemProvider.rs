//! `file_system` provider-unregistration atom.
//! Logs the bound scheme so routing mismatches are visible after disposal.

use serde_json::Value;

use crate::{Host::VineHost, Server::Notification::Support::UnregisterByHandle, dev_log};

pub async fn UnregisterFileSystemProvider(Host:&dyn VineHost, Parameter:&Value) {
	let Scheme = Parameter.get("scheme").and_then(Value::as_str).unwrap_or("");

	dev_log!("provider-register", "[ProviderUnregister] file_system scheme={}", Scheme);

	UnregisterByHandle::UnregisterByHandle(Host, Parameter, "file_system");
}
