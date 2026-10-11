//! Load/save errors. Boundary paths return these as stable codes (engine-core.md §8); messages
//! are diagnostics, never user-facing copy.

use std::fmt;

/// Every way a `.gip` load or save can fail.
#[derive(Debug, Clone, PartialEq)]
pub enum GipError {
    /// The text is not valid JSON for the schema (includes unknown kinds, bad id encodings).
    Json(String),
    /// The `format` tag is not `gip`.
    UnsupportedFormat(String),
    /// The file's version is newer than this build understands (no down-migrations, ADR-003 #8).
    UnsupportedVersion(u32),
    /// A structural or value invariant is violated.
    Invalid(String),
}

impl fmt::Display for GipError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            GipError::Json(msg) => write!(f, "gip: malformed document: {msg}"),
            GipError::UnsupportedFormat(tag) => write!(f, "gip: not a .gip file (format {tag:?})"),
            GipError::UnsupportedVersion(v) => {
                write!(f, "gip: file version {v} is newer than this build")
            }
            GipError::Invalid(msg) => write!(f, "gip: invalid document: {msg}"),
        }
    }
}

impl std::error::Error for GipError {}

impl From<serde_json::Error> for GipError {
    fn from(e: serde_json::Error) -> Self {
        GipError::Json(e.to_string())
    }
}
