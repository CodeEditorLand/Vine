//! `task` provider-unregistration atom.

use serde_json::Value;

use crate::{Host::VineHost, Server::Notification::Support::UnregisterByHandle};

/// Handles : `task` provider-unregistration atom..
pub async fn UnregisterTaskProvider(Host:&dyn VineHost, Parameter:&Value) {
	UnregisterByHandle::UnregisterByHandle(Host, Parameter, "task");
}
