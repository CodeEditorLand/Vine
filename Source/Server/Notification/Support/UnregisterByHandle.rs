//! Shared helper for provider-unregistration notification atoms.
//!
//! Every `unregister_*_provider` wire method does the same three steps:
//! read the `handle` u32 from the parameter, call
//! `VineHost::UnregisterProvider`, and emit a tagged dev-log line. This
//! function centralises that triple so each atom shrinks to a single
//! delegation call.
//!
//! `TypeName` is the provider kind string used in the log line
//! (e.g. `"authentication"`, `"debug_adapter"`, `"task"`).

use serde_json::Value;

use crate::{Host::VineHost, dev_log};

/// Centralised provider-unregistration handler shared by all
/// `unregister_*_provider` atoms.
///
/// Reads the `handle` field from `Parameter`, calls
/// `Host.UnregisterProvider`, and logs the event.
///
/// # Parameters
///
/// * `Host` - the embedder host interface.
/// * `Parameter` - JSON payload containing a `handle` field.
/// * `TypeName` - provider kind label used in log output (e.g. `"task"`).
///
/// # Behaviour
///
/// When `handle` is zero or missing, the call is silently skipped with a
/// dev-log warning. Otherwise `Host.UnregisterProvider` is invoked with
/// the parsed handle.
pub fn UnregisterByHandle(Host:&dyn VineHost, Parameter:&Value, TypeName:&str) {
	let Handle = Parameter.get("handle").and_then(Value::as_u64).unwrap_or(0) as u32;

	if Handle == 0 {
		dev_log!("provider-register", "[ProviderUnregister] {} skip: missing handle", TypeName);

		return;
	}

	Host.UnregisterProvider(Handle);

	dev_log!("provider-register", "[ProviderUnregister] {} handle={}", TypeName, Handle);
}
