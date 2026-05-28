#![allow(
	non_snake_case,
	non_camel_case_types,
	non_upper_case_globals,
	dead_code,
	unused_imports,
	unused_variables,
	unused_assignments
)]

//! # Vine 🌿 - gRPC Protocol Layer for Land 🏞️
//!
//! Vine is the canonical home of the gRPC IPC schema + runtime that wires
//! the Land elements together:
//!
//! - **Mountain** (Tauri editor host) - hosts the `MountainService` gRPC
//!   server; routes notifications from Cocoon and Air into Tauri's renderer.
//! - **Cocoon** (Node.js extension host) - speaks Vine to invoke VS
//!   Code-shaped operations on Mountain.
//! - **Air** (background daemon) - speaks Vine as a client to query Mountain
//!   for editor state when running indexing / update / download tasks; also
//!   hosts its own `AirService` gRPC server on `[::1]:50053`.
//!
//! ## Module layout
//!
//! - [`Error`] - canonical [`VineError`](Error::VineError) variants and
//!   `From` conversions for `serde_json`, `tonic::transport`, `tonic::Status`,
//!   `http::uri::InvalidUri`, and `std::net::AddrParseError`.
//! - [`Host`] - the [`VineHost`](Host::VineHost) trait,
//!   [`IPCProvider`](Host::IPCProvider), and
//!   [`ApplicationStateAccess`](Host::ApplicationStateAccess) - the seam
//!   between Vine handlers and any embedder runtime.
//! - [`Generated`] - prost-built message types + tonic clients and servers
//!   produced from [`Proto/Vine.proto`](../Proto/Vine.proto) by `build.rs`.
//! - [`Client`] - client building blocks (cargo feature `client`).
//! - [`Server`] - server scaffolding + notification handler tree (cargo
//!   feature `server`).
//! - [`Multiplexer`] - bidirectional envelope multiplexer
//!   (`OpenChannelFromMountain` / `OpenChannelFromCocoon` per
//!   LAND-PATCH B7-S6 P14.1; cargo feature `multiplexer`).
//!
//! ## Cargo features
//!
//! | Feature | Default | Pulls in |
//! | --- | :---: | --- |
//! | `client` | ✅ | `Source/Client/` - tonic-generated client stub wrappers + connection pool |
//! | `server` | ✅ | `Source/Server/` - bind helpers + notification handler tree |
//! | `multiplexer` |  | bidirectional streaming envelope multiplexer |
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
/// `SendRequestToSideCar` invocation that does not pass an explicit
/// override. Per-call overrides are supported - long-running tree-view
/// fetches use ~1 500 ms, indexing queries use the default.
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
