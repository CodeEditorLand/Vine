//! Cocoon → `set_language_configuration` notification.
//! Carries brackets / indent rules / word-pattern / comments.
//! Forwarded on `sky://language/configure`; Monaco's config side reads
//! the payload and calls `monaco.languages.setLanguageConfiguration(...)`.

use serde_json::Value;

use crate::{Host::VineHost, dev_log};

pub async fn SetLanguageConfiguration(Host:&dyn VineHost, Parameter:&Value) {
	Host.EmitToRenderer("sky://language/configure", Parameter.clone());

	dev_log!(
		"grpc",
		"[Language] configure id={}",
		Parameter.get("languageId").and_then(Value::as_str).unwrap_or("?")
	);
}
