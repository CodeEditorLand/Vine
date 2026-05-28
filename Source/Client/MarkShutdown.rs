//! Flip the global Vine-client shutdown flag. Embedders call this
//! immediately before SIGKILL'ing the sidecar so any inflight notification
//! attempted after the kill window returns silently with `Ok(())` instead
//! of logging a `Connection refused` error.

use crate::Client::Shared;

pub fn Fn() { Shared::ShutdownFlagStore(true); }
