//! Send a request and await a response. Validates method-name length and
//! message size, prefers the streaming multiplexer when
//! `LAND_VINE_STREAMING=1` is on and the `multiplexer` cargo feature is
//! enabled (falls through to unary on any failure except the authoritative
//! streaming-path timeout), enforces a per-call timeout via
//! `tokio::time::timeout`, and updates per-connection activity / failure
//! metadata on completion.
//!
//! The hard upper bound on per-call timeouts is
//! [`crate::DefaultRequestTimeoutMs`] (15 000 ms); when the caller passes
//! `0` for `TimeoutMilliseconds` the unary path falls back to
//! `crate::Client::Shared::DEFAULT_TIMEOUT_MS` (5 000 ms).
//!
//! [`FnCancellable`] additionally accepts a `watch::Receiver<bool>` cancel
//! signal; when the sender flips it to `true` mid-flight the unary call is
//! abandoned, a fire-and-forget `CancelOperation` carrying the wire
//! `RequestIdentifier` is sent on the same client channel, and
//! [`VineError::RequestCanceled`] is returned. [`Fn`] delegates to it with
//! a never-cancelled signal. A unary timeout also fires a best-effort
//! `CancelOperation` so the side-car aborts work Mountain has stopped
//! waiting for.

use std::{sync::OnceLock, time::Duration};

use serde_json::{Value, from_slice, to_vec};
use tokio::{sync::watch, time::timeout};

use crate::{
	Client::{
		IsShuttingDown,
		Shared::{
			DEFAULT_TIMEOUT_MS,
			RecordSideCarFailure,
			SIDECAR_CLIENTS,
			UpdateSideCarActivity,
			ValidateMessageSize,
		},
	},
	Error::VineError,
	Generated::{CancelOperationRequest, GenericRequest},
	dev_log,
};

/// Send a request and await a response. Validates method-name length and message size, prefers the streaming multiplexer wh
///
pub async fn Fn(
	SideCarIdentifier:&str,

	Method:String,

	Parameters:Value,

	TimeoutMilliseconds:u64,
) -> Result<Value, VineError> {
	FnCancellable(SideCarIdentifier, Method, Parameters, TimeoutMilliseconds, NeverCancelled()).await
}

pub async fn FnCancellable(
	SideCarIdentifier:&str,

	Method:String,

	Parameters:Value,

	TimeoutMilliseconds:u64,

	CancelSignal:watch::Receiver<bool>,
) -> Result<Value, VineError> {
	if IsShuttingDown::Fn() {
		return Err(VineError::ClientNotConnected(SideCarIdentifier.to_string()));
	}

	if Method.is_empty() || Method.len() > 128 {
		return Err(VineError::RPCError(
			"Method name must be between 1 and 128 characters".to_string(),
		));
	}

	if *CancelSignal.borrow() {
		return Err(VineError::RequestCanceled {
			SideCarIdentifier:SideCarIdentifier.to_string(),
			MethodName:Method,
		});
	}

	let TimeoutDuration =
		Duration::from_millis(if TimeoutMilliseconds > 0 { TimeoutMilliseconds } else { DEFAULT_TIMEOUT_MS });

	#[cfg(feature = "multiplexer")]
	{
		if std::env::var("LAND_VINE_STREAMING").as_deref() == Ok("1") {
			if let Some(Mux) = crate::Multiplexer::Multiplexer::Lookup(SideCarIdentifier) {
				if !Mux.IsClosed() {
					match Mux.Request(Method.clone(), Parameters.clone(), TimeoutDuration).await {
						Ok(Result_) => {
							UpdateSideCarActivity(SideCarIdentifier);

							return Ok(Result_);
						},

						Err(VineError::RequestTimeout { .. }) => {
							return Err(VineError::RequestTimeout {
								SideCarIdentifier:SideCarIdentifier.to_string(),
								MethodName:Method,
								TimeoutMilliseconds:TimeoutDuration.as_millis() as u64,
							});
						},

						Err(Error) => {
							dev_log!(
								"grpc",
								"warn: [VineClient::SendRequest] streaming send failed for '{}::{}' ({}); falling \
								 back to unary",
								SideCarIdentifier,
								Method,
								Error
							);
						},
					}
				}
			}
		}
	}

	let ParameterBytes =
		to_vec(&Parameters).map_err(|E| VineError::RPCError(format!("Failed to serialize parameters: {}", E)))?;

	ValidateMessageSize(&ParameterBytes)?;

	let Client = {
		let Pool = SIDECAR_CLIENTS.lock();

		Pool.get(SideCarIdentifier).cloned()
	};

	let Some(mut Client) = Client else {
		return Err(VineError::ClientNotConnected(SideCarIdentifier.to_string()));
	};

	use std::sync::atomic::{AtomicU64, Ordering as AO};

	static REQ_SEQ:AtomicU64 = AtomicU64::new(1);

	let RequestIdentifier = REQ_SEQ.fetch_add(1, AO::Relaxed);

	let MethodForLog = Method.clone();

	let Request = GenericRequest { request_identifier:RequestIdentifier, method:Method, parameter:ParameterBytes };

	let mut CancelClient = Client.clone();

	let Result_ = tokio::select! {
		Outcome = timeout(TimeoutDuration, Client.process_mountain_request(Request)) => Outcome,

		_ = WaitForCancel(CancelSignal) => {
			tokio::spawn(async move {
				match CancelClient
					.cancel_operation(CancelOperationRequest { request_identifier_to_cancel:RequestIdentifier })
					.await
				{
					Ok(_) => {
						dev_log!(
							"grpc",
							"[VineClient::SendRequest] CancelOperation delivered for request {}",
							RequestIdentifier
						);
					},

					Err(Status) => {
						dev_log!(
							"grpc",
							"warn: [VineClient::SendRequest] CancelOperation for request {} failed: {}",
							RequestIdentifier,
							Status
						);
					},
				}
			});

			dev_log!(
				"grpc",
				"[VineClient::SendRequest] request {} ('{}::{}') cancelled by caller",
				RequestIdentifier,
				SideCarIdentifier,
				MethodForLog
			);

			return Err(VineError::RequestCanceled {
				SideCarIdentifier:SideCarIdentifier.to_string(),
				MethodName:MethodForLog,
			});
		},
	};

	match Result_ {
		Ok(Ok(Response)) => {
			UpdateSideCarActivity(SideCarIdentifier);

			dev_log!(
				"grpc",
				"[VineClient] Request sent successfully to sidecar '{}': method='{}'",
				SideCarIdentifier,
				MethodForLog
			);

			let InnerResponse = Response.into_inner();

			let ResultBytes = InnerResponse.result;

			let ResultValue:Value = from_slice(&ResultBytes)
				.map_err(|E| VineError::RPCError(format!("Failed to deserialize response: {}", E)))?;

			if let Some(ErrorData) = InnerResponse.error {
				return Err(VineError::RPCError(format!(
					"RPC error from sidecar: code={}, message={}",
					ErrorData.code, ErrorData.message
				)));
			}

			Ok(ResultValue)
		},

		Ok(Err(Status)) => {
			RecordSideCarFailure(SideCarIdentifier);

			Err(VineError::RPCError(format!("gRPC error: {}", Status)))
		},

		Err(_) => {
			RecordSideCarFailure(SideCarIdentifier);

			tokio::spawn(async move {
				if let Err(Status) = CancelClient
					.cancel_operation(CancelOperationRequest { request_identifier_to_cancel:RequestIdentifier })
					.await
				{
					dev_log!(
						"grpc",
						"warn: [VineClient::SendRequest] CancelOperation after timeout for request {} failed: {}",
						RequestIdentifier,
						Status
					);
				}
			});

			Err(VineError::RequestTimeout {
				SideCarIdentifier:SideCarIdentifier.to_string(),
				MethodName:MethodForLog,
				TimeoutMilliseconds:TimeoutDuration.as_millis() as u64,
			})
		},
	}
}

/// A shared, always-`false` cancel signal whose sender is held alive for the
/// process lifetime, so `Fn` callers never observe a spurious cancellation.
fn NeverCancelled() -> watch::Receiver<bool> {
	static CHANNEL:OnceLock<(watch::Sender<bool>, watch::Receiver<bool>)> = OnceLock::new();

	CHANNEL.get_or_init(|| watch::channel(false)).1.clone()
}

/// Resolves once the signal reads `true`. A dropped sender means the caller
/// can no longer cancel, so the future pends forever instead of resolving.
async fn WaitForCancel(mut Signal:watch::Receiver<bool>) {
	loop {
		if *Signal.borrow_and_update() {
			return;
		}

		if Signal.changed().await.is_err() {
			std::future::pending::<()>().await;
		}
	}
}
