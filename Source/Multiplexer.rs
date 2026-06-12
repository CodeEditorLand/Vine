//! Bidirectional streaming multiplexer for the Vine gRPC bus.
//!
//! Owns one bidirectional h2 stream per sidecar. Inbound notifications fan
//! out to the process-wide broadcast
//! ([`crate::Client::SubscribeNotifications::Fn`]); inbound responses route
//! to the matching pending-request `oneshot` sender. Inbound reverse-RPC
//! requests dispatch to the process-wide [`RequestHandlerFn`] installed via
//! [`Multiplexer::InstallRequestHandler`], with the `GenericResponse` pushed
//! back over the same stream under the original correlation id; when no
//! handler is installed the frame is dropped with a warning. Cancellations
//! are accepted but dropped.
//!
//! Activated when `LAND_VINE_STREAMING=1` is set and the `multiplexer`
//! cargo feature is on; this is the LAND-PATCH B7-S6 P14.1 foundation.

use std::{
	collections::HashMap,
	sync::{
		Arc,
		atomic::{AtomicBool, AtomicU64, Ordering},
	},
	time::Duration,
};

use dashmap::DashMap;
use lazy_static::lazy_static;
use parking_lot::Mutex;
use serde_json::Value;
use tokio::sync::{mpsc, oneshot};
use tokio_stream::wrappers::ReceiverStream;
use tonic::Streaming;

use crate::{
	Error::VineError,
	Generated::{
		CancelOperationRequest,
		Envelope,
		GenericNotification,
		GenericRequest,
		GenericResponse,
		RpcError,
		cocoon_service_client::CocoonServiceClient,
		envelope::Payload,
	},
	dev_log,
};

/// Outbound queue capacity per multiplexer. Bounded so a stalled
/// sidecar applies backpressure to the producer side instead of
/// burning unbounded heap.
const SINK_CAPACITY:usize = 1024;

/// Handler for inbound reverse-RPC `Request` frames. The embedder
/// (Mountain) installs one process-wide via
/// [`Multiplexer::InstallRequestHandler`]; the read pump invokes it for
/// every inbound `Payload::Request` and pushes the returned
/// `GenericResponse` back over the stream under the original
/// correlation id.
pub type RequestHandlerFn =
	Arc<dyn Fn(GenericRequest) -> futures::future::BoxFuture<'static, GenericResponse> + Send + Sync>;

/// One multiplexer per sidecar connection. Holds the outbound sink,
/// the pending-request correlation map, and a shared-state shutdown
/// flag.
pub struct Multiplexer {
	SideCarIdentifier:String,

	Sink:mpsc::Sender<Envelope>,

	Pending:Arc<DashMap<u64, oneshot::Sender<GenericResponse>>>,

	NextRequestIdentifier:AtomicU64,

	Closed:AtomicBool,
}

lazy_static! {

	/// Process-wide registry, one entry per sidecar identifier.
	/// Lookup site for `SendNotification` / `SendRequest` to consult
	/// when `LAND_VINE_STREAMING=1`.
	static ref MULTIPLEXERS:Arc<Mutex<HashMap<String, Arc<Multiplexer>>>> = Arc::new(Mutex::new(HashMap::new()));

	/// Process-wide reverse-RPC request handler. Read lazily by every
	/// read pump on each inbound `Request` frame so installation order
	/// relative to `Open` does not matter.
	static ref REQUEST_HANDLER:Mutex<Option<RequestHandlerFn>> = Mutex::new(None);
}

impl Multiplexer {
	/// Open a bidirectional streaming channel against an existing
	/// `CocoonServiceClient`. Spawns the read pump as a detached
	/// tokio task and registers the multiplexer in the global
	/// registry. Returns once the stream is established.
	pub async fn Open(
		SideCarIdentifier:String,

		mut Client:CocoonServiceClient<tonic::transport::Channel>,
	) -> Result<Arc<Self>, VineError> {
		let (Sink, OutboundReceiver) = mpsc::channel::<Envelope>(SINK_CAPACITY);

		let OutboundStream = ReceiverStream::new(OutboundReceiver);

		let Response = Client
			.open_channel_from_mountain(OutboundStream)
			.await
			.map_err(|S| VineError::RPCError(format!("OpenChannelFromMountain failed: {}", S)))?;

		let InboundStream:Streaming<Envelope> = Response.into_inner();

		let SelfReference = Arc::new(Self {
			SideCarIdentifier:SideCarIdentifier.clone(),
			Sink,
			Pending:Arc::new(DashMap::new()),
			NextRequestIdentifier:AtomicU64::new(1),
			Closed:AtomicBool::new(false),
		});

		// Spawn the read pump.
		let SelfForReadPump = SelfReference.clone();

		tokio::spawn(async move {
			ReadPump(InboundStream, SelfForReadPump).await;
		});

		// Register globally so consumers can look us up.
		MULTIPLEXERS.lock().insert(SideCarIdentifier, SelfReference.clone());

		Ok(SelfReference)
	}

	/// Look up the multiplexer for a sidecar. Returns `None` if no
	/// streaming connection has been opened for that sidecar (the
	/// caller should fall back to the unary path).
	pub fn Lookup(SideCarIdentifier:&str) -> Option<Arc<Self>> { MULTIPLEXERS.lock().get(SideCarIdentifier).cloned() }

	/// Drop the registry entry. Called by the read-pump when the
	/// stream closes.
	pub fn Deregister(SideCarIdentifier:&str) { MULTIPLEXERS.lock().remove(SideCarIdentifier); }

	/// Install the process-wide handler for inbound reverse-RPC
	/// `Request` frames. The Multiplexer cannot depend on the
	/// embedder's dispatch tree, so the embedder injects it here
	/// (Mountain does this at gRPC-service construction). Replaces any
	/// previously installed handler.
	pub fn InstallRequestHandler(Handler:RequestHandlerFn) { *REQUEST_HANDLER.lock() = Some(Handler); }

	/// Send a notification frame (fire-and-forget). Non-blocking
	/// modulo Sink backpressure (capacity `SINK_CAPACITY`).
	pub async fn Notify(&self, Method:String, Parameters:Value) -> Result<(), VineError> {
		if self.Closed.load(Ordering::Relaxed) {
			return Err(VineError::ClientNotConnected(self.SideCarIdentifier.clone()));
		}

		let Bytes = serde_json::to_vec(&Parameters)?;

		let Frame = Envelope {
			payload:Some(Payload::Notification(GenericNotification { method:Method, parameter:Bytes })),

			channel_id:0,
		};

		self.Sink
			.send(Frame)
			.await
			.map_err(|_| VineError::RPCError(format!("Sink closed for sidecar {}", self.SideCarIdentifier)))
	}

	/// Send a request and await the matching response. Cancels the
	/// pending entry on timeout. The future is `Send + 'static`-clean
	/// so callers can drive it inside `tokio::select!` for finer-
	/// grained cancellation.
	pub async fn Request(&self, Method:String, Parameters:Value, Timeout:Duration) -> Result<Value, VineError> {
		if self.Closed.load(Ordering::Relaxed) {
			return Err(VineError::ClientNotConnected(self.SideCarIdentifier.clone()));
		}

		let Identifier = self.NextRequestIdentifier.fetch_add(1, Ordering::Relaxed);

		let (Tx, Rx) = oneshot::channel();

		self.Pending.insert(Identifier, Tx);

		let Bytes = serde_json::to_vec(&Parameters)?;

		let MethodForError = Method.clone();

		let Frame = Envelope {
			payload:Some(Payload::Request(GenericRequest {
				request_identifier:Identifier,
				method:Method,
				parameter:Bytes,
			})),

			channel_id:0,
		};

		if self.Sink.send(Frame).await.is_err() {
			self.Pending.remove(&Identifier);

			return Err(VineError::RPCError(format!(
				"Sink closed for sidecar {}",
				self.SideCarIdentifier
			)));
		}

		match tokio::time::timeout(Timeout, Rx).await {
			Ok(Ok(Response)) => {
				if let Some(Error) = Response.error {
					return Err(VineError::RPCError(format!("code={} message={}", Error.code, Error.message)));
				}

				if Response.result.is_empty() {
					return Ok(Value::Null);
				}

				serde_json::from_slice::<Value>(&Response.result).map_err(VineError::SerializationError)
			},

			Ok(Err(_)) => {
				self.Pending.remove(&Identifier);

				Err(VineError::RPCError(
					"response sender closed (peer disconnect mid-request)".into(),
				))
			},

			Err(_) => {
				self.Pending.remove(&Identifier);

				Err(VineError::RequestTimeout {
					SideCarIdentifier:self.SideCarIdentifier.clone(),
					MethodName:MethodForError,
					TimeoutMilliseconds:Timeout.as_millis() as u64,
				})
			},
		}
	}

	/// Send a Cancel frame asking the peer to abort an in-flight
	/// request matching `RequestIdentifier`. Best-effort; the peer
	/// chooses whether to honour it.
	pub async fn Cancel(&self, RequestIdentifier:u64) -> Result<(), VineError> {
		if self.Closed.load(Ordering::Relaxed) {
			return Ok(());
		}

		let Frame = Envelope {
			payload:Some(Payload::Cancel(CancelOperationRequest {
				request_identifier_to_cancel:RequestIdentifier,
			})),

			channel_id:0,
		};

		let _ = self.Sink.send(Frame).await;

		Ok(())
	}

	/// Check whether the multiplexed stream has been closed by the remote
	/// side-car (or shut down locally). `true` means no further I/O will be
	/// attempted on this channel.
	pub fn IsClosed(&self) -> bool { self.Closed.load(Ordering::Relaxed) }

	/// Borrow the side-car identifier that identifies which remote endpoint
	/// this multiplexer is paired with.
	pub fn SideCarIdentifierBorrow(&self) -> &str { &self.SideCarIdentifier }
}

/// Drains the inbound side of the bidirectional stream.
///
/// Notifications fan out to the process-wide broadcast; responses wake
/// the parked `Request` future. Reverse-RPC requests dispatch to the
/// installed [`RequestHandlerFn`] on a detached task (a slow handler
/// must not stall the pump) with the response pushed back via the sink
/// under the original correlation id; without an installed handler the
/// frame is dropped with a warning. Cancellations are dropped - the
/// unary path has no cancellation either, so accepting and dropping
/// preserves equivalence with the non-streaming fallback.
async fn ReadPump(mut Stream:Streaming<Envelope>, State:Arc<Multiplexer>) {
	use futures_util::StreamExt;

	while let Some(FrameResult) = Stream.next().await {
		let Frame = match FrameResult {
			Ok(F) => F,

			Err(Status) => {
				dev_log!(
					"grpc",
					"[Vine::Multiplexer] read err on {}: {}",
					State.SideCarIdentifier,
					Status
				);

				break;
			},
		};

		let Payload = match Frame.payload {
			Some(P) => P,

			None => continue,
		};

		match Payload {
			Payload::Notification(N) => {
				let Parameters:Value = if N.parameter.is_empty() {
					Value::Null
				} else {
					serde_json::from_slice(&N.parameter).unwrap_or(Value::Null)
				};

				crate::Client::PublishNotification::Fn(&State.SideCarIdentifier, &N.method, &Parameters);
			},

			Payload::Response(R) => {
				let Identifier = R.request_identifier;

				if let Some((_, Sender)) = State.Pending.remove(&Identifier) {
					let _ = Sender.send(R);
				}

				// A Response with no matching pending entry is a
				// duplicate or post-cancel arrival; drop silently.
			},

			Payload::Request(R) => {
				let Handler = REQUEST_HANDLER.lock().clone();

				match Handler {
					Some(Handler) => {
						let RequestIdentifier = R.request_identifier;

						let Sink = State.Sink.clone();

						let SideCarIdentifier = State.SideCarIdentifier.clone();

						let ResponseFuture = Handler(R);

						tokio::spawn(async move {
							let mut Response = ResponseFuture.await;

							if Response.request_identifier == 0 {
								Response.request_identifier = RequestIdentifier;
							}

							let Frame = Envelope { payload:Some(Payload::Response(Response)), channel_id:0 };

							if Sink.send(Frame).await.is_err() {
								dev_log!(
									"grpc",
									"[Vine::Multiplexer] sink closed before response id={} sidecar={}",
									RequestIdentifier,
									SideCarIdentifier
								);
							}
						});
					},

					None => {
						dev_log!(
							"grpc",
							"warn: [Vine::Multiplexer] dropping inbound Request method={} id={} sidecar={}: no \
							 request handler installed",
							R.method,
							R.request_identifier,
							State.SideCarIdentifier
						);
					},
				}
			},

			Payload::Cancel(_) => {

				// Cancel propagation is a no-op; the unary path doesn't
				// support cancellation either, so accepting and dropping
				// preserves equivalence with the non-streaming fallback.
			},
		}
	}

	State.Closed.store(true, Ordering::Relaxed);

	// Drain pending senders with disconnect errors so awaiting
	// fibers don't hang forever.
	let Keys:Vec<u64> = State.Pending.iter().map(|R| *R.key()).collect();

	for Key in Keys {
		if let Some((_, Sender)) = State.Pending.remove(&Key) {
			let _ = Sender.send(GenericResponse {
				request_identifier:Key,
				result:Vec::new(),
				error:Some(RpcError { code:-32099, message:"stream closed".into(), data:Vec::new() }),
			});
		}
	}

	Multiplexer::Deregister(&State.SideCarIdentifier);

	dev_log!("grpc", "[Vine::Multiplexer] closed sidecar={}", State.SideCarIdentifier);
}
