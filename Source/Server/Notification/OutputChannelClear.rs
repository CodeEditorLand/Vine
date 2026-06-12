//! Extension Host `outputChannel.clear` notification. Relays the payload to Sky
//! as `sky://output/clear`.

use serde_json::Value;

use crate::{Host::VineHost, Server::Notification::Support::RelayToSky};

/// Handles : `outputChannel.clear` Relays the payload to Sky as
/// `sky://output/clear`..
pub async fn OutputChannelClear(Host:&dyn VineHost, Parameter:&Value) {
	RelayToSky::Fn(Host, "sky://output/clear", Parameter, "grpc", "[OutputChannel] clear");
}
