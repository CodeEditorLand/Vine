//! # Vine::Client
//!
//! Client-side gRPC wrappers for embedders that need to *call* Vine
//! services (Air talking to Mountain, Cocoon-Rust talking to Mountain, …).
//! One entry-point per file:
//!
//! - [`MarkShutdown::Fn`] / [`IsShuttingDown::Fn`] - process-wide flag.
//! - [`NotificationFrame::Struct`] - broadcast payload.
//! - [`SubscribeNotifications::Fn`] / [`SubscriberCount::Fn`] - fan-out access.
//! - [`ConnectToSideCar::Fn`] / [`DisconnectFromSideCar::Fn`] - pool lifecycle.
//!   Driven by [`TryConnectSingle::Fn`] (single attempt).
//! - [`IsClientConnected::Fn`] / [`WaitForClientConnection::Fn`] -
//!   boot-race-friendly readiness checks.
//! - [`CheckSideCarHealth::Fn`] - pool + metadata health summary.
//! - [`SendRequest::Fn`] / [`SendNotification::Fn`] - wire dispatch with
//!   optional streaming-multiplexer fast path under `LAND_VINE_STREAMING=1`
//!   when the `multiplexer` cargo feature is enabled.
//! - `PublishNotification::Fn` (private) and `PublishNotificationFromMux::Fn`
//!   (`pub(crate)`) - broadcast publishers.
//! - [`Shared`] - module-private state (statics, helpers, constants).
//!
//! ## Behaviours
//!
//! - 3-attempt exponential backoff (50 ms / 100 ms / 200 ms base schedule) in
//!   [`ConnectToSideCar::Fn`].
//! - `Arc<Notify>` connection-ready wake-up (no polling) in
//!   [`WaitForClientConnection::Fn`].
//! - 5 000 ms unary default timeout, bounded by
//!   [`crate::DefaultRequestTimeoutMs`] at 15 000 ms; per-call override on
//!   [`SendRequest::Fn`].
//! - Atomic-counter request IDs (`AtomicU64`, no `SystemTime` syscall) in
//!   [`SendRequest::Fn`].
//! - h2 transport tuning (4 MB stream / 16 MB connection windows;
//!   `LAND_TONIC_TUNED=0` to disable) in [`TryConnectSingle::Fn`].

pub mod CheckSideCarHealth;

pub mod ConnectToSideCar;

pub mod DisconnectFromSideCar;

pub mod IsClientConnected;

pub mod IsShuttingDown;

pub mod MarkShutdown;

pub mod NotificationFrame;

pub mod SendNotification;

pub mod SendRequest;

pub mod SubscribeNotifications;

pub mod SubscriberCount;

pub mod TryConnectSingle;

pub mod WaitForClientConnection;

pub(crate) mod PublishNotification;

pub mod Shared;
