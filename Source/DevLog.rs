//! # Vine::DevLog
//!
//! Tag-gated, debug-only logging macro. Gates on `cfg!(debug_assertions)`
//! and the `LAND_DEV_LOG` environment variable, then emits through the
//! `log` crate so the embedder's configured logger (Air's `env_logger`,
//! Mountain's tracing layer, …) decides where it lands.
//!
//! ## Usage
//!
//! ```ignore
//! use Vine::dev_log;
//!
//! dev_log!("grpc", "Connecting to sidecar '{}' at '{}'", id, addr);
//! ```
//!
//! ## Tag filter
//!
//! `LAND_DEV_LOG` accepts:
//! - unset / empty - no dev-log output (release default).
//! - `all` - every tag emits.
//! - comma-separated tags (e.g. `LAND_DEV_LOG=grpc,vine,boot`) - only the
//!   listed tags emit.
//!
//! The check is `O(1)` for `all` and `O(tag-count)` for the list form.
//! Release builds short-circuit at the `cfg!(debug_assertions)` gate so the
//! environment read never executes.

/// Monotonic process-relative nanosecond timestamp.
///
/// Used by Vine notification fan-out to stamp every frame without paying for
/// a `SystemTime::now()` syscall per call.
pub fn NowNano() -> u64 {
	use std::time::Instant;

	static START:once_cell::sync::Lazy<Instant> = once_cell::sync::Lazy::new(Instant::now);

	START.elapsed().as_nanos() as u64
}

/// Returns `true` when dev-log emission is enabled for `Tag`.
///
/// Reads `LAND_DEV_LOG` on every call. Cheap in practice (single
/// `std::env::var` lookup) and avoids the cache-invalidation problem that a
/// `OnceLock`-backed parse would create when the environment changes after
/// boot (e.g. tests that flip the variable per-case).
pub fn IsEnabled(Tag:&str) -> bool {
	let Filter = match std::env::var("LAND_DEV_LOG") {
		Ok(Value) => Value,

		Err(_) => return false,
	};

	if Filter.is_empty() {
		return false;
	}

	if Filter == "all" || Filter == "1" || Filter == "true" {
		return true;
	}

	Filter.split(',').any(|Entry| Entry.trim().eq_ignore_ascii_case(Tag))
}

/// Tag-gated dev log. Compiled out in release builds via
/// `cfg!(debug_assertions)` short-circuit.
///
/// The body of the macro lives in this crate's `log` crate facade so the
/// embedder's logger decides routing. Mountain's tracing subscriber + Air's
/// `env_logger` both pick this up automatically without further wiring.
#[macro_export]
macro_rules! dev_log {

	($Tag:expr, $($Arg:tt)*) => {{
		if cfg!(debug_assertions) && $crate::DevLog::IsEnabled($Tag) {
			let Message = format!($($Arg)*);

			::log::debug!(target: $Tag, "{}", Message);
		}
	}};
}
