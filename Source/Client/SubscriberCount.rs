//! Number of currently-active broadcast subscribers. Diagnostic; useful
//! for validating that subscribers haven't leaked.

use crate::Client::Shared;

/// Number of currently-active broadcast subscribers. Diagnostic; useful for validating that subscribers haven't leaked.
///
pub fn Fn() -> usize { Shared::NOTIFICATION_BROADCAST.receiver_count() }
