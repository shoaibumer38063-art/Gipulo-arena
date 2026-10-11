//! The transaction collector (ADR-004 #6/#7). A `Tx` is handed to exactly one mutator; every
//! write it performs is applied immediately AND recorded, so a failure can roll back exactly.
//! Nesting is structural: helper functions called inside one mutator share the same `Tx`, so
//! they fold into one entry by construction.

use gip_document::{Document, FieldKey, FieldValue, GipFile, NodeId};

use gip_document::edit::{Dir, EditOp, Parent, Subtree, apply, snapshot};

/// A write session for one user intent. Only `History::transact` creates one.
pub struct Tx<'a> {
    pub(crate) file: &'a mut GipFile,
    pub(crate) ops: Vec<EditOp>,
}

impl Tx<'_> {
    /// Read access to the document as it is inside this transaction.
    #[must_use]
    pub fn document(&self) -> &Document {
        self.file.document()
    }

    /// Allocates a fresh node id. Ids are never reused (CAN-SER-3), so allocation is NOT undone
    /// by rollback or undo: an undone insert leaves a gap in the counter, by design.
    ///
    /// # Errors
    /// Returns a message when the 40-bit counter space is exhausted.
    pub fn alloc_id(&mut self) -> Result<NodeId, String> {
        self.file.allocate_id()
    }

    /// Sets one scalar field, recording its previous value.
    ///
    /// # Errors
    /// Returns a message when the node is missing or the value type does not match the key.
    pub fn set_field(
        &mut self,
        node: NodeId,
        key: FieldKey,
        after: FieldValue,
    ) -> Result<(), String> {
        let before = self
            .file
            .document()
            .nodes()
            .get(&node)
            .ok_or_else(|| format!("no node {node}"))?
            .field(key);
        self.record(EditOp::SetField {
            node,
            key,
            before,
            after,
        })
    }

    /// Inserts a subtree (root first, ids from [`Tx::alloc_id`]) at `(parent, index)`.
    ///
    /// # Errors
    /// Returns a message when the subtree is empty, an id already exists, or the index is out of range.
    pub fn insert(&mut self, parent: Parent, index: usize, subtree: Subtree) -> Result<(), String> {
        self.record(EditOp::InsertNode {
            parent,
            index,
            subtree,
        })
    }

    /// Removes a node and its whole subtree, recording the snapshot.
    ///
    /// # Errors
    /// Returns a message when the node is not in the document.
    pub fn remove(&mut self, node: NodeId) -> Result<(), String> {
        let (parent, index) = self.locate(node)?;
        let subtree = snapshot(self.file.document(), node)?;
        self.record(EditOp::RemoveNode {
            parent,
            index,
            subtree,
        })
    }

    /// Moves an existing node to `(parent, index)`. Index is in the list after the node is taken out.
    ///
    /// # Errors
    /// Returns a message when the node is not in the document.
    pub fn move_node(&mut self, node: NodeId, to: (Parent, usize)) -> Result<(), String> {
        let from = self.locate(node)?;
        self.record(EditOp::MoveNode { node, from, to })
    }

    /// Replaces a child list with a new order.
    ///
    /// # Errors
    /// Returns a message when the parent is missing.
    pub fn reorder(&mut self, parent: Parent, after: Vec<NodeId>) -> Result<(), String> {
        let doc = self.file.document();
        let before = match parent {
            Parent::Page(id) => doc
                .pages()
                .iter()
                .find(|p| p.id == id)
                .map(|p| p.children.clone()),
            Parent::Node(id) => doc.nodes().get(&id).map(|n| n.children.clone()),
        }
        .ok_or_else(|| format!("no parent {parent:?}"))?;
        self.record(EditOp::ReorderChildren {
            parent,
            before,
            after,
        })
    }

    /// Renames the document.
    ///
    /// # Errors
    /// Returns a message when the op cannot be applied (not expected).
    pub fn set_name(&mut self, name: impl Into<String>) -> Result<(), String> {
        let before = self.file.meta().name.clone();
        self.record(EditOp::SetName {
            before,
            after: name.into(),
        })
    }

    /// Finds the parent list and index of a node.
    fn locate(&self, node: NodeId) -> Result<(Parent, usize), String> {
        let doc = self.file.document();
        for page in doc.pages() {
            if let Some(i) = page.children.iter().position(|c| *c == node) {
                return Ok((Parent::Page(page.id), i));
            }
        }
        for (id, n) in doc.nodes() {
            if let Some(i) = n.children.iter().position(|c| *c == node) {
                return Ok((Parent::Node(*id), i));
            }
        }
        Err(format!("node {node} is not attached to the document"))
    }

    /// Applies an op forward and records it. `apply` checks preconditions before it mutates,
    /// so a failed record changes nothing.
    fn record(&mut self, op: EditOp) -> Result<(), String> {
        apply(self.file, &op, Dir::Forward)?;
        self.ops.push(op);
        Ok(())
    }
}
