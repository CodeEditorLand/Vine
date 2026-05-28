//! Cocoon → `disposeStatusBarItem` notification.
//! Forwarded onto `sky://statusbar/dispose-entry` so the Sky shim removes the
//! DOM node.

use serde_json::{Value, json};

use crate::{Host::VineHost, dev_log};

pub async fn DisposeStatusBarItem(Host:&dyn VineHost, Parameter:&Value) {
	let Id = Parameter.get("id").and_then(Value::as_str).unwrap_or("");

	if Id.is_empty() {
		dev_log!("grpc", "[StatusBar] dispose skip: missing id");

		return;
	}

	Host.EmitToRenderer("sky://statusbar/dispose-entry", json!({ "id": Id }));

	dev_log!("grpc", "[StatusBar] dispose id={}", Id);
}
