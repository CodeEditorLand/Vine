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
//! - The function name mirrors the file name verbatim so a grep for `fn <Name>`
//!   lands in exactly one place.
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
//!   application state via `Host.ApplicationState()` and downcasting to the
//!   embedder-specific sub-trait.
//! - `Parameter` is the raw JSON payload Cocoon sent; each atom extracts the
//!   fields it needs and validates locally.
//! - Return `()` - atoms that need to fail just log via `dev_log!`; the caller
//!   always returns `Empty` to Cocoon because notifications are
//!   fire-and-forget.

pub mod Support;

// --- Pure renderer-event relays (Output / Progress fan-out) ---

pub mod OutputAppend;

pub mod OutputAppendLine;

pub mod OutputChannelAppend;

pub mod OutputChannelClear;

pub mod OutputChannelCoalesce;

pub mod OutputChannelCreate;

pub mod OutputChannelDispose;

pub mod OutputChannelHide;

pub mod OutputChannelReplace;

pub mod OutputChannelShow;

pub mod OutputClear;

pub mod OutputCreate;

pub mod OutputDispose;

pub mod OutputReplace;

pub mod OutputShow;

pub mod ProgressComplete;

pub mod ProgressEnd;

pub mod ProgressReport;

pub mod ProgressStart;

pub mod ProgressUpdate;

// --- Webview lifecycle ---

pub mod WebviewDispose;

pub mod WebviewPostMessage;

pub mod WebviewReady;

// --- Window / Workspace renderer events ---

pub mod WindowShowMessage;

pub mod WindowShowTextDocument;

pub mod WorkspaceApplyEdit;

// --- Decoration batching ---

pub mod DecorationTypeLifecycle;

// --- Extension lifecycle events ---

pub mod ExtensionActivated;

pub mod ExtensionDeactivated;

// --- Tree view refresh ---

pub mod TreeRefresh;

// --- Misc renderer relays ---

pub mod OpenExternal;

pub mod SecurityIncident;

// --- StatusBar lifecycle ---

pub mod StatusBarLifecycle;

pub mod StatusBarMessage;

pub mod SetStatusBarText;

pub mod DisposeStatusBarItem;

// --- Debug lifecycle ---

pub mod DebugLifecycle;

// --- Editor text mutations ---

pub mod ApplyTextEdits;

pub mod SetTextEditorDecorations;

// --- Extension host protocol ---

pub mod ExtensionHostMessage;

// --- Language configuration ---

pub mod SetLanguageConfiguration;

pub mod LanguagesSetDocumentLanguage;

// --- Command registry ---

pub mod RegisterCommand;

pub mod UnregisterCommand;

// --- Provider unregistration ---

pub mod UnregisterAuthenticationProvider;

pub mod UnregisterDebugAdapter;

pub mod UnregisterDebugConfigurationProvider;

pub mod UnregisterExternalUriOpener;

pub mod UnregisterFileSystemProvider;
