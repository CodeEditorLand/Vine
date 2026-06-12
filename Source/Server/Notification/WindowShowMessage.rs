//! Extension Host `window.showMessage` notification - fire-and-forget toast.
//! Distinct from `Window.ShowMessage` (capitalised, round-trip
//! request); this is the notification form that does not wait for a
//! button selection.

use serde_json::Value;

use crate::{Host::VineHost, dev_log};

/// Handles : `window.showMessage` notification: fire-and-forget toast. Distinct from `Window.ShowMessage` (capitalised, round-trip request); this is the notification form that does not wait for a button selection..
pub async fn WindowShowMessage(Host:&dyn VineHost, Parameter:&Value) {
	dev_log!(
		"grpc",
		"[WindowShowMessage] message={:?}",
		Parameter.get("message").and_then(Value::as_str).unwrap_or("")
	);

	Host.EmitToRenderer("sky://notification/show", Parameter.clone());
}
