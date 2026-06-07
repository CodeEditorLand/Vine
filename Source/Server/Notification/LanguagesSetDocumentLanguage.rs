//! Cocoon → `languages.setDocumentLanguage` notification.
//! Emitted when an extension calls `vscode.languages.setTextDocumentLanguage`.
//! Forwarded verbatim to Sky on `sky://languages/setDocumentLanguage` so
//! Monaco swaps the language mode on the matching editor.

use serde_json::Value;

use crate::{Host::VineHost, Server::Notification::Support::RelayToSky};

pub async fn LanguagesSetDocumentLanguage(Host:&dyn VineHost, Parameter:&Value) {
	RelayToSky::Fn(Host, "sky://languages/setDocumentLanguage", Parameter, "grpc", "");
}
