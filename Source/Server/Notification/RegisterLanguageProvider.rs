//! Extension Host → `register_*` / `register_*_provider` notification dispatcher.
//!
//! Handles all 46+ language-feature provider registration wire methods that
//! Cocoon's vscode-API shim emits. Strips the `register_` prefix and optional
//! `_provider` suffix from the method name to derive a normalised `TypeName`,
//! logs the registration, then delegates to
//! `VineHost::RegisterLanguageProvider` which maps `TypeName` to the embedder's
//! internal provider-type enum and inserts a `ProviderRegistrationDTO`. Returns
//! `true` if the type was recognised.
//!
//! Wire-method naming uses snake_case with two trailing shapes:
//! - plain verbs:           `register_rename`, `register_debug_adapter`
//! - `_provider` suffix:    `register_hover_provider`,
//!   `register_code_lens_provider`

use serde_json::Value;

use crate::{Host::VineHost, dev_log};

/// Handles : → `register_*` / `register_*_provider` notification dispatcher.  Handles all 46+ language-feature provider registration wire methods that Cocoon's vscode-API shim emits. Strips the `register_` prefix and optional `_provider` suffix from the method name to derive a normalised `TypeName`, logs the registration, then delegates to `VineHost::RegisterLanguageProvider` which maps `TypeName` to the embedder's internal provider-type enum and inserts a `ProviderRegistrationDTO`. Returns `true` if the type was recognised.  Wire-method naming uses snake_case with two trailing shapes:: plain verbs:           `register_rename`, `register_debug_adapter`: `_provider` suffix:    `register_hover_provider`, `register_code_lens_provider`.
pub async fn RegisterLanguageProvider(Host:&dyn VineHost, MethodName:&str, Parameter:&Value) -> bool {
	let Handle = Parameter.get("handle").and_then(|H| H.as_u64()).unwrap_or(0) as u32;

	let Selector = Parameter
		.get("languageSelector")
		.or_else(|| Parameter.get("language_selector"))
		.and_then(Value::as_str)
		.unwrap_or("*");

	let TypeName = MethodName
		.strip_prefix("register_")
		.map(|S| S.strip_suffix("_provider").unwrap_or(S))
		.unwrap_or("");

	dev_log!(
		"provider-register",
		"[ProviderRegister] method={} type={} handle={} lang={}",
		MethodName,
		TypeName,
		Handle,
		Selector
	);

	Host.RegisterLanguageProvider(Handle, TypeName, Parameter)
}
