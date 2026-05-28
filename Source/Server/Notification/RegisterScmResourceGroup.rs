//! Cocoon → `register_scm_resource_group` notification.
//!
//! Pairs with `RegisterScmProvider`. An SCM provider creates one or more
//! resource groups (Git's "Changes", "Staged Changes", "Merge Changes").
//! Two side effects:
//! 1. `VineHost::UpdateSourceControlGroup` seeds the group with an empty
//!    `resourceStates` list so subsequent `update_scm_group` calls find it.
//! 2. `sky://scm/registerGroup` renderer event materialises the group header
//!    row in the SCM viewlet.

use serde_json::{Value, json};

use crate::{Host::VineHost, dev_log};

pub async fn RegisterScmResourceGroup(Host:&dyn VineHost, Parameter:&Value) {
	let ScmHandle = Parameter
		.get("scmHandle")
		.or_else(|| Parameter.get("scm_handle"))
		.and_then(Value::as_u64)
		.unwrap_or(0) as u32;

	let GroupHandleStr = Parameter
		.get("groupHandle")
		.or_else(|| Parameter.get("group_handle"))
		.and_then(Value::as_str)
		.unwrap_or("")
		.to_string();

	let GroupId = Parameter
		.get("groupId")
		.or_else(|| Parameter.get("group_id"))
		.and_then(Value::as_str)
		.unwrap_or("")
		.to_string();

	let Label = Parameter.get("label").and_then(Value::as_str).unwrap_or(&GroupId).to_string();

	if GroupId.is_empty() {
		dev_log!("provider-register", "[ProviderRegister] scm-group skip: missing group_id");

		return;
	}

	// Field names match `SourceControlGroupUpdateDTO`'s camelCase wire shape.
	let GroupData = json!({
		"providerHandle": ScmHandle,
		"groupId": &GroupId,
		"label": &Label,
		"resourceStates": [],
	});

	// Side effect 1 - seed group state (upsert on first call).
	Host.UpdateSourceControlGroup(ScmHandle, GroupData).await;

	// Side effect 2 - renderer materialises the group header row.
	Host.EmitToRenderer(
		"sky://scm/registerGroup",
		json!({
			"scmHandle": ScmHandle,
			"groupHandle": &GroupHandleStr,
			"groupId": &GroupId,
			"label": &Label,
		}),
	);

	dev_log!(
		"grpc",
		"[Scm] register group scm_handle={} group_id={} label={}",
		ScmHandle,
		GroupId,
		Label
	);
}
