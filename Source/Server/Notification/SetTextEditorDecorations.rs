//! Extension Host → `window.setTextEditorDecorations` notification.
//! Channel-drain batching: ~5-200 calls per extension per second during
//! scroll; one renderer event per frame (16 ms window, drain stragglers).
//! Uses `Arc<dyn RendererEmitter>` captured once from
//! `VineHost::RendererEmitter()` so the drain task never holds a reference
//! to the full host across await points.

use std::sync::{Arc, OnceLock};

use serde_json::{Value, json};
use tokio::sync::mpsc::{UnboundedSender, unbounded_channel};

use crate::{
	Host::{RendererEmitter, VineHost},
	dev_log,
};

struct DecoSetChannel {
	Sender:UnboundedSender<Value>,
}

static DECO_SET_CH:OnceLock<DecoSetChannel> = OnceLock::new();

fn GetOrInitChannel(Emitter:Arc<dyn RendererEmitter>) -> &'static DecoSetChannel {
	DECO_SET_CH.get_or_init(|| {
		let (Tx, mut Rx) = unbounded_channel::<Value>();

		tokio::spawn(async move {
			let mut Buf:Vec<Value> = Vec::with_capacity(64);

			loop {
				match Rx.recv().await {
					None => break,
					Some(V) => Buf.push(V),
				}

				// Drain everything already queued without blocking.
				Rx.recv_many(&mut Buf, 4096).await;

				// One animation frame - let stragglers accumulate.
				tokio::time::sleep(std::time::Duration::from_millis(16)).await;

				// Drain again after the frame window.
				Rx.recv_many(&mut Buf, 4096).await;

				if Buf.is_empty() {
					continue;
				}

				let Count = Buf.len();

				let Batch:Vec<Value> = Buf.drain(..).collect();

				Emitter.Emit("sky://decoration/set-ranges", json!({ "batch": Batch }));

				dev_log!("sky-emit", "[DecoSet] emitted batch={}", Count);
			}
		});

		DecoSetChannel { Sender:Tx }
	})
}

/// Handles : → `window.setTextEditorDecorations` Channel-drain batching: ~5-200
/// calls per extension per second during scroll; one renderer event per frame
/// (16 ms window, drain stragglers). Uses `Arc<dyn RendererEmitter>` captured
/// once from `VineHost::RendererEmitter()` so the drain task never holds a
/// reference to the full host across await points..
pub async fn SetTextEditorDecorations(Host:&dyn VineHost, Parameter:&Value) {
	let Ch = GetOrInitChannel(Host.RendererEmitter());

	let _ = Ch.Sender.send(Parameter.clone());
}
