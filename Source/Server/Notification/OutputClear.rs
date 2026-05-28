use serde_json::Value;

use crate::{Host::VineHost, Server::Notification::Support::RelayToSky};

pub async fn OutputClear(Host:&dyn VineHost, Parameter:&Value) {
	RelayToSky::Fn(Host, "sky://output/clear", Parameter, "grpc", "[Output] clear");
}
