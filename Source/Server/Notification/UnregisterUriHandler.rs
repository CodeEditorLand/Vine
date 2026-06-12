//! `uri_handler` provider-unregistration atom.
//! Logs the bound scheme alongside the handle for traceability.

use serde_json::Value;

use crate::{Host::VineHost, Server::Notification::Support::UnregisterByHandle, dev_log};

/// Handles : `uri_handler` provider-unregistration atom. Logs the bound scheme alongside the handle for traceability..
pub async fn UnregisterUriHandler(Host:&dyn VineHost, Parameter:&Value) {
	let Scheme = Parameter.get("scheme").and_then(Value::as_str).unwrap_or("");

	dev_log!("provider-register", "[ProviderUnregister] uri_handler scheme={}", Scheme);

	UnregisterByHandle::UnregisterByHandle(Host, Parameter, "uri_handler");
}
