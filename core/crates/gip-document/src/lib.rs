//! # gip-document — the `.gip` document model
//!
//! Single source of truth for user content: the node tree, design data on nodes, pages, and
//! document metadata. Contract: `docs/architecture/document-tree.md`; format: ADR-003;
//! paths: ADR-006.
//!
//! Binding laws for this crate: CAN-SER (serialization purity and determinism) · SM1/SM3
//! (state domains; document mutations only through transactions) · G7 (determinism) ·
//! CAN-FRAME-1 (frame vs group are different types, never interchangeable).
//!
//! **Status:** identifiers, transform fields, node/page/document data, the path data model,
//! the `.gip` v1 container, validation, and canonical serialization. Not yet: the transaction
//! API, observation, shared collections, asset bytes, `documentHash`, NFC normalization.

#![forbid(unsafe_code)]

pub mod document;
pub mod edit;
pub mod error;
pub mod file;
pub mod id;
pub mod node;
pub mod path;
pub mod transform;

#[cfg(test)]
mod serial_tests;

pub use document::{Document, Page};
pub use error::GipError;
pub use file::{AssetRecord, CURRENT_VERSION, FORMAT_TAG, GipFile, Meta, from_json, to_json};
pub use id::{AssetId, NodeId};
pub use node::{FieldKey, FieldValue, Node, NodeKind};
pub use path::{FillRule, HandleMode, Path, PointType, Subpath, Vertex};
pub use transform::Transform;
