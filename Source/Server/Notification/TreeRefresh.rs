//! Cocoon `tree.refresh` notification - extension's
//! `TreeDataProvider.onDidChangeTreeData` emitter fired. Relayed to Sky
//! as `sky://tree-view/refresh`; Sky resolves the view via
//! `TreeViewByViewId(viewId)` and calls `ITreeView.refresh()`.

use serde_json::Value;

use crate::{Host::VineHost, Server::Notification::Support::RelayToSky};

pub async fn TreeRefresh(Host:&dyn VineHost, Parameter:&Value) {
	RelayToSky::Fn(Host, "sky://tree-view/refresh", Parameter, "grpc", "[Tree] refresh");
}
