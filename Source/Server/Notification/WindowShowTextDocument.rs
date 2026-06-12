//! Extension Host `window.showTextDocument` notification - extension called
//! `vscode.window.showTextDocument(uri, options)`. Forwarded on
//! `sky://window/showTextDocument` for the workbench to focus.

use serde_json::Value;

use crate::Host::VineHost;

/// Handles : `window.showTextDocument` notification: extension called `vscode.window.showTextDocument(uri, options)`. Forwarded on `sky://window/showTextDocument` for the workbench to focus..
pub async fn WindowShowTextDocument(Host:&dyn VineHost, Parameter:&Value) {
	Host.EmitToRenderer("sky://window/showTextDocument", Parameter.clone());
}
