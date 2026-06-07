use serde_json::Value;

use crate::{Host::VineHost, Server::Notification::Support::RelayToSky};

pub async fn OutputDispose(Host:&dyn VineHost, Parameter:&Value) {

	RelayToSky::Fn(Host, "sky://output/dispose", Parameter, "grpc", "[Output] dispose");
}
