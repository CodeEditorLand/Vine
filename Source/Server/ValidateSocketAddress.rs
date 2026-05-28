//! Validate a socket address string before binding.
//!
//! Ported verbatim from
//! `Mountain/Source/Vine/Server/Initialize.rs::ValidateSocketAddress`.
//! Catches three classes of error before the embedder spawns the bind task:
//!
//! 1. Empty address strings (operator typo / missing env var).
//! 2. Address strings longer than 256 characters (defensive cap).
//! 3. Addresses that fail `parse::<SocketAddr>()` (malformed).
//!
//! Logs a warning - but does not reject - when the port is in the privileged
//! range (< 1024), since Land's defaults all live above 50 000.

use std::net::SocketAddr;

use crate::{Error::VineError, dev_log};

/// Parses and validates `AddressString` for use as a Vine gRPC bind address.
///
/// # Parameters
///
/// * `AddressString` - the address string to validate (e.g. `"[::1]:50051"`).
/// * `ServerName`    - human-readable name of the server, used in error
///   messages and the privileged-port warning (e.g. `"MountainService"`).
///
/// # Errors
///
/// Returns `VineError::InvalidMessageFormat` for empty or oversize strings,
/// `VineError::AddressParseError` for malformed addresses.
pub fn Fn(AddressString:&str, ServerName:&str) -> Result<SocketAddr, VineError> {
	if AddressString.is_empty() {
		return Err(VineError::InvalidMessageFormat(format!(
			"{} address cannot be empty",
			ServerName
		)));
	}

	if AddressString.len() > 256 {
		return Err(VineError::InvalidMessageFormat(format!(
			"{} address exceeds maximum length (256 characters)",
			ServerName
		)));
	}

	match AddressString.parse::<SocketAddr>() {
		Ok(Address) => {
			if Address.port() < 1024 {
				dev_log!(
					"grpc",
					"warn: [Vine::Server] {} using privileged port {}, this may require elevated privileges",
					ServerName,
					Address.port()
				);
			}

			Ok(Address)
		},

		Err(Error) => Err(VineError::AddressParseError(Error)),
	}
}
