//! Extension Host `output.replace` notification - swap the channel's entire
//! contents. Mapped to `clear` + `append` since Sky has no dedicated
//! `sky://output/replace` listener yet.

use serde_json::{Value, json};

use crate::{Host::VineHost, dev_log};

/// Handles : `output.replace` notification: swap the channel's entire contents. Mapped to `clear` + `append` since Sky has no dedicated `sky://output/replace` listener yet..
pub async fn OutputReplace(Host:&dyn VineHost, Parameter:&Value) {
	let Channel = Parameter.get("channel").and_then(Value::as_str).unwrap_or("");

	let Text = Parameter.get("text").and_then(Value::as_str).unwrap_or("");

	Host.EmitToRenderer("sky://output/clear", json!({ "channel": Channel }));

	Host.EmitToRenderer(
		"sky://output/append",
		json!({
			"channel": Channel,
			"text": Text,
		}),
	);

	dev_log!("grpc", "[Output] replace channel={} bytes={}", Channel, Text.len());
}
