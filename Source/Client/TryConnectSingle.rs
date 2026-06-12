//! Single connection attempt without retry logic. Tunes h2 transport
//! windows for loopback-to-Cocoon traffic (4 MB stream / 16 MB connection)
//! so a single rust-analyzer diagnostic emit (200-500 KB) doesn't cause
//! `WINDOW_UPDATE` ping-pong.
//!
//! On success stores the connected `CocoonClient` in
//! `Shared::SIDECAR_CLIENTS`. If `LAND_VINE_STREAMING=1` is set and the
//! crate is built with the `multiplexer` cargo feature, we also open the
//! bidirectional streaming multiplexer alongside the unary client;
//! failures there are logged and tolerated.

use std::time::Duration;

use crate::{
	Client::Shared::{CocoonClient, MAX_MESSAGE_SIZE_BYTES, SIDECAR_CLIENTS},
	Error::VineError,
	dev_log,
};

pub async fn Fn(SideCarIdentifier:&str, Endpoint:&str) -> Result<(), VineError> {
	let EndpointURL = if Endpoint.starts_with("http://") || Endpoint.starts_with("https://") {
		Endpoint.to_string()
	} else {
		format!("http://{}", Endpoint)
	};

	let UseTuned = std::env::var("LAND_TONIC_TUNED").as_deref() != Ok("0");

	let mut Channel = tonic::transport::Channel::from_shared(EndpointURL)
		.map_err(|E| VineError::RPCError(format!("Failed to create channel: {}", E)))?;

	if UseTuned {
		Channel = Channel
			.tcp_nodelay(true)
			.http2_keep_alive_interval(Duration::from_secs(10))
			.keep_alive_timeout(Duration::from_secs(20))
			.http2_adaptive_window(true)
			.initial_stream_window_size(4 * 1024 * 1024)
			.initial_connection_window_size(16 * 1024 * 1024)
			.concurrency_limit(1024)
			.buffer_size(256 * 1024)
			.timeout(Duration::from_secs(30))
			.connect_timeout(Duration::from_secs(5));
	}

	let Connected = Channel
		.connect()
		.await
		.map_err(|E| VineError::RPCError(format!("Failed to connect: {}", E)))?;

	// Pin the codec limits to `MAX_MESSAGE_SIZE_BYTES` so the wire bound
	// matches `ValidateMessageSize` instead of tonic's implicit defaults.
	let Client = CocoonClient::new(Connected)
		.max_decoding_message_size(MAX_MESSAGE_SIZE_BYTES)
		.max_encoding_message_size(MAX_MESSAGE_SIZE_BYTES);

	{
		let mut Pool = SIDECAR_CLIENTS.lock();

		Pool.insert(SideCarIdentifier.to_string(), Client.clone());
	}

	#[cfg(feature = "multiplexer")]
	{
		if std::env::var("LAND_VINE_STREAMING").as_deref() == Ok("1") {
			let SideCarForMux = SideCarIdentifier.to_string();

			match crate::Multiplexer::Multiplexer::Open(SideCarForMux, Client).await {
				Ok(_) => {
					dev_log!(
						"grpc",
						"[VineClient] streaming multiplexer opened for sidecar '{}'",
						SideCarIdentifier
					);
				},

				Err(Error) => {
					dev_log!(
						"grpc",
						"warn: [VineClient] streaming multiplexer open failed for '{}' ({}); falling back to unary",
						SideCarIdentifier,
						Error
					);
				},
			}
		}
	}

	#[cfg(not(feature = "multiplexer"))]
	{
		if std::env::var("LAND_VINE_STREAMING").as_deref() == Ok("1") {
			dev_log!(
				"grpc",
				"[VineClient] LAND_VINE_STREAMING=1 set but the `multiplexer` cargo feature is disabled; using unary"
			);
		}

		// Keep Client used so the compiler doesn't warn under
		// --no-default-features.
		let _ = &Client;
	}

	Ok(())
}
