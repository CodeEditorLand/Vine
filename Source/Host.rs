//! # Vine::Host
//!
//! The `VineHost` trait is the embedder-facing seam between Vine and its
//! consumer. Mountain (Tauri runtime), Air (background daemon), and any
//! future Rust Cocoon client implement this trait to expose the minimum
//! surface Vine's notification handlers need: application-state access,
//! renderer event emission, and an `IPCProvider` handle for cross-channel
//! re-entrancy.
//!
//! Defined as part of `.hermes/plan/Mountain-Crate-Split.md` task #1 Phase 2
//! - the goal is that notification handlers operate on `&dyn VineHost`
//! instead of a Mountain-runtime-specific struct. Mountain ships a
//! `MountainVineHost` impl that holds `Arc<MountainEnvironment>` plus a
//! `tauri::AppHandle`. Air ships an `AirVineHost` impl that wires into Air's
//! `ApplicationState` and HTTP/Indexing daemons.
//!
//! ## Design notes
//!
//! - `application_state` returns `&dyn ApplicationStateAccess` (declared in
//!   this module). Vine treats application state as opaque; embedders decide
//!   what their state exposes. Sub-traits live in `Host::State`.
//! - `emit_to_renderer` is the single entry point for "send a value to the
//!   workbench / Sky window." Mountain wires it to `tauri::WebviewWindow::emit`;
//!   Air leaves it as a no-op (no renderer).
//! - `ipc_provider` returns `Arc<dyn IPCProvider>`. This is the eventual
//!   integration point with `CommonLibrary::IPC::IPCProvider`. Until the
//!   Common-trait wiring lands, the local `IPCProvider` trait below stands
//!   in as a forward-compatible placeholder.
//!
//! ## Stability
//!
//! Extending the trait is allowed during synthesis: if a handler needs a
//! capability the current `VineHost` cannot express, add a method here
//! rather than narrowing the destination crate. Removing a method requires
//! sweeping every consumer in the same change.

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
/// Mirrors the shape of `CommonLibrary::IPC::IPCProvider` so that the
/// migration from this local trait to Common's canonical trait is a single
/// `use` swap. Kept local for now to avoid pulling `CommonLibrary` into
/// `Vine`'s dependency graph before the Common refactor (#5) lands.
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
