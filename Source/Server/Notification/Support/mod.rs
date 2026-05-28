//! # Vine::Server::Notification::Support
//!
//! Reusable helpers shared by multiple notification atoms. Currently:
//!
//! - [`RelayToSky::Fn`] - collapse the `host.EmitToRenderer(SkyEvent,
//!   Parameter); dev_log!(tag, line);` pair that ~25 % of the notification
//!   atoms use into a one-liner call.
//!
//! Synthesised from `Mountain/Source/Vine/Server/Notification/Support/`.

pub mod RelayToSky;
