//! Whether the Vine client has been marked shutting down.

use crate::Client::Shared;

pub fn Fn() -> bool { Shared::ShutdownFlagLoad() }
