//! Extension Host `webview.dispose` notification - extension disposed a
//! webview panel or the user closed the tab. Sky's webview shim
//! listens on `sky://webview/dispose` and tears down the DOM container
//! and unregisters the handle.

use serde_json::Value;

use crate::{Host::VineHost, dev_log};

/// Handles : `webview.dispose` notification: extension disposed a webview panel
/// or the user closed the tab. Sky's webview shim listens on
/// `sky://webview/dispose` and tears down the DOM container and unregisters the
/// handle..
pub async fn WebviewDispose(Host:&dyn VineHost, Parameter:&Value) {
	Host.EmitToRenderer("sky://webview/dispose", Parameter.clone());

	dev_log!(
		"grpc",
		"[Webview] dispose handle={}",
		Parameter.get("handle").and_then(Value::as_str).unwrap_or("?")
	);
}
