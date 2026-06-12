//! Extension Host `output.dispose` notification. Relays the payload to Sky as
//! `sky://output/dispose`.

use serde_json::Value;

use crate::{Host::VineHost, Server::Notification::Support::RelayToSky};

/// Handles `output.dispose` by forwarding the payload as a renderer event.
pub async fn OutputDispose(Host:&dyn VineHost, Parameter:&Value) {
	RelayToSky::Fn(Host, "sky://output/dispose", Parameter, "grpc", "[Output] dispose");
}
