//! Extension Host `security.incident` notification - Cocoon-side security
//! policy flagged a breach (extension violated permission set, blocked
//! filesystem access, etc.). Land has no central security dashboard
//! yet; the atom surfaces the incident via `dev_log!` and re-emits on
//! `sky://security/incident` for future listeners.

use serde_json::Value;

use crate::{Host::VineHost, dev_log};

/// Handles : `security.incident` notification: Cocoon-side security policy flagged a breach (extension violated permission set, blocked filesystem access, etc.). Land has no central security dashboard yet; the atom surfaces the incident via `dev_log!` and re-emits on `sky://security/incident` for future listeners..
pub async fn SecurityIncident(Host:&dyn VineHost, Parameter:&Value) {
	Host.EmitToRenderer("sky://security/incident", Parameter.clone());

	dev_log!(
		"grpc",
		"warn: [Security] incident type={} severity={} ext={}",
		Parameter.get("type").and_then(Value::as_str).unwrap_or("?"),
		Parameter.get("severity").and_then(Value::as_str).unwrap_or("?"),
		Parameter.get("extensionId").and_then(Value::as_str).unwrap_or("?")
	);
}
