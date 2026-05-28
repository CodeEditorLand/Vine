//! Spawn a detached tokio task that serves a tonic gRPC `Router` with
//! graceful shutdown signalling.
//!
//! Synthesised from `Air/Source/Initialize/Service/Vine/StartService.rs`,
//! which uses `tonic::transport::Server::serve_with_shutdown(...)` so the
//! Air daemon can drain in-flight RPCs before exiting. Mountain's
//! `Initialize.rs` uses the simpler [`crate::Server::SpawnBindTask::Fn`]
//! that has no shutdown signal; both shapes are kept side-by-side here so
//! embedders pick whichever matches their lifecycle model.
//!
//! ## Lifecycle
//!
//! - The task runs until `ShutdownSignal` resolves OR the OS socket closes.
//! - On `Ok(())` exit, dev_log marks clean shutdown.
//! - On `Err`, dev_log captures the transport error.
//!
//! ## Choosing between the two helpers
//!
//! | Need                                  | Helper                            |
//! | ------------------------------------- | --------------------------------- |
//! | Serve until process termination       | `SpawnBindTask`                   |
//! | Serve until a signal fires (recommended for daemons) | `SpawnBindTaskWithShutdown` |

use std::{future::Future, net::SocketAddr};

use tonic::transport::server::Router;

use crate::dev_log;

/// Spawn a detached tokio task that serves `Router` on `Address` until
/// `ShutdownSignal` resolves.
///
/// # Parameters
///
/// * `ServerName`      - label used in dev-log messages.
/// * `Address`         - resolved socket address (callers should run
///   [`crate::Server::ValidateSocketAddress::Fn`] first).
/// * `Router`          - constructed tonic Router with services already added.
/// * `ShutdownSignal`  - any `Future<Output = ()> + Send + 'static`. When it
///   resolves, the server gracefully stops accepting new connections and drains
///   in-flight calls.
pub fn Fn<S>(ServerName:String, Address:SocketAddr, Router:Router, ShutdownSignal:S)
where
	S: Future<Output = ()> + Send + 'static, {
	tokio::spawn(async move {
		dev_log!(
			"grpc",
			"[Vine::Server] Starting {} gRPC server on {} (with shutdown signal)",
			ServerName,
			Address
		);

		let Result_ = Router.serve_with_shutdown(Address, ShutdownSignal).await;

		match Result_ {
			Ok(_) => {
				dev_log!("grpc", "[Vine::Server] {} server stopped cleanly", ServerName);
			},

			Err(Error) => {
				dev_log!("grpc", "error: [Vine::Server] {} gRPC server error: {}", ServerName, Error);
			},
		}
	});
}
