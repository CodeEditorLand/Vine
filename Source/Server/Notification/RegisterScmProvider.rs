//! Extension Host → `register_scm_provider` notification.
//!
//! Three side effects, all best-effort and independent:
//! 1. `VineHost::RegisterScmInRegistry` records the handle in the embedder's
//!    `ProviderRegistration` table so future handle-keyed dispatches resolve.
//! 2. `VineHost::CreateSourceControl` mutates the embedder's SCM marker state
//!    and emits `SkyEvent::SCMProviderAdded` - the canonical path the SCM view
//!    uses.
//! 3. `Host.EmitToRenderer("sky://scm/register", ...)` covers renderer code
//!    that listens for the legacy simpler event shape.
//!
//! Handle disambiguation: Cocoon's `ScmNamespace.ts` allocates a
//! process-local sequential handle and includes it on the wire. Subsequent
//! `register_scm_resource_group`, `update_scm_group`, and
//! `unregister_scm_provider` notifications reference the SAME sequential
//! handle. Falling back to a DJB hash of `ScmId` is only allowed when
//! Extension Host omits the field (legacy callers).

use serde_json::{Value, json};

use crate::{Host::VineHost, dev_log};

/// Rebuilds a full URL string from a VS Code `UriComponents` JSON object
/// (`{ scheme, authority, path, query, fragment }`). Returns `None` when
/// the scheme field is absent or empty.
fn BuildUrlFromComponents(O:&serde_json::Map<String, Value>) -> Option<String> {
	let Scheme = O.get("scheme").and_then(Value::as_str)?;

	if Scheme.is_empty() {
		return None;
	}

	let Authority = O.get("authority").and_then(Value::as_str).unwrap_or("");

	let Path = O.get("path").and_then(Value::as_str).unwrap_or("");

	let Query = O.get("query").and_then(Value::as_str).unwrap_or("");

	let Fragment = O.get("fragment").and_then(Value::as_str).unwrap_or("");

	let mut Url = format!("{}://{}{}", Scheme, Authority, Path);

	if !Query.is_empty() {
		Url.push('?');

		Url.push_str(Query);
	}

	if !Fragment.is_empty() {
		Url.push('#');

		Url.push_str(Fragment);
	}

	Some(Url)
}

/// Handles : → `register_scm_provider`  Three side effects, all best-effort and
/// independent: 1. `VineHost::RegisterScmInRegistry` records the handle in the
/// embedder's `ProviderRegistration` table so future handle-keyed dispatches
/// resolve. 2. `VineHost::CreateSourceControl` mutates the embedder's SCM
/// marker state and emits `SkyEvent::SCMProviderAdded`: the canonical path the
/// SCM view uses. 3. `Host.EmitToRenderer("sky://scm/register", ...)` covers
/// renderer code that listens for the legacy simpler event shape.  Handle
/// disambiguation: Cocoon's `ScmNamespace.ts` allocates a process-local
/// sequential handle and includes it on the wire. Subsequent
/// `register_scm_resource_group`, `update_scm_group`, and
/// `unregister_scm_provider` notifications reference the SAME sequential
/// handle. Falling back to a DJB hash of `ScmId` is only allowed when omits the
/// field (legacy callers)..
pub async fn RegisterScmProvider(Host:&dyn VineHost, Parameter:&Value) {
	// Wire-shape: camelCase first (current Cocoon), snake_case fallback for
	// transitional compatibility when Mountain is ahead of Cocoon.
	let ScmId = Parameter
		.get("id")
		.or_else(|| Parameter.get("scmId"))
		.or_else(|| Parameter.get("scm_id"))
		.and_then(Value::as_str)
		.unwrap_or("")
		.to_string();

	let Label = Parameter.get("label").and_then(Value::as_str).unwrap_or(&ScmId).to_string();

	let ExtensionId = Parameter
		.get("extensionId")
		.or_else(|| Parameter.get("extension_id"))
		.and_then(Value::as_str)
		.unwrap_or("")
		.to_string();

	let RootUri = Parameter
		.get("rootUri")
		.or_else(|| Parameter.get("root_uri"))
		.cloned()
		.unwrap_or(Value::Null);

	if ScmId.is_empty() {
		dev_log!("provider-register", "[ProviderRegister] scm skip: missing scm_id");

		return;
	}

	// Preserve Cocoon's sequential handle verbatim so all subsequent
	// resource-group / update notifications key under the same value.
	let Handle = Parameter
		.get("handle")
		.or_else(|| Parameter.get("scmHandle"))
		.or_else(|| Parameter.get("scm_handle"))
		.and_then(Value::as_u64)
		.map(|H| H as u32)
		.unwrap_or_else(|| {
			ScmId
				.as_bytes()
				.iter()
				.fold(0u32, |Acc, B| Acc.wrapping_mul(31).wrapping_add(*B as u32))
		});

	// Side effect 1 - register in provider table.
	Host.RegisterScmInRegistry(Handle, &ScmId, &Label, &ExtensionId);

	// Reconstruct a full URL from a VS Code UriComponents object if needed.
	let RootUriString = match &RootUri {
		Value::String(S) => S.clone(),

		Value::Object(O) => {
			BuildUrlFromComponents(O)
				.or_else(|| O.get("external").and_then(Value::as_str).map(str::to_string))
				.or_else(|| {
					O.get("path")
						.and_then(Value::as_str)
						.filter(|P| P.starts_with('/'))
						.map(|P| format!("file://{}", P))
				})
				.unwrap_or_else(|| "file:///".to_string())
		},

		_ => "file:///".to_string(),
	};

	// Field names must match `SourceControlCreateDTO`'s camelCase wire shape.
	// Including `handle` here makes `MountainEnvironment::CreateSourceControl`
	// key its marker maps under the SAME handle that
	// `register_scm_resource_group` and `update_scm_group` reference.
	let CreateData = json!({
		"handle": Handle,
		"id": &ScmId,
		"label": &Label,
		"rootUri": RootUriString,
	});

	// Side effect 2 - SCM provider state + SkyEvent::SCMProviderAdded.
	Host.CreateSourceControl(CreateData).await;

	// Side effect 3 - legacy renderer channel.
	Host.EmitToRenderer(
		"sky://scm/register",
		json!({
			"scmId": &ScmId,
			"label": &Label,
			"rootUri": &RootUriString,
			"extensionId": &ExtensionId,
			"handle": Handle,
		}),
	);

	dev_log!(
		"grpc",
		"[Scm] register provider scmId={} label={} ext={} handle={}",
		ScmId,
		Label,
		ExtensionId,
		Handle
	);
}
