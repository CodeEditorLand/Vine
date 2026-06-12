//! `authentication` provider-unregistration atom.

use serde_json::Value;

use crate::{Host::VineHost, Server::Notification::Support::UnregisterByHandle};

/// Handles : `authentication` provider-unregistration atom..
pub async fn UnregisterAuthenticationProvider(Host:&dyn VineHost, Parameter:&Value) {
	UnregisterByHandle::UnregisterByHandle(Host, Parameter, "authentication");
}
