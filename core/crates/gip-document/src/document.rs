//! The document: pages, a flat node table, and the structural invariants that keep the tree one
//! truth (document-tree.md §3; ADR-003 #4/#5). Mutations happen only through transactions
//! (SM3) — this module provides data and validation, not an edit API.

use std::collections::{BTreeMap, BTreeSet};

use serde::{Deserialize, Serialize};

use crate::error::GipError;
use crate::id::NodeId;
use crate::node::{Node, NodeKind};

/// A page: an ordered list of top-level node ids.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Page {
    /// Page identity (allocated from the same id space as nodes).
    pub id: NodeId,
    /// Page name.
    pub name: String,
    /// Top-level nodes in z-order.
    pub children: Vec<NodeId>,
    /// Unrecognized keys preserved verbatim (ADR-003 #9).
    #[serde(flatten, default, skip_serializing_if = "BTreeMap::is_empty")]
    pub ext: BTreeMap<String, serde_json::Value>,
}

/// The document body: pages and the node table keyed by id (sorted → deterministic output).
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Document {
    /// Pages in order. Crate-private: read through [`Document::pages`]; written only via `edit`.
    pub(crate) pages: Vec<Page>,
    /// Every node in the document. Crate-private: read through [`Document::nodes`].
    pub(crate) nodes: BTreeMap<NodeId, Node>,
    /// Unrecognized keys preserved verbatim (ADR-003 #9).
    #[serde(flatten, default, skip_serializing_if = "BTreeMap::is_empty")]
    pub(crate) ext: BTreeMap<String, serde_json::Value>,
}

impl Document {
    /// Pages in order (read-only).
    #[must_use]
    pub fn pages(&self) -> &[Page] {
        &self.pages
    }

    /// Every node in the document, keyed by id (read-only).
    #[must_use]
    pub fn nodes(&self) -> &BTreeMap<NodeId, Node> {
        &self.nodes
    }

    /// Checks every structural and value invariant for a document of lineage `doc_salt` whose
    /// id counter high-water mark is `next_id`.
    ///
    /// Rules: ids belong to this lineage (salt matches) and are below `next_id`; ids are unique
    /// across pages and nodes; every child reference resolves; every node is referenced exactly
    /// once and reachable from a page (no orphans, no cycles); child lists only on container
    /// kinds; `VectorPath` ⇔ path payload present; transforms and paths are finite and valid.
    ///
    /// # Errors
    /// Returns [`GipError::Invalid`] naming the first violated rule.
    pub fn validate(&self, doc_salt: u32, next_id: u64) -> Result<(), GipError> {
        let invalid = |msg: String| GipError::Invalid(msg);
        let mut seen_ids = BTreeSet::new();
        for id in self
            .pages
            .iter()
            .map(|p| p.id)
            .chain(self.nodes.keys().copied())
        {
            if !seen_ids.insert(id) {
                return Err(invalid(format!("duplicate id {id}")));
            }
            if id.salt() != doc_salt {
                return Err(invalid(format!(
                    "id {id} does not belong to lineage salt {doc_salt:06x}"
                )));
            }
            if id.counter() >= next_id {
                return Err(invalid(format!(
                    "id {id} counter is at or above nextId {next_id}"
                )));
            }
        }
        let mut referenced: BTreeMap<NodeId, usize> = BTreeMap::new();
        for page in &self.pages {
            for child in &page.children {
                *referenced.entry(*child).or_default() += 1;
            }
        }
        for (id, node) in &self.nodes {
            node.transform
                .validate()
                .map_err(|e| invalid(format!("node {id}: {e}")))?;
            if !(node.opacity.is_finite() && (0.0..=1.0).contains(&node.opacity)) {
                return Err(invalid(format!("node {id}: opacity must be in [0, 1]")));
            }
            if !node.children.is_empty() && !node.kind.is_container() {
                return Err(invalid(format!(
                    "node {id}: {:?} cannot hold children",
                    node.kind
                )));
            }
            match (node.kind, &node.path) {
                (NodeKind::VectorPath, None) => {
                    return Err(invalid(format!(
                        "node {id}: vector-path requires a path payload"
                    )));
                }
                (kind, Some(_)) if kind != NodeKind::VectorPath => {
                    return Err(invalid(format!(
                        "node {id}: only vector-path carries a path payload"
                    )));
                }
                (_, Some(path)) => path
                    .validate_finite()
                    .map_err(|e| invalid(format!("node {id}: {e}")))?,
                _ => {}
            }
            for child in &node.children {
                *referenced.entry(*child).or_default() += 1;
            }
        }
        for (id, count) in &referenced {
            if !self.nodes.contains_key(id) {
                return Err(invalid(format!(
                    "child reference {id} does not resolve to a node"
                )));
            }
            if *count > 1 {
                return Err(invalid(format!("node {id} is referenced {count} times")));
            }
        }
        for id in self.nodes.keys() {
            if !referenced.contains_key(id) {
                return Err(invalid(format!(
                    "node {id} is an orphan (not reachable from a page)"
                )));
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    // Behavioural tests for validation live with the golden fixtures (tests/document_rules.rs);
    // this module keeps only a smoke test that an empty document is valid.
    use super::*;

    #[test]
    fn empty_document_is_valid() {
        assert!(Document::default().validate(1, 1).is_ok());
    }
}
