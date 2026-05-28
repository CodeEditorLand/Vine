//! Cocoon → `registerCommand` notification.
//! Two side effects per call:
//! 1. Insert a `Proxied` handler into the embedder's command dispatch registry
//!    via `VineHost::RegisterCommandInRegistry` (synchronous; allows
//!    `commands.executeCommand` to route back to Cocoon immediately).
//! 2. Push the command descriptor into a channel-drain coalescer that emits one
//!    `sky://command/register` batch per 16 ms frame, avoiding 1000+ individual
//!    renderer events during extension boot.
//!
//! The coalescer holds a captured `Arc<dyn RendererEmitter>` so the drain
//! task never borrows the full host across await points.

use std::sync::{Arc, OnceLock};

use serde_json::{Value, json};
use tokio::sync::mpsc::{UnboundedSender, unbounded_channel};

use crate::{
	Host::{RendererEmitter, VineHost},
	dev_log,
};

struct CommandBatchChannel {
	Sender:UnboundedSender<Value>,
}

static CMD_CHANNEL:OnceLock<CommandBatchChannel> = OnceLock::new();

fn GetOrInitChannel(Emitter:Arc<dyn RendererEmitter>) -> &'static CommandBatchChannel {
	CMD_CHANNEL.get_or_init(|| {
		let (Tx, mut Rx) = unbounded_channel::<Value>();

		tokio::spawn(async move {
			let mut Buf:Vec<Value> = Vec::with_capacity(128);

			loop {
				// Block until at least one item arrives.
				match Rx.recv().await {
					None => break,
					Some(V) => Buf.push(V),
				}

				// Drain everything already queued without blocking.
				Rx.recv_many(&mut Buf, 4096).await;

				// One frame - let stragglers accumulate.
				tokio::time::sleep(std::time::Duration::from_millis(16)).await;

				// Drain again after the frame window.
				Rx.recv_many(&mut Buf, 4096).await;

				if Buf.is_empty() {
					continue;
				}

				let Count = Buf.len();
				let Commands:Vec<Value> = Buf.drain(..).collect();

				Emitter.Emit("sky://command/register", json!({ "commands": Commands }));
				dev_log!("commands", "[RegisterCommand] batch={}", Count);
			}
		});

		CommandBatchChannel { Sender:Tx }
	})
}

pub async fn RegisterCommand(Host:&dyn VineHost, Parameter:&Value) {
	let CommandId = Parameter.get("commandId").and_then(Value::as_str).unwrap_or("");

	dev_log!("command-register", "[RegisterCommand] id={}", CommandId);

	if CommandId.is_empty() {
		return;
	}

	let Kind = Parameter.get("kind").and_then(Value::as_str).unwrap_or("command").to_string();

	// Synchronous registry insert so executeCommand works immediately.
	Host.RegisterCommandInRegistry(CommandId, "cocoon-main");

	// Queue for batched Sky emit.
	let Ch = GetOrInitChannel(Host.RendererEmitter());
	let _ = Ch.Sender.send(json!({ "id": CommandId, "commandId": CommandId, "kind": Kind }));
}
