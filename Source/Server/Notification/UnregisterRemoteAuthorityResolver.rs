//! `remote_authority_resolver` provider-unregistration atom.

use serde_json::Value;

use crate::{Host::VineHost, Server::Notification::Support::UnregisterByHandle};

pub async fn UnregisterRemoteAuthorityResolver(Host:&dyn VineHost, Parameter:&Value) {

	UnregisterByHandle::UnregisterByHandle(Host, Parameter, "remote_authority_resolver");
}
