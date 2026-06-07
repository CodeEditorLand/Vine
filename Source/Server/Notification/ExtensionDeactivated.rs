//! Cocoon `ExtensionDeactivated` notification - log-only. Wind listens
//! on `cocoon:extensionActivated` for the positive half; extensions
//! rarely deactivate at runtime outside uninstall.

use serde_json::Value;

use crate::{Host::VineHost, dev_log};

pub async fn ExtensionDeactivated(_Host:&dyn VineHost, Parameter:&Value) {
	dev_log!(
		"grpc",
		"[Extension] deactivated id={}",
		Parameter.get("extensionId").and_then(Value::as_str).unwrap_or("?")
	);
}
