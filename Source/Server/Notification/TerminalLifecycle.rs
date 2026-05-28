//! Cocoon → `terminal.sendText` / `terminal.show` / `terminal.hide` /
//! `terminal.dispose` notifications.
//!
//! Two concerns per invocation:
//! 1. Relay `sky://terminal/<suffix>` to the renderer so the xterm panel can
//!    show/hide / print text / remove itself.
//! 2. Drive the underlying PTY via `VineHost::SpawnSendTextToTerminal` or
//!    `VineHost::SpawnDisposeTerminal` so the OS process sees the text /
//!    receives SIGHUP on dispose. (No-op for embedders without terminal
//!    support.)

use serde_json::Value;

use crate::{Host::VineHost, dev_log};

pub async fn TerminalLifecycle(Host:&dyn VineHost, MethodName:&str, Parameter:&Value) {
	let EventName = format!("sky://terminal/{}", &MethodName["terminal.".len()..]);

	Host.EmitToRenderer(&EventName, Parameter.clone());

	// Terminal handles from Cocoon arrive as `"terminal:N"`; strip the
	// prefix to recover the numeric id the provider expects.
	let HandleNumeric = Parameter
		.get("handle")
		.and_then(|H| H.as_str())
		.and_then(|S| S.trim_start_matches("terminal:").parse::<u64>().ok());

	if let Some(TerminalId) = HandleNumeric {
		match MethodName {
			"terminal.sendText" => {
				let Text = Parameter.get("text").and_then(|T| T.as_str()).unwrap_or("").to_string();

				dev_log!("grpc", "[TerminalLifecycle] sendText id={} len={}", TerminalId, Text.len());

				Host.SpawnSendTextToTerminal(TerminalId, Text);
			},

			"terminal.dispose" => {
				dev_log!("grpc", "[TerminalLifecycle] dispose id={}", TerminalId);

				Host.SpawnDisposeTerminal(TerminalId);
			},

			_ => {},
		}
	}
}
