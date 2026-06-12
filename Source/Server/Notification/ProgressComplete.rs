//! Extension Host `progress.complete` notification. Relays the payload to Sky
//! as `sky://progress/complete`.

use serde_json::Value;

use crate::{Host::VineHost, Server::Notification::Support::RelayToSky};

/// Handles : `progress.complete` Relays the payload to Sky as
/// `sky://progress/complete`..
pub async fn ProgressComplete(Host:&dyn VineHost, Parameter:&Value) {
	RelayToSky::Fn(Host, "sky://progress/complete", Parameter, "grpc", "[Progress] complete");
}
