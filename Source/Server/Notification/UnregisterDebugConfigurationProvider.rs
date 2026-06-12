//! `debug_configuration` provider-unregistration atom.

use serde_json::Value;

use crate::{Host::VineHost, Server::Notification::Support::UnregisterByHandle};

/// Handles : `debug_configuration` provider-unregistration atom..
pub async fn UnregisterDebugConfigurationProvider(Host:&dyn VineHost, Parameter:&Value) {
	UnregisterByHandle::UnregisterByHandle(Host, Parameter, "debug_configuration");
}
