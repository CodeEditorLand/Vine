//! # Vine::Server
//!
//! Server-side gRPC scaffolding for embedders that *host* Vine services
//! (Mountain hosting `MountainService`, Cocoon-Rust hosting `CocoonService`,
//! Air hosting `AirService`, …).
//!
//! ## Synthesis status (2026-05-28)
//!
//! This submodule is the destination of `Mountain/Source/Vine/Server/`. The
//! current Mountain inventory (kept as source of truth until the migration
//! phase lands) is:
//!
//! - `Source/Vine/Server/MountainVinegRPCService.rs` - top-level service
//!   implementation, `SendRequestToSideCar` envelope, notification fan-out
//! - `Source/Vine/Server/Initialize.rs` - server bootstrap + bind
//! - `Source/Vine/Server/Notification/*` (~90 files) - one file per
//!   notification kind. The atomization is part of the stable surface;
//!   **do not collapse handlers into mega-modules during port**.
//!
//! Critical perf work that must survive the synthesis port:
//!
//! - `DecorationTypeLifecycle.rs` - channel-drain debounce (was 16 ms sleep)
//! - `ProgressReport.rs` - channel-drain debounce (was 16 ms sleep)
//! - `RegisterCommand.rs` - channel-drain debounce (was 16 ms sleep)
//! - `EnqueueTreeViewEmit.rs` (in `RPC/CocoonService/TreeView/`) - batched
//!   `sky://tree-view/create` emit with `views: [...]` payload
//!
//! The synthesised handlers operate on `&dyn crate::Host::VineHost` instead
//! of a concrete Mountain struct, so Air and any future Rust client can host
//! the same handlers against their own `VineHost` impl.
//!
//! Until the port runs, this module is intentionally empty. The cargo feature
//! `server` (enabled by default) keeps the surface area available for
//! consumers that opt into server-side compilation.

// Intentionally empty - populated by Track-B task #1 Phase 2 follow-up
// session. See `.hermes/plan/Vine-Synthesis-Audit.md` for the file inventory
// and the perf-critical patterns each handler must preserve.
