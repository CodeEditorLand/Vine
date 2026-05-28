//! # Vine::Error
//!
//! Canonical, structured error types for every operation that flows through
//! Vine - the gRPC IPC layer that connects Mountain, Cocoon, and Air.
//!
//! Synthesised from `Mountain/Source/Vine/Error.rs` per Track-B task #1 of
//! `.hermes/plan/Mountain-Crate-Split.md`. The variant set + `From` impls
//! match Mountain bit-for-bit so consumers compile unchanged when they later
//! switch from Mountain's in-tree copy to this crate.
//!
//! ## Error Categories
//!
//! ### Connection Errors
//! - `ClientNotConnected`: Sidecar not in connection pool
//! - `ConnectionFailed`: Unable to establish connection
//! - `ConnectionLost`: Established connection was lost
//!
//! ### RPC Errors
//! - `RPCError`: Generic gRPC status error
//! - `RequestTimeout`: Request exceeded configured timeout
//! - `RequestCanceled`: Request was explicitly canceled
//!
//! ### Serialization Errors
//! - `SerializationError`: JSON serialization/deserialization failure
//! - `MessageTooLarge`: Message exceeds size limits
//! - `InvalidMessageFormat`: Message format validation failed
//!
//! ### Transport Errors
//! - `TonicTransportError`: Low-level tonic transport failure
//! - `InvalidUri`: Invalid URI format
//! - `AddressParseError`: Invalid socket address format
//!
//! ### Internal Errors
//! - `InternalLockError`: Mutex poisoned (panic in another thread)
//! - `InvalidState`: Invalid internal state detected

use std::{
	net::AddrParseError,
	sync::{MutexGuard, PoisonError},
};

use http::uri::InvalidUri;
use thiserror::Error;

/// A comprehensive error enum for the Vine IPC layer.
///
/// Each variant carries detailed context so callers can choose between retry,
/// fallback, and surface-to-user strategies.
#[derive(Debug, Error)]
pub enum VineError {
	/// A gRPC client channel for the specified sidecar could not be found or
	/// is not ready in the connection pool.
	#[error("SideCar '{0}' not found or its gRPC client channel is not ready.")]
	ClientNotConnected(String),

	/// Failed to establish a connection to the specified sidecar.
	#[error("Failed to connect to sidecar '{SideCarIdentifier}' at '{Address}': {Reason}")]
	ConnectionFailed { SideCarIdentifier:String, Address:String, Reason:String },

	/// An established connection to the sidecar was lost.
	#[error("Connection to sidecar '{0}' was lost")]
	ConnectionLost(String),

	/// An RPC call to a sidecar failed with a specific gRPC status.
	#[error("gRPC call failed: {0}")]
	RPCError(String),

	/// A request did not receive a response within the configured timeout.
	#[error(
		"Request to sidecar '{SideCarIdentifier}' (method: '{MethodName}') timed out after {TimeoutMilliseconds}ms"
	)]
	RequestTimeout { SideCarIdentifier:String, MethodName:String, TimeoutMilliseconds:u64 },

	/// A request was explicitly cancelled before completion.
	#[error("Request to sidecar '{SideCarIdentifier}' (method: '{MethodName}') was canceled")]
	RequestCanceled { SideCarIdentifier:String, MethodName:String },

	/// An error occurred while serializing or deserializing a JSON payload.
	#[error("JSON serialization error for gRPC payload: {0}")]
	SerializationError(#[from] serde_json::Error),

	/// Message exceeded the maximum allowed size.
	#[error("Message size {ActualSize} bytes exceeds maximum allowed size {MaxSize} bytes")]
	MessageTooLarge { ActualSize:usize, MaxSize:usize },

	/// Message format validation failed.
	#[error("Invalid message format: {0}")]
	InvalidMessageFormat(String),

	/// A low-level error occurred in the `tonic` gRPC transport layer.
	#[error("Tonic transport error: {0}")]
	TonicTransportError(#[from] tonic::transport::Error),

	/// A shared state mutex was "poisoned," indicating a panic in another
	/// thread while holding the lock.
	#[error("Internal state lock poisoned: {0}")]
	InternalLockError(String),

	/// Invalid internal state detected - the system reached an unexpected
	/// state that should never happen during normal operation.
	#[error("Invalid internal state detected: {0}")]
	InvalidState(String),

	/// An error occurred from an invalid URI.
	#[error("Invalid URI: {0}")]
	InvalidUri(#[from] InvalidUri),

	/// An error occurred while parsing a socket address.
	#[error("Invalid Socket Address: {0}")]
	AddressParseError(#[from] AddrParseError),
}

impl VineError {
	/// Returns `true` when the error is recoverable (the caller can sensibly
	/// retry the operation).
	pub fn IsRecoverable(&self) -> bool {
		matches!(
			self,
			Self::RequestTimeout { .. }
				| Self::ConnectionFailed { .. }
				| Self::ConnectionLost(_)
				| Self::TonicTransportError(_)
		)
	}

	/// Maps the error to a `tonic::Status` suitable for a gRPC error response.
	pub fn ToTonicStatus(&self) -> tonic::Status {
		match self {
			Self::RequestTimeout { .. } => tonic::Status::deadline_exceeded(self.to_string()),

			Self::ClientNotConnected(_) | Self::ConnectionFailed { .. } => tonic::Status::unavailable(self.to_string()),

			Self::SerializationError(_) | Self::InternalLockError(_) | Self::InvalidState(_) => {
				tonic::Status::internal(self.to_string())
			},

			Self::MessageTooLarge { .. } => tonic::Status::resource_exhausted(self.to_string()),

			Self::InvalidMessageFormat(_) | Self::InvalidUri(_) | Self::AddressParseError(_) => {
				tonic::Status::invalid_argument(self.to_string())
			},

			Self::RequestCanceled { .. } => tonic::Status::cancelled(self.to_string()),

			Self::RPCError(msg) => tonic::Status::unknown(msg.clone()),

			Self::ConnectionLost(_) => tonic::Status::aborted(self.to_string()),

			Self::TonicTransportError(_) => tonic::Status::unavailable(self.to_string()),
		}
	}
}

impl<T> From<PoisonError<MutexGuard<'_, T>>> for VineError {
	fn from(Error:PoisonError<MutexGuard<'_, T>>) -> Self {
		VineError::InternalLockError(format!("Shared state lock poisoned: {}", Error))
	}
}

impl From<tonic::Status> for VineError {
	fn from(Status:tonic::Status) -> Self {
		match Status.code() {
			tonic::Code::DeadlineExceeded => VineError::RPCError(format!("Timeout: {}", Status.message())),

			tonic::Code::NotFound => VineError::ClientNotConnected(Status.message().to_string()),

			tonic::Code::AlreadyExists | tonic::Code::InvalidArgument | tonic::Code::OutOfRange => {
				VineError::InvalidMessageFormat(Status.message().to_string())
			},

			tonic::Code::FailedPrecondition | tonic::Code::Aborted => {
				VineError::ConnectionLost(Status.message().to_string())
			},

			tonic::Code::ResourceExhausted => VineError::MessageTooLarge { ActualSize:0, MaxSize:4 * 1024 * 1024 },

			tonic::Code::Cancelled => {
				VineError::RequestCanceled { SideCarIdentifier:"unknown".to_string(), MethodName:"unknown".to_string() }
			},

			tonic::Code::Unavailable => {
				VineError::ConnectionFailed {
					SideCarIdentifier:"unknown".to_string(),
					Address:"unknown".to_string(),
					Reason:Status.message().to_string(),
				}
			},

			_ => VineError::RPCError(Status.to_string()),
		}
	}
}

/// Convenience `Result` alias for Vine operations.
pub type Result<T> = std::result::Result<T, VineError>;
