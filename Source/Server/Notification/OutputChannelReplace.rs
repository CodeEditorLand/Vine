//! Extension Host `outputChannel.replace` notification. Atomic buffer
//! replacement: equivalent to `clear` + `append`, but rendered as a
//! single workbench frame so the user does not see an empty flash.

use serde_json::Value;

use crate::{Host::VineHost, Server::Notification::Support::RelayToSky};

/// Handles : `outputChannel.replace` Atomic buffer replacement: equivalent to
/// `clear` + `append`, but rendered as a single workbench frame so the user
/// does not see an empty flash..
pub async fn OutputChannelReplace(Host:&dyn VineHost, Parameter:&Value) {
	RelayToSky::Fn(Host, "sky://output/replace", Parameter, "grpc", "[OutputChannel] replace");
}
