//! Cocoon → `webview.setTitle` / `webview.setIconPath` / `webview.setHtml` /
//! `webview.postMessage` / `webview.updateView` / `webview.viewState` /
//! `webview.dispose` notifications.
//!
//! Wire-shape canonicalisation: SkyBridge listeners read named keys
//! (`Payload.viewId`, `Payload.html`, `Payload.message`). Cocoon's legacy
//! positional `[Handle, Value]` arrays are projected to named aliases here so
//! every producer shape lands on the same Sky channel. Mirrors the reshape
//! `Track/Effect/CreateEffectForRequest/Webview.rs` applies on the request
//! path.
//!
//! Suffix mapping: `setHtml` → `set-html` (kebab) to match the typed-RPC
//! channel; other suffixes pass through camelCase.

use serde_json::{Map, Value, json};

use crate::{Host::VineHost, dev_log};

pub async fn WebviewLifecycle(Host:&dyn VineHost, MethodName:&str, Parameter:&Value) {
	let RawSuffix = &MethodName["webview.".len()..];

	let Suffix = match RawSuffix {
		"setHtml" => "set-html",

		"postMessage" => "postMessage",

		Other => Other,
	};

	let EventName = format!("sky://webview/{}", Suffix);

	// Canonicalise positional [Handle, Value] arrays to named-key objects.
	let CanonicalPayload:Value = if Parameter.is_object() {
		Parameter.clone()
	} else if let Some(First) = Parameter.get(0) {
		if First.is_object() {
			First.clone()
		} else {
			let mut Object = Map::new();

			Object.insert("method".to_string(), Value::String(MethodName.to_string()));

			Object.insert("handle".to_string(), First.clone());

			Object.insert("args".to_string(), Parameter.clone());

			if let Some(Second) = Parameter.get(1) {
				let Alias = match MethodName {
					"webview.setHtml" => "html",

					"webview.postMessage" => "message",

					"webview.registerView" | "webview.unregisterView" => "viewId",

					"webview.registerCustomEditor" | "webview.unregisterCustomEditor" | "webview.create" => "viewType",

					_ => "value",
				};

				Object.insert(Alias.to_string(), Second.clone());

				if MethodName == "webview.create" {
					if let Some(Third) = Parameter.get(2) {
						Object.insert("title".to_string(), Third.clone());
					}
				}
			}

			Value::Object(Object)
		}
	} else {
		json!({ "method": MethodName, "handle": Parameter.clone() })
	};

	dev_log!(
		"grpc",
		"[WebviewLifecycle] emit {} handle={:?}",
		EventName,
		CanonicalPayload.get("handle")
	);

	Host.EmitToRenderer(&EventName, CanonicalPayload);
}
