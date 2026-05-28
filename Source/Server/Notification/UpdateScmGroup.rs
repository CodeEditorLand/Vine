//! Cocoon → `update_scm_group` notification.
//! Fire-and-forget resource-state update for an SCM group. Two side effects:
//! 1. Persist the snapshot via `VineHost::UpdateScmGroupMarkers` so the
//!    boot-time replay (`sky:replay-events`) can re-emit it for late
//!    subscribers (without this, only live arrivals are visible).
//! 2. `sky://scm/updateGroup` renderer event so the SCM viewlet updates without
//!    waiting for a request/response round-trip.
//!
//! `group_handle` is `"<scm_handle>/<group_id>"` per `ScmNamespace.ts:77`.
//! Splits on `/` to expose a flat `groupId` for the renderer payload;
//! legacy `provider_id`/`group_id` flat keys are probed as fallback.

use serde_json::{Value, json};

use crate::{Host::VineHost, dev_log};

pub async fn UpdateScmGroup(Host:&dyn VineHost, Parameter:&Value) {
	let ScmHandle = Parameter
		.get("scmHandle")
		.or_else(|| Parameter.get("scm_handle"))
		.and_then(Value::as_u64)
		.map(|H| H as u32);

	let GroupHandle = Parameter
		.get("groupHandle")
		.or_else(|| Parameter.get("group_handle"))
		.and_then(Value::as_str)
		.unwrap_or("")
		.to_string();

	let LegacyProviderId = Parameter
		.get("providerId")
		.or_else(|| Parameter.get("provider_id"))
		.and_then(Value::as_str)
		.unwrap_or("")
		.to_string();

	let LegacyGroupId = Parameter
		.get("groupId")
		.or_else(|| Parameter.get("group_id"))
		.and_then(Value::as_str)
		.unwrap_or("")
		.to_string();

	let ResourceStates = Parameter
		.get("resourceStates")
		.or_else(|| Parameter.get("resource_states"))
		.cloned()
		.unwrap_or_else(|| Value::Array(Vec::new()));

	// Split `"<scm_handle>/<group_id>"` into its components.
	let (HandleFromString, GroupIdFromHandle) = match GroupHandle.split_once('/') {
		Some((H, G)) => (H.parse::<u32>().ok(), G.to_string()),
		None => (None, String::new()),
	};

	let ResolvedScmHandle = ScmHandle.or(HandleFromString);

	let ResolvedGroupId = if !GroupIdFromHandle.is_empty() {
		GroupIdFromHandle
	} else if !LegacyGroupId.is_empty() {
		LegacyGroupId
	} else {
		String::new()
	};

	if ResolvedScmHandle.is_none() && LegacyProviderId.is_empty() {
		dev_log!(
			"grpc",
			"[ScmGroup] skip: missing scm_handle / provider_id (group_handle={:?} group={:?})",
			GroupHandle,
			ResolvedGroupId
		);

		return;
	}

	if ResolvedGroupId.is_empty() {
		dev_log!(
			"grpc",
			"[ScmGroup] skip: missing group_id (scm_handle={:?} group_handle={:?})",
			ResolvedScmHandle,
			GroupHandle
		);

		return;
	}

	// Persist snapshot for boot-time replay.
	if let Some(Handle) = ResolvedScmHandle {
		Host.UpdateScmGroupMarkers(Handle, &ResolvedGroupId, &ResourceStates);
	}

	Host.EmitToRenderer(
		"sky://scm/updateGroup",
		json!({
			"scmHandle": ResolvedScmHandle,
			"providerId": &LegacyProviderId,
			"groupHandle": &GroupHandle,
			"groupId": &ResolvedGroupId,
			"resourceStates": ResourceStates,
		}),
	);

	dev_log!(
		"grpc",
		"[ScmGroup] scm_handle={:?} group={} resources={}",
		ResolvedScmHandle,
		ResolvedGroupId,
		ResourceStates.as_array().map(Vec::len).unwrap_or(0)
	);
}
