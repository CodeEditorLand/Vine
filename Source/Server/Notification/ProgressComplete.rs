use serde_json::Value;

use crate::{Host::VineHost, Server::Notification::Support::RelayToSky};

pub async fn ProgressComplete(Host:&dyn VineHost, Parameter:&Value) {

	RelayToSky::Fn(Host, "sky://progress/complete", Parameter, "grpc", "[Progress] complete");
}
