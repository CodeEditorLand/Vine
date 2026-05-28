//! Cocoon `webview.postMessage` notification - extension called
//! `WebviewPanel.webview.postMessage(...)`. Stock VS Code delivers
//! this as a DOM `message` event inside the webview iframe; Land emits
//! on `sky://webview/postMessage` and lets the Sky bridge relay into
//! the specific webview DOM container keyed on `{ handle, message }`.

use serde_json::Value;

use crate::{Host::VineHost, dev_log};

pub async fn WebviewPostMessage(Host:&dyn VineHost, Parameter:&Value) {
	Host.EmitToRenderer("sky://webview/postMessage", Parameter.clone());

	dev_log!(
		"grpc",
		"[Webview] postMessage handle={}",
		Parameter.get("handle").and_then(Value::as_str).unwrap_or("?")
	);
}
