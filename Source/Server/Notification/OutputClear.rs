//! Extension Host `output.clear` notification. Relays the payload to Sky as
//! `sky://output/clear`.

use serde_json::Value;

use crate::{Host::VineHost, Server::Notification::Support::RelayToSky};

/// Handles `output.clear` by forwarding the payload as a renderer event.
pub async fn OutputClear(Host:&dyn VineHost, Parameter:&Value) {
	RelayToSky::Fn(Host, "sky://output/clear", Parameter, "grpc", "[Output] clear");
}
