//! Cocoon `outputChannel.dispose` notification. Relays the payload to
//! Sky as `sky://output/dispose`.

use serde_json::Value;

use crate::{Host::VineHost, Server::Notification::Support::RelayToSky};

pub async fn OutputChannelDispose(Host:&dyn VineHost, Parameter:&Value) {
	RelayToSky::Fn(Host, "sky://output/dispose", Parameter, "grpc", "[OutputChannel] dispose");
}
