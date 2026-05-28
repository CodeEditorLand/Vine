#![allow(
	non_snake_case,
	non_camel_case_types,
	non_upper_case_globals,
	dead_code,
	unused_imports,
	unused_variables,
	unused_assignments
)]

//! # Vine 🌿 - The gRPC Protocol Layer for Land 🏞️
//!
//! Vine is the canonical home of the gRPC IPC schema and runtime that wires
//! the Land elements together:
//!
//! - **Mountain** (Tauri editor host) - runs Vine's server side; routes
//!   notifications from Cocoon and Air into Tauri's renderer.
//! - **Cocoon** (Node.js extension host) - speaks Vine to invoke VS Code-shaped
//!   operations on Mountain.
//! - **Air** (background daemon) - speaks Vine as a client to query Mountain
//!   for editor state when running indexing / update / download tasks.
//!
//! ## Synthesis status (2026-05-28)
//!
//! This crate is the destination of Track-B task #1 from
//! `.hermes/plan/Mountain-Crate-Split.md`. It is being synthesised from
//! `Mountain/Source/Vine/`, which today remains the source of truth and the
//! production code path. The migration runs in three phases:
//!
//! 1. **Synthesis** (this commit set) - `Element/Vine` exposes the canonical
//!    protocol surface as a standalone crate. Mountain's in-tree module is
//!    untouched.
//! 2. **Consume** (next session) - Mountain gains a `MountainVineHost` impl and
//!    re-exports `Vine::*` from `Source/Vine/` as a `#[deprecated]` shim. Air's
//!    `Source/Vine/` collapses to the same shim shape.
//! 3. **Drop shim** (after Cocoon TS regen + Air client conversion verified) -
//!    Mountain stops re-exporting; `Element/Vine` is the only home.
//!
//! ## Module layout
//!
//! - [`Error`] - canonical [`VineError`](Error::VineError) variants and `From`
//!   conversions for `serde_json`, `tonic::transport`, `tonic::Status`,
//!   `http::uri::InvalidUri`, and `std::net::AddrParseError`.
//! - [`Host`] - the `VineHost` trait + `IPCProvider` + `ApplicationStateAccess`
//!   seam between Vine and its embedder runtime.
//! - [`Generated`] - prost-built message types + tonic clients and servers,
//!   produced from [`Proto/Vine.proto`](../Proto/Vine.proto) by `build.rs`.
//! - [`Client`] - reusable client building blocks (cargo feature `client`).
//! - [`Server`] - reusable server scaffolding (cargo feature `server`).
//!
//! ## Cargo features
//!
//! | Feature | Default | Pulls in |
//! | --- | :---: | --- |
//! | `client` | ✅ | `Source/Client/` - thin wrappers around tonic-generated client stubs |
//! | `server` | ✅ | `Source/Server/` - notification-dispatch primitives + handler scaffolding |
//! | `multiplexer` |  | the bidirectional envelope multiplexer (LAND-PATCH B7-S6 P14.1) |
//!
//! ## Port allocation
//!
//! - `50051` - Mountain Vine server (Cocoon ↔ Mountain)
//! - `50052` - Cocoon Vine server (Mountain → Cocoon callbacks)
//! - `50053` - Air Vine server (Mountain / external → Air)

pub mod DevLog;

pub mod Error;

pub mod Generated;

pub mod Host;

#[cfg(feature = "client")]
pub mod Client;

#[cfg(feature = "server")]
pub mod Server;

#[cfg(feature = "multiplexer")]
pub mod Multiplexer;

/// Canonical Vine protocol version. Sent on every gRPC envelope; receivers
/// reject mismatched versions early in the dispatch path.
pub const ProtocolVersion:u32 = 1;

/// Default maximum gRPC message size, in bytes (4 MB).
///
/// Mirrors tonic's default. The dispatch layer enforces this at the envelope
/// boundary so individual handlers can ignore size validation.
pub const DefaultMaxMessageSize:usize = 4 * 1024 * 1024;

/// Default request timeout, in milliseconds, applied to every
/// `SendRequestToSideCar` invocation that does not pass an explicit override.
///
/// Mountain's mature implementation uses 15 000 ms; the Cradle session
/// reduced `tree:getChildren` to 1 500 ms via per-call override. Both are
/// expressible against this default.
pub const DefaultRequestTimeoutMs:u64 = 15_000;

/// Default Mountain Vine server bind address.
pub const DefaultMountainAddress:&str = "[::1]:50051";

/// Default Cocoon Vine server bind address.
pub const DefaultCocoonAddress:&str = "[::1]:50052";

/// Default Air Vine server bind address.
pub const DefaultAirAddress:&str = "[::1]:50053";

/// Re-export the canonical error type at the crate root so downstream
/// consumers can write `use Vine::VineError;` without spelling out the
/// `Error::` module path.
pub use Error::{Result, VineError};
/// Re-export the embedder seam types at the crate root for the same reason.
pub use Host::{ApplicationStateAccess, IPCProvider, VineHost};
