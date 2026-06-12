//! Cocoon `output.show` notification. Relays the payload to Sky as
//! `sky://output/show`.

use serde_json::Value;

use crate::{Host::VineHost, Server::Notification::Support::RelayToSky};

pub async fn OutputShow(Host:&dyn VineHost, Parameter:&Value) {
	RelayToSky::Fn(Host, "sky://output/show", Parameter, "grpc", "[Output] show");
}
