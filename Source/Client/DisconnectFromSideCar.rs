//! Disconnect from a sidecar process. Removes the entry from both the
//! connection pool and the metadata tracker.

use crate::{
	Client::Shared::{CONNECTION_METADATA, SIDECAR_CLIENTS},
	Error::VineError,
	dev_log,
};

/// Disconnect from a sidecar process. Removes the entry from both the
/// connection pool and the metadata tracker.
pub fn Fn(SideCarIdentifier:String) -> Result<(), VineError> {
	if SIDECAR_CLIENTS.remove(&SideCarIdentifier).is_some() {
		CONNECTION_METADATA.lock().remove(&SideCarIdentifier);

		dev_log!("grpc", "[VineClient] Disconnected from sidecar '{}'", SideCarIdentifier);

		Ok(())
	} else {
		Err(VineError::ClientNotConnected(SideCarIdentifier))
	}
}
