#![allow(
	non_snake_case,
	non_camel_case_types,
	non_upper_case_globals,
	dead_code,
	unused_imports,
	unused_variables,
	unused_assignments
)]

//! # Vine build.rs
//!
//! Compiles `Proto/Vine.proto` into `Source/Generated/vine.rs` via
//! `tonic_prost_build`. Generates both client and server stubs and enables
//! well-known-types compilation.

fn main() -> Result<(), Box<dyn std::error::Error>> {
	println!("cargo:rerun-if-changed=Cargo.toml");

	println!("cargo:rerun-if-changed=Proto/Vine.proto");

	tonic_prost_build::configure()
		.build_server(true)
		.build_client(true)
		.out_dir("Source/Generated")
		.compile_well_known_types(true)
		.compile_protos(&["Proto/Vine.proto"], &["Proto"])?;

	Ok(())
}
