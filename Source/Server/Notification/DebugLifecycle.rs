//! Cocoon → `debug.addBreakpoints` / `debug.removeBreakpoints` /
//! `debug.consoleAppend` notifications.
//! Fans on `sky://debug/<suffix>` so the Sky-side debug view picks up
//! breakpoint changes and console output from the extension's
//! `vscode.debug.*` surface.
//! For breakpoint mutations specifically, also fans back to Cocoon so
//! `vscode.debug.onDidChangeBreakpoints` subscribers in OTHER extensions
//! observe the change. Without this round-trip, only the extension that
//! called `addBreakpoints`/`removeBreakpoints` knows about its own write.

use serde_json::{Value, json};

use crate::{Host::VineHost, dev_log};

pub async fn DebugLifecycle(Host:&dyn VineHost, MethodName:&str, Parameter:&Value) {
	let EventName = format!("sky://debug/{}", &MethodName["debug.".len()..]);

	Host.EmitToRenderer(&EventName, Parameter.clone());

	if MethodName == "debug.addBreakpoints" || MethodName == "debug.removeBreakpoints" {
		let Added:Vec<Value> = if MethodName == "debug.addBreakpoints" {
			Parameter
				.get("breakpoints")
				.and_then(Value::as_array)
				.cloned()
				.unwrap_or_default()
		} else {
			Vec::new()
		};

		let Removed:Vec<Value> = if MethodName == "debug.removeBreakpoints" {
			Parameter
				.get("breakpoints")
				.and_then(Value::as_array)
				.cloned()
				.unwrap_or_default()
		} else {
			Vec::new()
		};

		dev_log!(
			"grpc",
			"[DebugLifecycle] fan-back {} added={} removed={}",
			MethodName,
			Added.len(),
			Removed.len()
		);

		Host.IPCProvider().SendNotification(
			"cocoon-main",
			"$onDidChangeBreakpoints",
			json!({
				"added": Added,
				"removed": Removed,
				"changed": [],
			}),
		);
	}
}
