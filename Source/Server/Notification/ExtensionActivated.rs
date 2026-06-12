//! Extension Host `ExtensionActivated` notification - extension's `activate`
//! export resolved. Forwarded to Wind on `cocoon:extensionActivated`
//! so the Extensions sidebar updates its row state without polling.

use serde_json::Value;

use crate::Host::VineHost;

/// Handles `ExtensionActivated` by forwarding the payload on
/// `cocoon:extensionActivated` so the Extensions sidebar updates without
/// polling.
pub async fn ExtensionActivated(Host:&dyn VineHost, Parameter:&Value) {
	Host.EmitToRenderer("cocoon:extensionActivated", Parameter.clone());
}
