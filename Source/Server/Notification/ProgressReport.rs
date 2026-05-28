//! Cocoon `progress.report` notification.
//!
//! The git extension alone fires 6000+ of these per session. Items are
//! pushed into an `mpsc::unbounded_channel`; a single long-lived flusher
//! task wakes on the first item, drains everything queued, sleeps one
//! frame (16 ms), drains again, then emits one merged
//! `sky://notification/progress-update` per progress handle. Per-handle
//! merge: latest non-empty `message`, summed `increment`. Zero spawns
//! per call; one renderer event per handle per frame.

use std::sync::{Arc, OnceLock};

use serde_json::{Value, json};
use tokio::sync::mpsc::{UnboundedSender, unbounded_channel};

use crate::{
	Host::{RendererEmitter, VineHost},
	dev_log,
};

struct ProgressItem {
	Emitter:Arc<dyn RendererEmitter>,

	ProgressHandle:String,

	Message:String,

	Increment:f64,
}

struct ProgressChannel {
	Sender:UnboundedSender<ProgressItem>,
}

static PROGRESS_CH:OnceLock<ProgressChannel> = OnceLock::new();

fn GetOrInitChannel() -> &'static ProgressChannel {
	PROGRESS_CH.get_or_init(|| {
		let (Tx, mut Rx) = unbounded_channel::<ProgressItem>();

		tokio::spawn(async move {
			let mut Buf:Vec<ProgressItem> = Vec::with_capacity(64);

			loop {
				match Rx.recv().await {
					None => break,
					Some(Item) => Buf.push(Item),
				}

				Rx.recv_many(&mut Buf, 4096).await;

				tokio::time::sleep(std::time::Duration::from_millis(16)).await;

				Rx.recv_many(&mut Buf, 4096).await;

				if Buf.is_empty() {
					continue;
				}

				let mut ByHandle:std::collections::HashMap<String, (Arc<dyn RendererEmitter>, String, f64)> =
					std::collections::HashMap::new();

				for Item in Buf.drain(..) {
					let Entry = ByHandle
						.entry(Item.ProgressHandle.clone())
						.or_insert_with(|| (Item.Emitter.clone(), String::new(), 0.0));

					if !Item.Message.is_empty() {
						Entry.1 = Item.Message;
					}

					Entry.2 += Item.Increment;
				}

				for (ProgressHandleId, (Emitter, Message, Increment)) in ByHandle {
					Emitter.Emit(
						"sky://notification/progress-update",
						json!({
							"id": ProgressHandleId,
							"message": Message,
							"increment": Increment,
						}),
					);

					dev_log!(
						"sky-emit",
						"[ProgressReport] emit handle={} increment={}",
						ProgressHandleId,
						Increment
					);
				}
			}
		});

		ProgressChannel { Sender:Tx }
	})
}

pub async fn ProgressReport(Host:&dyn VineHost, Parameter:&Value) {
	let ProgressHandle = Parameter.get("handle").and_then(Value::as_str).unwrap_or("").to_string();

	let Message = Parameter.get("message").and_then(Value::as_str).unwrap_or("").to_string();

	let Increment = Parameter.get("increment").and_then(Value::as_f64).unwrap_or(0.0);

	let Ch = GetOrInitChannel();

	let _ = Ch
		.Sender
		.send(ProgressItem { Emitter:Host.RendererEmitter(), ProgressHandle, Message, Increment });
}
