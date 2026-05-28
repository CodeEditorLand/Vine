//! # Vine::Server::Notification
//!
//! Cocoon → Mountain notification handlers with non-trivial logic.
//! Pure relay atoms (single RelayToSky::Fn call, single EmitToRenderer,
//! or single dev_log!) are inlined directly in the Mountain dispatcher
//! rather than given their own file.
//!
//! Every handler here has at least one of:
//! - Payload reshape / field extraction
//! - Channel-drain coalescer (OnceLock flusher)
//! - Multiple side effects
//! - Non-obvious sky-event mapping

pub mod Support;

// --- Output: coalescer + payload-reshape ---

pub mod OutputAppendLine;

pub mod OutputChannelAppend;

pub mod OutputChannelCoalesce;

pub mod OutputChannelHide;

pub mod OutputReplace;

// --- Progress: channel-drain coalescer + payload reshape ---

pub mod ProgressEnd;

pub mod ProgressReport;

pub mod ProgressStart;

// --- Webview ---

pub mod WebviewDispose;

pub mod WebviewPostMessage;

pub mod WebviewLifecycle;

// --- Window / Workspace ---

pub mod WindowShowMessage;

// --- Decoration batching ---

pub mod DecorationTypeLifecycle;

// --- Misc with logic ---

pub mod OpenExternal;

pub mod SecurityIncident;

// --- StatusBar ---

pub mod StatusBarLifecycle;

pub mod StatusBarMessage;

pub mod SetStatusBarText;

pub mod DisposeStatusBarItem;

// --- Debug ---

pub mod DebugLifecycle;

// --- Editor text mutations ---

pub mod ApplyTextEdits;

pub mod SetTextEditorDecorations;

// --- Language configuration ---

pub mod SetLanguageConfiguration;

// --- Command registry ---

pub mod RegisterCommand;

pub mod UnregisterCommand;

// --- Provider unregistration (multi-step logic only) ---

pub mod UnregisterScmProvider;

// --- Terminal ---

pub mod TerminalLifecycle;

pub mod WindowCreateTerminal;

// --- SCM registration ---

pub mod RegisterScmProvider;

pub mod RegisterScmResourceGroup;

// --- Language-feature provider registration ---

pub mod RegisterLanguageProvider;

// --- SCM group update ---

pub mod UpdateScmGroup;
