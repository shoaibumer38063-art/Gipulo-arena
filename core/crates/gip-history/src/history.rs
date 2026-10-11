//! The history stack (ADR-004): entries, transact, undo/redo, byte budget, and dirty tracking.
//! Only the document domain is historical; selection, camera, hover, and previews never reach
//! this module (CAN-HIST-2).

use std::collections::{BTreeSet, VecDeque};
use std::fmt;

use gip_document::edit::{Dir, EditOp, affected_ids, apply, op_bytes};
use gip_document::{Document, GipError, GipFile, NodeId};

use crate::IntentLabel;
use crate::tx::Tx;

/// Default undo byte budget (ADR-004 #9: 64 MiB, [PROPOSED] — tuned at measurement).
pub const DEFAULT_BUDGET_BYTES: usize = 64 * 1024 * 1024;

/// One user intent: its label and its ordered ops.
#[derive(Debug, Clone)]
pub struct Entry {
    intent: IntentLabel,
    ops: Vec<EditOp>,
    bytes: usize,
}

impl Entry {
    fn new(intent: IntentLabel, ops: Vec<EditOp>) -> Self {
        let bytes = 64 + ops.iter().map(op_bytes).sum::<usize>();
        Self { intent, ops, bytes }
    }

    /// The intent label.
    #[must_use]
    pub fn intent(&self) -> &IntentLabel {
        &self.intent
    }

    /// Union of node ids touched by the entry's ops.
    #[must_use]
    pub fn affected(&self) -> BTreeSet<NodeId> {
        let mut out = BTreeSet::new();
        for op in &self.ops {
            affected_ids(op, &mut out);
        }
        out
    }
}

/// Why a transaction or history step did not complete.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TxError {
    /// The mutator returned an error. Nothing changed.
    Rejected(String),
    /// The result violates a document invariant. Nothing changed.
    Invalid(String),
    /// Replay did not match the document. Indicates an engine bug, not a user error.
    Internal(String),
}

impl fmt::Display for TxError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TxError::Rejected(m) => write!(f, "transaction rejected: {m}"),
            TxError::Invalid(m) => write!(f, "transaction invalid: {m}"),
            TxError::Internal(m) => write!(f, "history internal error: {m}"),
        }
    }
}

impl std::error::Error for TxError {}

/// What a successful commit, undo, or redo changed.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Changes {
    /// Node ids whose state changed (for cache and index invalidation, ADR-004 #12).
    pub affected: BTreeSet<NodeId>,
    /// True when a new entry was recorded (false for no-op transactions).
    pub recorded: bool,
}

/// The document plus its undo/redo stacks.
#[derive(Debug, Clone)]
pub struct History {
    file: GipFile,
    undo: VecDeque<Entry>,
    redo: Vec<Entry>,
    bytes: usize,
    budget: usize,
    version: u64,
    saved_version: u64,
}

impl History {
    /// Starts a history over a validated file with the default budget.
    ///
    /// # Errors
    /// Returns the file's validation error when it is not a valid document.
    pub fn new(file: GipFile) -> Result<Self, GipError> {
        Self::with_budget(file, DEFAULT_BUDGET_BYTES)
    }

    /// Starts a history with an explicit undo byte budget.
    ///
    /// # Errors
    /// Returns the file's validation error when it is not a valid document.
    pub fn with_budget(file: GipFile, budget: usize) -> Result<Self, GipError> {
        file.validate()?;
        Ok(Self {
            file,
            undo: VecDeque::new(),
            redo: Vec::new(),
            bytes: 0,
            budget,
            version: 0,
            saved_version: 0,
        })
    }

    /// The whole file as it currently is.
    #[must_use]
    pub fn file(&self) -> &GipFile {
        &self.file
    }

    /// The document as it currently is.
    #[must_use]
    pub fn document(&self) -> &Document {
        self.file.document()
    }

    /// Consumes the history and returns the file (history itself is not persisted, ADR-004 #11).
    #[must_use]
    pub fn into_file(self) -> GipFile {
        self.file
    }

    /// Runs one intent. The mutator's writes are applied and recorded; on error or an
    /// invariant violation every write is rolled back and no entry is recorded.
    ///
    /// # Errors
    /// [`TxError::Rejected`] when the mutator fails, [`TxError::Invalid`] when the result would
    /// violate a document invariant, [`TxError::Internal`] on rollback failure.
    pub fn transact<T>(
        &mut self,
        intent: IntentLabel,
        mutate: impl FnOnce(&mut Tx<'_>) -> Result<T, String>,
    ) -> Result<(T, Changes), TxError> {
        let mut tx = Tx {
            file: &mut self.file,
            ops: Vec::new(),
        };
        let outcome = mutate(&mut tx);
        let ops = std::mem::take(&mut tx.ops);
        let value = match outcome {
            Ok(value) => value,
            Err(message) => {
                rollback(&mut self.file, &ops)?;
                return Err(TxError::Rejected(message));
            }
        };
        if let Err(e) = self.file.validate() {
            rollback(&mut self.file, &ops)?;
            return Err(TxError::Invalid(e.to_string()));
        }
        if ops.is_empty() {
            return Ok((value, Changes::default()));
        }
        let entry = Entry::new(intent, ops);
        let affected = entry.affected();
        self.bytes -= self.redo.iter().map(|e| e.bytes).sum::<usize>();
        self.redo.clear();
        self.bytes += entry.bytes;
        self.undo.push_back(entry);
        self.enforce_budget();
        self.version += 1;
        Ok((
            value,
            Changes {
                affected,
                recorded: true,
            },
        ))
    }

    /// Undoes the most recent entry. `Ok(None)` when there is nothing to undo.
    ///
    /// # Errors
    /// [`TxError::Internal`] when replay does not match the document.
    pub fn undo(&mut self) -> Result<Option<Changes>, TxError> {
        let Some(entry) = self.undo.pop_back() else {
            return Ok(None);
        };
        for op in entry.ops.iter().rev() {
            apply(&mut self.file, op, Dir::Backward).map_err(TxError::Internal)?;
        }
        let affected = entry.affected();
        self.redo.push(entry);
        self.version += 1;
        Ok(Some(Changes {
            affected,
            recorded: false,
        }))
    }

    /// Redoes the most recently undone entry. `Ok(None)` when there is nothing to redo.
    ///
    /// # Errors
    /// [`TxError::Internal`] when replay does not match the document.
    pub fn redo(&mut self) -> Result<Option<Changes>, TxError> {
        let Some(entry) = self.redo.pop() else {
            return Ok(None);
        };
        for op in &entry.ops {
            apply(&mut self.file, op, Dir::Forward).map_err(TxError::Internal)?;
        }
        let affected = entry.affected();
        self.undo.push_back(entry);
        self.version += 1;
        Ok(Some(Changes {
            affected,
            recorded: false,
        }))
    }

    /// Whether `undo` would do anything.
    #[must_use]
    pub fn can_undo(&self) -> bool {
        !self.undo.is_empty()
    }

    /// Whether `redo` would do anything.
    #[must_use]
    pub fn can_redo(&self) -> bool {
        !self.redo.is_empty()
    }

    /// Label of the entry `undo` would revert (for the undo menu and announcements).
    #[must_use]
    pub fn undo_label(&self) -> Option<&IntentLabel> {
        self.undo.back().map(Entry::intent)
    }

    /// Label of the entry `redo` would re-apply.
    #[must_use]
    pub fn redo_label(&self) -> Option<&IntentLabel> {
        self.redo.last().map(Entry::intent)
    }

    /// Marks the current state as saved.
    pub fn mark_saved(&mut self) {
        self.saved_version = self.version;
    }

    /// Whether the document differs from the last saved state (by history position, not by hash).
    #[must_use]
    pub fn is_dirty(&self) -> bool {
        self.version != self.saved_version
    }

    /// Drops the oldest undo entries until the byte budget holds. The newest entry is kept.
    fn enforce_budget(&mut self) {
        while self.bytes > self.budget && self.undo.len() > 1 {
            if let Some(old) = self.undo.pop_front() {
                self.bytes -= old.bytes;
            }
        }
    }
}

/// Reverts recorded ops in reverse order (atomic failure path).
fn rollback(file: &mut GipFile, ops: &[EditOp]) -> Result<(), TxError> {
    for op in ops.iter().rev() {
        apply(file, op, Dir::Backward).map_err(TxError::Internal)?;
    }
    Ok(())
}
