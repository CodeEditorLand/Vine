//! Cocoon `output.appendLine` notification. Appends `text` to the
//! named output channel; we suffix the newline here so the downstream
//! `sky://output/append` listener stays a single append code path (no
//! dedicated `appendLine` listener in Sky).

use serde_json::{Value, json};

use crate::{Host::VineHost, dev_log};

pub async fn OutputAppendLine(Host:&dyn VineHost, Parameter:&Value) {
	let Channel = Parameter.get("channel").and_then(Value::as_str).unwrap_or("");

	let Text = Parameter.get("text").and_then(Value::as_str).unwrap_or("");

	Host.EmitToRenderer(
		"sky://output/append",
		json!({
			"channel": Channel,
			"text": format!("{}\n", Text),
		}),
	);

	dev_log!("grpc", "[Output] appendLine channel={} bytes={}", Channel, Text.len());
}
