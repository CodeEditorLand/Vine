//! Cocoon → `window.createTerminal` notification.
//! Spawns a PTY via `VineHost::CreateTerminal`, then emits
//! `sky://terminal/create` with the provider-minted `{ id, pid, name }`.

use serde_json::{Value, json};

use crate::{Host::VineHost, dev_log};

pub async fn WindowCreateTerminal(Host:&dyn VineHost, Parameter:&Value) {
	let Handle = Parameter.get("handle").and_then(Value::as_str).unwrap_or("").to_string();

	let Options = Parameter.get("options").cloned().unwrap_or_else(|| Parameter.clone());

	let Name = Options.get("name").and_then(Value::as_str).unwrap_or("Terminal").to_string();

	dev_log!("grpc", "[WindowCreateTerminal] handle={} name={}", Handle, Name);

	let Some(Result) = Host.CreateTerminal(&Options).await else {
		dev_log!("grpc", "warn: [WindowCreateTerminal] CreateTerminal returned None");

		return;
	};

	let Id = Result.get("id").and_then(Value::as_u64).unwrap_or(0);

	let Pid = Result.get("pid").and_then(Value::as_u64).unwrap_or(0);

	let ResultName = Result.get("name").and_then(Value::as_str).unwrap_or(&Name).to_string();

	dev_log!("grpc", "[WindowCreateTerminal] created id={} pid={}", Id, Pid);

	Host.EmitToRenderer(
		"sky://terminal/create",
		json!({
			"handle": Handle,
			"id": Id,
			"pid": Pid,
			"name": ResultName,
		}),
	);
}
