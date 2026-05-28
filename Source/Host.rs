//! # Vine::Host
//!
//! [`VineHost`] is the embedder-facing seam between Vine and its consumer
//! runtime. Mountain (Tauri runtime), Air (background daemon), and any
//! Rust-side Cocoon client implement this trait to expose the minimum
//! surface Vine's notification handlers need: application-state access,
//! renderer event emission, and an [`IPCProvider`] handle for cross-channel
//! re-entrancy. Handlers operate on `&dyn VineHost` so a single handler
//! tree can be hosted against any embedder runtime.
//!
//! ## Design notes
//!
//! - [`ApplicationState`](VineHost::ApplicationState) returns
//!   `&dyn ApplicationStateAccess`. Vine treats application state as
//!   opaque; embedders decide what their state exposes via embedder-local
//!   sub-traits and downcasting.
//! - [`EmitToRenderer`](VineHost::EmitToRenderer) is the single entry point
//!   for "send a value to the workbench / Sky window." Mountain wires it to
//!   `tauri::WebviewWindow::emit`; Air leaves it as a no-op (no renderer).
//! - [`IPCProvider`](VineHost::IPCProvider) returns `Arc<dyn IPCProvider>`
//!   for handlers that need to re-enter the IPC bus.
//!
//! ## Stability
//!
//! Adding methods to [`VineHost`] is a compatible change so long as every
//! impl is updated in the same revision. Removing a method requires
//! sweeping every consumer.

use std::sync::Arc;

use serde_json::Value;

/// Opaque application-state handle exposed to Vine handlers.
///
/// Embedders downcast or call sub-trait methods declared in their own crate.
/// Vine itself never reaches into the state; handlers do.
pub trait ApplicationStateAccess: Send + Sync {
	/// Returns the embedder name (e.g. "Mountain", "Air"). Useful for
	/// diagnostic logs that fan in from multiple hosts.
	fn EmbedderName(&self) -> &'static str;
}

/// Cross-channel IPC provider abstraction.
///
/// Mirrors the shape of `CommonLibrary::IPC::IPCProvider` so consumers can
/// swap between the two with a single `use` change. Kept local to Vine to
/// avoid pulling `CommonLibrary` into the dependency graph.
pub trait IPCProvider: Send + Sync {
	/// Sends a request on the named channel and waits for a JSON response.
	fn SendRequest(&self, Channel:&str, Payload:Value) -> futures::future::BoxFuture<'_, crate::Error::Result<Value>>;

	/// Fire-and-forget notification on the named channel.
	fn SendNotification(&self, Channel:&str, Payload:Value);
}

/// The embedder-facing seam between Vine and its host runtime.
///
/// Implementations belong in the embedder crate (Mountain, Air, …), not in
/// Vine.
pub trait VineHost: Send + Sync {
	/// Returns the embedder's application state for handler use.
	fn ApplicationState(&self) -> &(dyn ApplicationStateAccess);

	/// Emits a JSON event on the named renderer channel. No-op for embedders
	/// that have no renderer (e.g. Air).
	fn EmitToRenderer(&self, Channel:&str, Payload:Value);

	/// Returns the cross-channel IPC provider. Used by handlers that need to
	/// re-enter the IPC bus (e.g. tree-view registration that needs to call
	/// `sky:replay-events`).
	fn IPCProvider(&self) -> Arc<dyn IPCProvider>;
}
