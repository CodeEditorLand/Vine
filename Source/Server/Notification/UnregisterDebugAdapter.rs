//! `debug_adapter` provider-unregistration atom.

use serde_json::Value;

use crate::{Host::VineHost, Server::Notification::Support::UnregisterByHandle};

pub async fn UnregisterDebugAdapter(Host:&dyn VineHost, Parameter:&Value) {

	UnregisterByHandle::UnregisterByHandle(Host, Parameter, "debug_adapter");
}
