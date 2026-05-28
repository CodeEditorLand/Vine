//! # Vine::Server::Notification
//!
//! Cocoon → Mountain notification handlers, one entry-point per file.
//! Each atom encapsulates exactly one wire-method's side effects so the
//! main `send_cocoon_notification` dispatcher in the embedder's
//! `MountainServiceServer` impl stays a thin match that routes into these
//! files.
//!
//! ## Naming convention
//!
//! - Wire string `outputChannel.create` → atom file `OutputChannelCreate.rs`
//!   with `pub async fn OutputChannelCreate(...)`.
//! - Snake_case / dotted wire strings collapse to PascalCase file names.
//! - The function name mirrors the file name verbatim so a grep for
//!   `fn <Name>` lands in exactly one place.
//!
//! ## Signature contract
//!
//! Every atom takes the same two parameters:
//!
//! ```ignore
//! pub async fn <Atom>(
//!     Host: &dyn crate::Host::VineHost,
//!     Parameter: &serde_json::Value,
//! );
//! ```
//!
//! - `Host` is the embedder seam. For pure renderer-event relays it gives
//!   access to `EmitToRenderer`; richer handlers reach into the embedder's
//!   application state via `Host.ApplicationState()` and downcasting to
//!   the embedder-specific sub-trait.
//! - `Parameter` is the raw JSON payload Cocoon sent; each atom extracts
//!   the fields it needs and validates locally.
//! - Return `()` - atoms that need to fail just log via `dev_log!`; the
//!   caller always returns `Empty` to Cocoon because notifications are
//!   fire-and-forget.

pub mod Support;

// --- Pure renderer-event relays ---

pub mod OutputAppend;

pub mod OutputChannelClear;

pub mod OutputChannelDispose;

pub mod OutputChannelShow;

pub mod OutputClear;

pub mod OutputCreate;

pub mod OutputDispose;

pub mod OutputShow;

pub mod ProgressComplete;
