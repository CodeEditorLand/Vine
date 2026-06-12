//! Extension Host `outputChannel.hide` notification. Forwards to Sky as
//! `sky://output/show { visible: false, channel }` so the workbench
//! panel can dismiss the channel via the same handler that processes
//! `show()` calls.

use serde_json::{Value, json};

use crate::{Host::VineHost, dev_log};

/// Handles : `outputChannel.hide` Forwards to Sky as `sky://output/show { visible: false, channel }` so the workbench panel can dismiss the channel via the same handler that processes `show()` calls..
pub async fn OutputChannelHide(Host:&dyn VineHost, Parameter:&Value) {
	let Channel = Parameter
		.get("channel")
		.or_else(|| Parameter.get("name"))
		.or_else(|| Parameter.get("handle"))
		.and_then(Value::as_str)
		.unwrap_or("");

	Host.EmitToRenderer(
		"sky://output/show",
		json!({
			"visible": false,
			"channel": Channel,
		}),
	);

	dev_log!("grpc", "[OutputChannel] hide channel={}", Channel);
}
