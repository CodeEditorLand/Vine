//! Cocoon `window.createTextEditorDecorationType` /
//! `window.disposeTextEditorDecorationType` notifications. Forwards the
//! payload on `sky://decoration/<suffix>` as a batch; Sky demultiplexes
//! back to per-decoration `cel:decoration:*` CustomEvents.
//!
//! ~337 create + 317 dispose calls per session. Channel-drain pattern:
//! a single long-lived flusher wakes on first item, drains, sleeps one
//! frame (16 ms), drains stragglers, then emits one batched event per
//! channel name. Zero spawns per call.

use std::sync::{Arc, OnceLock};

use serde_json::{Value, json};
use tokio::sync::mpsc::{UnboundedSender, unbounded_channel};

use crate::{
	Host::{RendererEmitter, VineHost},
	dev_log,
};

struct DecorationItem {
	Emitter:Arc<dyn RendererEmitter>,

	Channel:String,

	Payload:Value,
}

struct DecorationChannel {
	Sender:UnboundedSender<DecorationItem>,
}

static DECO_CH:OnceLock<DecorationChannel> = OnceLock::new();

fn GetOrInitChannel() -> &'static DecorationChannel {
	DECO_CH.get_or_init(|| {
		let (Tx, mut Rx) = unbounded_channel::<DecorationItem>();

		tokio::spawn(async move {
			let mut Buf:Vec<DecorationItem> = Vec::with_capacity(64);

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

				let Emitter = Buf[0].Emitter.clone();

				let mut ByChannel:std::collections::HashMap<String, Vec<Value>> = std::collections::HashMap::new();

				for Item in Buf.drain(..) {
					ByChannel.entry(Item.Channel).or_default().push(Item.Payload);
				}

				for (ChannelName, Payloads) in ByChannel {
					let Count = Payloads.len();

					Emitter.Emit(&ChannelName, json!({ "batch": Payloads }));

					dev_log!("sky-emit", "[SkyEmit] ok channel={} batch={}", ChannelName, Count);
				}
			}
		});

		DecorationChannel { Sender:Tx }
	})
}

pub async fn DecorationTypeLifecycle(Host:&dyn VineHost, MethodName:&str, Parameter:&Value) {
	let EventName = format!("sky://decoration/{}", &MethodName["window.".len()..]);

	let Ch = GetOrInitChannel();

	let _ = Ch.Sender.send(DecorationItem {
		Emitter:Host.RendererEmitter(),
		Channel:EventName,
		Payload:Parameter.clone(),
	});
}
