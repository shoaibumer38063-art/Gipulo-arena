//! # gip-history — transactions and undo/redo
//!
//! The sole mutation path for the document model: transactional, intent-labeled, undoable.
//! Two history laws are absolute: **one user intent = one entry** (a whole drag is one undo)
//! and **transient state never enters history** (CAN-HIST-1/2).
//! Contracts: `docs/architecture/undo-redo.md` and ADR-004 (op log, replay of recorded values).
//!
//! The TS shell has NO document write API: it sends typed intents that arrive here
//! (engine-core.md §6).
//!
//! **Status:** transaction collector (`tx`) and the stack with byte budget, redo truncation, and
//! dirty tracking (`history`). The op vocabulary and `apply` live in `gip_document::edit` (the write
//! door, ADR-004 A6). Not yet: text-session coalescing (500 ms window), gesture sessions, and
//! subscriptions.

#![forbid(unsafe_code)]

pub mod history;
pub mod tx;

pub use gip_document::edit::{EditOp, Parent, Subtree};
pub use history::{Changes, DEFAULT_BUDGET_BYTES, Entry, History, TxError};
pub use tx::Tx;

/// Human-meaningful intent label carried by every transaction entry.
///
/// Used for UX copy (undo menu), accessibility announcements (CAN-A11Y-2 coalescing), and
/// deterministic test assertions. Labels are data, not formatting: no locale-dependent text.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct IntentLabel(String);

impl IntentLabel {
    /// Creates a label (e.g. `"move"`, `"resize"`, `"add-layer"`).
    #[must_use]
    pub fn new(label: impl Into<String>) -> Self {
        Self(label.into())
    }

    /// Returns the label text.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}
