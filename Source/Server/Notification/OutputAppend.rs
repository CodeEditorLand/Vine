//! Cocoon `output.append` notification. Relays the payload to Sky as
//! `sky://output/append`.

use serde_json::Value;

use crate::{Host::VineHost, Server::Notification::Support::RelayToSky};

pub async fn OutputAppend(Host:&dyn VineHost, Parameter:&Value) {
	RelayToSky::Fn(Host, "sky://output/append", Parameter, "grpc", "[Output] append");
}
