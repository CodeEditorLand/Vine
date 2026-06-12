//! Extension Host `workspace.applyEdit` notification - extension called
//! `vscode.workspace.applyEdit(edit)` with a multi-file
//! `WorkspaceEdit`. Sky's BulkEditService applies the edits against
//! open models.

use serde_json::Value;

use crate::Host::VineHost;

/// Handles : `workspace.applyEdit` notification: extension called
/// `vscode.workspace.applyEdit(edit)` with a multi-file `WorkspaceEdit`. Sky's
/// BulkEditService applies the edits against open models..
pub async fn WorkspaceApplyEdit(Host:&dyn VineHost, Parameter:&Value) {
	Host.EmitToRenderer("sky://workspace/applyEdit", Parameter.clone());
}
