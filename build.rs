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
//! `tonic_prost_build`. Mirrors Mountain's invocation so both crates produce
//! bit-identical generated output during the additive overlap phase.
//!
//! ## Source of truth
//!
//! `Mountain/Proto/Vine.proto` is the canonical schema today. This crate
//! ships its own authoritative copy at `Element/Vine/Proto/Vine.proto`. The
//! migration phase (see `.hermes/plan/Mountain-Crate-Split.md` task #1
//! Phase 3) consolidates the two copies once Cocoon's TS regen and the
//! Mountain shim are wired.

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
