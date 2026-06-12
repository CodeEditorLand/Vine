//! Extension Host → `extensionHostMessage` notification.
//! Forwards the extension-host binary protocol reply to Wind via
//! `cocoon:extensionHostReply`. Wind's extension-host bridge consumes
//! these replies to complete pending ext-host RPC calls.

use serde_json::Value;

use crate::Host::VineHost;

/// Handles : → `extensionHostMessage` Forwards the extension-host binary
/// protocol reply to Wind via `cocoon:extensionHostReply`. Wind's
/// extension-host bridge consumes these replies to complete pending ext-host
/// RPC calls..
pub async fn ExtensionHostMessage(Host:&dyn VineHost, Parameter:&Value) {
	Host.EmitToRenderer("cocoon:extensionHostReply", Parameter.clone());
}
