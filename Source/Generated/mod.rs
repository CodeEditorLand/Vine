//! # Vine::Generated
//!
//! `tonic_prost_build` output for `Proto/Vine.proto`. Do not edit
//! `vine.rs` by hand - it is regenerated on every build by `build.rs` at
//! the crate root.
//!
//! ## Message Types
//!
//! - `Empty` - placeholder for RPCs with no return value
//! - `GenericRequest` - request with ID, method, JSON parameters
//! - `GenericResponse` - response with ID, result, or error
//! - `GenericNotification` - fire-and-forget notification
//! - `RpcError` - JSON-RPC compliant error structure
//! - `CancelOperationRequest` - request to cancel an in-flight operation
//! - `RpcDataPayload` - generic data payload (future extension point)
//!
//! ## Service Clients
//!
//! - `cocoon_service_client::CocoonServiceClient`
//! - `mountain_service_client::MountainServiceClient`
//!
//! ## Service Servers
//!
//! - `cocoon_service_server::{CocoonService, CocoonServiceServer}`
//! - `mountain_service_server::{MountainService, MountainServiceServer}`

#![allow(
	clippy::all,

	unused_imports,

	unknown_lints,

	non_shorthand_field_patterns,

	non_snake_case,

	non_camel_case_types,

	non_upper_case_globals,

	dead_code,

	unused_variables,

	unused_assignments
)]

include!("vine.rs");
