//! Whether the Vine client has been marked shutting down.

use crate::Client::Shared;

/// Whether the Vine client has been marked shutting down.
pub fn Fn() -> bool { Shared::ShutdownFlagLoad() }
