//! Cocoon → `extensionHostMessage` notification.
//! Forwards the extension-host binary protocol reply to Wind via
//! `cocoon:extensionHostReply`. Wind's extension-host bridge consumes
//! these replies to complete pending ext-host RPC calls.

use serde_json::Value;

use crate::Host::VineHost;

pub async fn ExtensionHostMessage(Host:&dyn VineHost, Parameter:&Value) {
	Host.EmitToRenderer("cocoon:extensionHostReply", Parameter.clone());
}
