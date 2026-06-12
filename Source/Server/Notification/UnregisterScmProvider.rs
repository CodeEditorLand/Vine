//! Extension Host → `unregister_scm_provider` notification.
//! Emitted when `vscode.scm.createSourceControl(...).dispose()` fires.
//! Resolves the provider handle: uses the explicit `handle` field when
//! Extension Host sends it, otherwise recomputes the DJB-31 hash of `scmId`
//! that `RegisterScmProvider` used when it originally registered the provider
//! (necessary because Cocoon's dispose path only carries the string id).

use serde_json::{Value, json};

use crate::{Host::VineHost, dev_log};

/// Handles : → `unregister_scm_provider` Emitted when
/// `vscode.scm.createSourceControl(...).dispose()` fires. Resolves the provider
/// handle: uses the explicit `handle` field when sends it, otherwise recomputes
/// the DJB-31 hash of `scmId` that `RegisterScmProvider` used when it
/// originally registered the provider (necessary because Cocoon's dispose path
/// only carries the string id)..
pub async fn UnregisterScmProvider(Host:&dyn VineHost, Parameter:&Value) {
	let ScmId = Parameter
		.get("scmId")
		.or_else(|| Parameter.get("scm_id"))
		.and_then(Value::as_str)
		.unwrap_or("")
		.to_string();

	let DirectHandle = Parameter.get("handle").and_then(Value::as_u64).map(|H| H as u32);

	if ScmId.is_empty() && DirectHandle.is_none() {
		dev_log!("provider-register", "[ProviderUnregister] scm skip: missing handle / scmId");

		return;
	}

	let Handle = DirectHandle.unwrap_or_else(|| {
		ScmId
			.as_bytes()
			.iter()
			.fold(0u32, |Acc, B| Acc.wrapping_mul(31).wrapping_add(*B as u32))
	});

	Host.UnregisterProvider(Handle);

	Host.EmitToRenderer("sky://scm/unregister", json!({ "scmId": ScmId }));

	dev_log!(
		"provider-register",
		"[ProviderUnregister] scm scm_id={} handle={}",
		ScmId,
		Handle
	);
}
