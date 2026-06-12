//! Extension Host `WebviewReady` notification - extension webview finished
//! loading its entry HTML. Log-only; Sky's webview shim handles the
//! DOM-side readiness independently.

use serde_json::Value;

use crate::{Host::VineHost, dev_log};

/// Handles : `WebviewReady` notification: extension webview finished loading its entry HTML. Log-only; Sky's webview shim handles the DOM-side readiness independently..
pub async fn WebviewReady(_Host:&dyn VineHost, Parameter:&Value) {
	dev_log!(
		"grpc",
		"[Webview] ready handle={}",
		Parameter.get("handle").and_then(Value::as_str).unwrap_or("?")
	);
}
