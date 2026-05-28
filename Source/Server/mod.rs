//! # Vine::Server
//!
//! Server-side gRPC scaffolding shared by every embedder that hosts a Vine
//! service: Mountain (`MountainService`), Cocoon-Rust (future), Air
//! (`AirService`). The pieces below are the generic boilerplate that every
//! bind site needs; concrete service implementations stay in their owning
//! crate (Mountain hosts `MountainVinegRPCService`, Air hosts
//! `AirVinegRPCService`).
//!
//! ## Synthesis status (2026-05-28)
//!
//! - [`Constants`] - default ports / timeouts / message-size cap.
//! - [`ValidateSocketAddress`] - port-and-format pre-flight check (was
//!   `Mountain::Vine::Server::Initialize::ValidateSocketAddress`).
//! - [`SpawnBindTask`] - the detached `tokio::spawn` that runs
//!   `Router::serve(Address)` with consistent dev-log instrumentation
//!   (extracted from `Mountain::Vine::Server::Initialize::Initialize`).
//!
//! ## Pending synthesis
//!
//! `Mountain/Source/Vine/Server/MountainVinegRPCService.rs` and the ~90
//! `Source/Vine/Server/Notification/*` handlers are intentionally NOT
//! ported in this slice. Those depend on Mountain-specific runtime types
//! (`Arc<MountainEnvironment>`, `tauri::AppHandle`, `ApplicationRunTime`)
//! and the right abstraction (a `VineHost` extension carrying the provider
//! surface each handler reaches into) is still being designed. The
//! atomization is part of the stable surface; **do not collapse handlers
//! into mega-modules during port**.
//!
//! Critical perf work that must survive the synthesis port:
//!
//! - `DecorationTypeLifecycle.rs` - channel-drain debounce (was 16 ms sleep)
//! - `ProgressReport.rs` - channel-drain debounce (was 16 ms sleep)
//! - `RegisterCommand.rs` - channel-drain debounce (was 16 ms sleep)
//! - `EnqueueTreeViewEmit.rs` (in `RPC/CocoonService/TreeView/`) - batched
//!   `sky://tree-view/create` emit with `views: [...]` payload
//!
//! ## Embedder call pattern
//!
//! ```ignore
//! use Vine::Server::{Constants, SpawnBindTask, ValidateSocketAddress};
//!
//! let Address = ValidateSocketAddress::Fn("[::1]:50051", "MountainService")?;
//! let Service = MyMountainServiceImpl::new(state);
//! let Wrapped = MountainServiceServer::new(Service)
//!     .max_decoding_message_size(Constants::MAX_MESSAGE_SIZE)
//!     .max_encoding_message_size(Constants::MAX_MESSAGE_SIZE);
//! let Router  = tonic::transport::Server::builder().add_service(Wrapped);
//! SpawnBindTask::Fn("MountainService".to_string(), Address, Router);
//! ```

pub mod Constants;

pub mod Notification;

pub mod SpawnBindTask;

pub mod SpawnBindTaskWithShutdown;

pub mod ValidateSocketAddress;
