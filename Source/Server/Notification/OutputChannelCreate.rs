//! Cocoon `outputChannel.create` notification - twin of `output.create`.
//! See `OutputCreate.rs` for the duplicate-wire rationale.

use serde_json::Value;

use crate::{Host::VineHost, Server::Notification::Support::RelayToSky};

pub async fn OutputChannelCreate(Host:&dyn VineHost, Parameter:&Value) {
	RelayToSky::Fn(
		Host,
		"sky://output/create",
		Parameter,
		"output-verbose",
		"[OutputChannel] create",
	);
}
