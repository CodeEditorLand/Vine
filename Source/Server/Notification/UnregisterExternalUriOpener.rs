//! `external_uri_opener` provider-unregistration atom.

use serde_json::Value;

use crate::{Host::VineHost, Server::Notification::Support::UnregisterByHandle};

pub async fn UnregisterExternalUriOpener(Host:&dyn VineHost, Parameter:&Value) {

	UnregisterByHandle::UnregisterByHandle(Host, Parameter, "external_uri_opener");
}
