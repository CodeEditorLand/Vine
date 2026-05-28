//! Default configuration constants for Vine gRPC servers.
//!
//! Canonical home for the values every Vine bind site shares: default
//! ports, connection timeout, max concurrent connection budget, max message
//! size. Consumers (Mountain's `MountainVinegRPCService`, Air's
//! `AirVinegRPCService`, any Rust-side Cocoon client) reference these
//! constants instead of redefining them.

use std::time::Duration;

/// Default port for the MountainService gRPC server.
///
/// Mountain binds this port and listens for connections from Cocoon and Air.
pub const DEFAULT_MOUNTAIN_PORT:u16 = 50051;

/// Default port for the CocoonService gRPC server.
///
/// Cocoon binds this port; Mountain connects as a client.
pub const DEFAULT_COCOON_PORT:u16 = 50052;

/// Default port for the AirService gRPC server.
///
/// Air binds this port; Mountain (and external clients) connect as clients.
pub const DEFAULT_AIR_PORT:u16 = 50053;

/// Maximum concurrent connections per server. Tonic does not directly enforce
/// this today; the constant is exposed so the embedder can apply it via
/// `tower::limit::ConcurrencyLimitLayer` if needed.
pub const MAX_CONNECTIONS:usize = 100;

/// Default connection timeout used by the bind helper when configuring the
/// underlying tonic Server.
pub const CONNECTION_TIMEOUT:Duration = Duration::from_secs(30);

/// Default message size limit (4 MB).
///
/// Mirrors tonic's own default and the value enforced on the client side
/// via `crate::Client::Shared::MAX_MESSAGE_SIZE_BYTES`. Apply this to every
/// service wrapper via `.max_decoding_message_size()` /
/// `.max_encoding_message_size()` before `add_service` so unary calls fail
/// fast for oversized payloads.
pub const MAX_MESSAGE_SIZE:usize = 4 * 1024 * 1024;
