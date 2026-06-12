//! Extension Host `openExternal` notification - extension called
//! `vscode.env.openExternal(uri)`. Delegates to the platform default
//! handler via the `opener` crate. Fire-and-forget; success/failure is
//! logged but not surfaced back to the extension.

use serde_json::Value;

use crate::{Host::VineHost, dev_log};

/// Handles : `openExternal` notification: extension called
/// `vscode.env.openExternal(uri)`. Delegates to the platform default handler
/// via the `opener` crate. Fire-and-forget; success/failure is logged but not
/// surfaced back to the extension..
pub async fn OpenExternal(_Host:&dyn VineHost, Parameter:&Value) {
	let Uri = Parameter.get("uri").and_then(Value::as_str).unwrap_or("");

	if Uri.is_empty() {
		dev_log!("grpc", "[OpenExternal] skip: missing uri");

		return;
	}

	match open::that(Uri) {
		Ok(()) => dev_log!("grpc", "[OpenExternal] uri={} ok", Uri),

		Err(Error) => dev_log!("grpc", "[OpenExternal] uri={} err={}", Uri, Error),
	}
}
