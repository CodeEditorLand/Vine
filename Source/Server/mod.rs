//! # Vine::Server
//!
//! Server-side gRPC scaffolding shared by every embedder that hosts a Vine
//! service: Mountain (MountainService), Air (AirService), and Rust-side
//! extension host services. The pieces below are the boilerplate every bind
//! site needs; concrete service implementations stay in their owning crate.
//!
//! ## Modules
//!
//! - [`Constants`] — default ports / timeouts / message-size cap.
//! - [`ValidateSocketAddress`] — port-and-format pre-flight check.
//! - [`SpawnBindTask`] — detached `tokio::spawn` that runs
//!   `Router::serve(Address)` until process termination.
//! - [`SpawnBindTaskWithShutdown`] — same shape but takes a shutdown future so
//!   daemons can drain in-flight calls before exit.
//! - [`Notification`] — one-entry-point-per-file extension host → Mountain
//!   notification handlers dispatched against [`crate::Host::VineHost`].
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
