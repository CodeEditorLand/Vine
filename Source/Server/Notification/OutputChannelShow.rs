use serde_json::Value;

use crate::{Host::VineHost, Server::Notification::Support::RelayToSky};

pub async fn OutputChannelShow(Host:&dyn VineHost, Parameter:&Value) {
	RelayToSky::Fn(Host, "sky://output/show", Parameter, "grpc", "[OutputChannel] show");
}
