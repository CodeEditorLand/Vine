//! Extension Host `outputChannel.append` notification. Twin of `output.append`;
//! see `OutputCreate.rs` for the duplicate-wire rationale.
//!
//! Tries the per-channel coalescer first; falls back to legacy
//! per-append emit when `OutputCoalesce=0` is set or when the payload
//! has no `value` field. Channel-aware dev_log tagging routes Git/SCM
//! to `grpc`, everything else to `output-verbose`.

use serde_json::Value;

use crate::{Host::VineHost, Server::Notification::OutputChannelCoalesce, dev_log};

/// Handles : `outputChannel.append` Twin of `output.append`; see
/// `OutputCreate.rs` for the duplicate-wire rationale.  Tries the per-channel
/// coalescer first; falls back to legacy per-append emit when
/// `OutputCoalesce=0` is set or when the payload has no `value` field.
/// Channel-aware dev_log tagging routes Git/SCM to `grpc`, everything else to
/// `output-verbose`..
pub async fn OutputChannelAppend(Host:&dyn VineHost, Parameter:&Value) {
	let ChannelName = Parameter
		.get("channel")
		.or_else(|| Parameter.get("name"))
		.and_then(Value::as_str)
		.unwrap_or("?");

	let TextValue = Parameter.get("value").and_then(Value::as_str);

	let CoalesceEnqueued = match TextValue {
		Some(Text) => {
			OutputChannelCoalesce::TryEnqueue(Host.RendererEmitter(), ChannelName.to_string(), Text.to_string())
		},

		None => false,
	};

	if CoalesceEnqueued {
		return;
	}

	Host.EmitToRenderer("sky://output/append", Parameter.clone());

	// Char-aware truncation. Slicing at `&S[..200]` panics when byte
	// 200 lands inside a multi-byte UTF-8 codepoint (vscode.git's
	// progress messages contain `•` which is 3 bytes); walk char
	// boundaries instead.
	let TruncatedValue = Parameter
		.get("value")
		.and_then(Value::as_str)
		.map(|S| {
			if S.len() > 200 {
				let CutAt = S
					.char_indices()
					.map(|(Index, _)| Index)
					.take_while(|Index| *Index <= 200)
					.last()
					.unwrap_or(0);

				format!("{}…", &S[..CutAt])
			} else {
				S.to_string()
			}
		})
		.unwrap_or_else(|| "<no-value>".to_string());

	if ChannelName.eq_ignore_ascii_case("git")
		|| ChannelName.eq_ignore_ascii_case("source control")
		|| ChannelName.eq_ignore_ascii_case("scm")
	{
		dev_log!(
			"grpc",
			"[OutputChannel:{}] {}",
			ChannelName,
			TruncatedValue.trim_end_matches('\n')
		);
	} else {
		dev_log!("output-verbose", "[OutputChannel] append channel={}", ChannelName);
	}
}
