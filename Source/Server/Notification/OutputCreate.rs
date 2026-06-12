//! Extension Host `output.create` notification. Relays the payload to Sky as
//! `sky://output/create`.

use serde_json::Value;

use crate::{Host::VineHost, Server::Notification::Support::RelayToSky};

/// Handles `output.create` by forwarding the payload as a renderer event.
pub async fn OutputCreate(Host:&dyn VineHost, Parameter:&Value) {
	RelayToSky::Fn(Host, "sky://output/create", Parameter, "grpc", "[Output] create");
}
