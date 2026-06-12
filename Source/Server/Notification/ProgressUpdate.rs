//! Extension Host `progress.update` notification. Per-tick progress payload
//! relayed onto `sky://notification/progress-update`.

use serde_json::Value;

use crate::{Host::VineHost, Server::Notification::Support::RelayToSky};

/// Handles : `progress.update` Per-tick progress payload relayed onto
/// `sky://notification/progress-update`..
pub async fn ProgressUpdate(Host:&dyn VineHost, Parameter:&Value) {
	RelayToSky::Fn(
		Host,
		"sky://notification/progress-update",
		Parameter,
		"grpc",
		"[Progress] update",
	);
}
