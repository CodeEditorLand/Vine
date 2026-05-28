//! Spawn a detached tokio task that runs a tonic gRPC `Router` on the
//! given address.
//!
//! Lifted into the Vine crate so every embedder (Mountain's
//! `MountainVinegRPCService`, Air's `AirVinegRPCService`, any Rust-side
//! Cocoon client) shares one bind path with consistent dev-log
//! instrumentation.
//!
//! ## Why a `Router`, not a `Server` builder
//!
//! tonic's `Server::builder()` returns a [`tonic::transport::Server`]. After
//! the first `.add_service(...)` call you get a
//! [`tonic::transport::server::Router`]. The Router type carries the
//! generic stack of registered services, which is opaque to Vine - only the
//! embedder knows the concrete service set. So Vine accepts a constructed
//! `Router` and runs the boilerplate.
//!
//! ## Lifecycle
//!
//! The serve future runs until either the OS socket closes or the tokio
//! runtime shuts down. The task is detached; no `JoinHandle` is returned.
//! Errors are logged via `dev_log!` and otherwise swallowed - the embedder
//! decides whether to retry, restart, or fail loud via separate
//! supervisory code.

use std::net::SocketAddr;

use tonic::transport::server::Router;

use crate::dev_log;

/// Spawn a detached tokio task that serves `Router` on `Address`.
///
/// # Parameters
///
/// * `ServerName` - label used in dev-log messages.
/// * `Address`    - resolved socket address (callers should run
///   [`crate::Server::ValidateSocketAddress::Fn`] first).
/// * `Router`     - constructed tonic Router with services already added.
///
/// # Behaviour
///
/// - Emits an `info`-level dev_log on start.
/// - Awaits `Router.serve(Address)`.
/// - On `Ok(())`, dev_logs graceful shutdown.
/// - On `Err`, dev_logs the error and exits the task.
pub fn Fn(ServerName:String, Address:SocketAddr, Router:Router) {
	tokio::spawn(async move {
		dev_log!("grpc", "[Vine::Server] Starting {} gRPC server on {}", ServerName, Address);

		let Result_ = Router.serve(Address).await;

		match Result_ {
			Ok(_) => {
				dev_log!("grpc", "[Vine::Server] {} server shut down gracefully", ServerName);
			},

			Err(Error) => {
				dev_log!("grpc", "error: [Vine::Server] {} gRPC server error: {}", ServerName, Error);
			},
		}
	});
}
