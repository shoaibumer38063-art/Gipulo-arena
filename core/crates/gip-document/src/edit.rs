//! THE WRITE DOOR (ADR-004 #4, ADR-004 A6). The closed op vocabulary and the only code that
//! changes an existing document. Callers outside `gip-document` and `gip-history` must not use
//! this module (enforced by `scripts/check-boundaries.py`). Edit operations (ADR-004 #4): the closed op vocabulary. Every op records both `before` and
//! `after` payloads, so undo and redo replay recorded values and never re-run mutator logic.
//! `apply` is the only code that changes the document during undo, redo, and rollback.

use crate::{Document, FieldKey, FieldValue, GipFile, Node, NodeId};

/// Where a child list lives: a page's top-level list or a container node's children.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Parent {
    /// Top-level nodes of the page with this id.
    Page(NodeId),
    /// Children of the container node with this id.
    Node(NodeId),
}

/// A node and its subtree, root first (ADR-004 #4: subtree snapshots make inverses atomic).
pub type Subtree = Vec<(NodeId, Node)>;

/// One field-level or structural change.
#[derive(Debug, Clone, PartialEq)]
pub enum EditOp {
    /// One scalar field of one node.
    SetField {
        /// Node that changes.
        node: NodeId,
        /// Field that changes.
        key: FieldKey,
        /// Value before the change (replayed on undo).
        before: FieldValue,
        /// Value after the change (replayed on redo).
        after: FieldValue,
    },
    /// A subtree enters the document at `(parent, index)`. Its inverse removes it.
    InsertNode {
        /// Parent child list.
        parent: Parent,
        /// Position in that list.
        index: usize,
        /// Root first, then descendants.
        subtree: Subtree,
    },
    /// A subtree leaves the document from `(parent, index)`. Its inverse re-inserts it.
    RemoveNode {
        /// Parent child list.
        parent: Parent,
        /// Position in that list.
        index: usize,
        /// Root first, then descendants (the full removed subtree).
        subtree: Subtree,
    },
    /// An existing node changes place (identity preserved). Both locations are recorded.
    MoveNode {
        /// Node that moves.
        node: NodeId,
        /// Parent and index before the move.
        from: (Parent, usize),
        /// Parent and index after the move (index in the list after the node is taken out).
        to: (Parent, usize),
    },
    /// A child list is replaced with a recorded order (sibling reorder, or any list rewrite).
    ReorderChildren {
        /// Parent whose list changes.
        parent: Parent,
        /// List before the change.
        before: Vec<NodeId>,
        /// List after the change.
        after: Vec<NodeId>,
    },
    /// Document name.
    SetName {
        /// Name before the change.
        before: String,
        /// Name after the change.
        after: String,
    },
}

/// Direction in which an op is replayed.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Dir {
    /// Redo / original application.
    Forward,
    /// Undo / rollback.
    Backward,
}

/// Mutable access to a page's or container's child list.
fn list_mut(doc: &mut Document, parent: Parent) -> Result<&mut Vec<NodeId>, String> {
    match parent {
        Parent::Page(id) => doc
            .pages
            .iter_mut()
            .find(|p| p.id == id)
            .map(|p| &mut p.children)
            .ok_or_else(|| format!("no page {id}")),
        Parent::Node(id) => doc
            .nodes
            .get_mut(&id)
            .map(|n| &mut n.children)
            .ok_or_else(|| format!("no node {id}")),
    }
}

/// Collects `root` and all descendants, root first.
///
/// # Errors
/// Returns a message when a referenced node is missing.
pub fn snapshot(doc: &Document, root: NodeId) -> Result<Subtree, String> {
    let mut out = Vec::new();
    let mut stack = vec![root];
    while let Some(id) = stack.pop() {
        let node = doc.nodes.get(&id).ok_or_else(|| format!("no node {id}"))?;
        out.push((id, node.clone()));
        stack.extend(node.children.iter().rev().copied());
    }
    Ok(out)
}

fn insert_subtree(
    doc: &mut Document,
    parent: Parent,
    index: usize,
    subtree: &[(NodeId, Node)],
) -> Result<(), String> {
    let root = subtree.first().map(|(id, _)| *id).ok_or("empty subtree")?;
    if subtree.iter().any(|(id, _)| doc.nodes.contains_key(id)) {
        return Err("insert collides with an existing node id".into());
    }
    {
        let list = list_mut(doc, parent)?;
        if index > list.len() {
            return Err(format!(
                "insert index {index} beyond list length {}",
                list.len()
            ));
        }
        list.insert(index, root);
    }
    for (id, node) in subtree {
        doc.nodes.insert(*id, node.clone());
    }
    Ok(())
}

fn remove_subtree(
    doc: &mut Document,
    parent: Parent,
    index: usize,
    subtree: &[(NodeId, Node)],
) -> Result<(), String> {
    let root = subtree.first().map(|(id, _)| *id).ok_or("empty subtree")?;
    if subtree.iter().any(|(id, _)| !doc.nodes.contains_key(id)) {
        return Err("remove references a node that is not in the document".into());
    }
    {
        let list = list_mut(doc, parent)?;
        if list.get(index) != Some(&root) {
            return Err(format!("remove expected {root} at index {index}"));
        }
        list.remove(index);
    }
    for (id, _) in subtree {
        doc.nodes.remove(id);
    }
    Ok(())
}

fn take_child(
    doc: &mut Document,
    parent: Parent,
    index: usize,
    node: NodeId,
) -> Result<(), String> {
    let list = list_mut(doc, parent)?;
    if list.get(index) != Some(&node) {
        return Err(format!("move expected {node} at index {index}"));
    }
    list.remove(index);
    Ok(())
}

fn put_child(doc: &mut Document, parent: Parent, index: usize, node: NodeId) -> Result<(), String> {
    let list = list_mut(doc, parent)?;
    if index > list.len() {
        return Err(format!(
            "move index {index} beyond list length {}",
            list.len()
        ));
    }
    list.insert(index, node);
    Ok(())
}

/// Replays one op in one direction. Each arm checks its preconditions before mutating, so a
/// failed apply leaves the document unchanged.
///
/// # Errors
/// Returns a message when the recorded state does not match the document (an internal
/// invariant failure, never a user error).
pub fn apply(file: &mut GipFile, op: &EditOp, dir: Dir) -> Result<(), String> {
    let doc = &mut file.document;
    match op {
        EditOp::SetField {
            node,
            key,
            before,
            after,
        } => {
            let value = if dir == Dir::Forward { after } else { before };
            doc.nodes
                .get_mut(node)
                .ok_or_else(|| format!("no node {node}"))?
                .set_field(*key, value.clone())
        }
        EditOp::InsertNode {
            parent,
            index,
            subtree,
        } => match dir {
            Dir::Forward => insert_subtree(doc, *parent, *index, subtree),
            Dir::Backward => remove_subtree(doc, *parent, *index, subtree),
        },
        EditOp::RemoveNode {
            parent,
            index,
            subtree,
        } => match dir {
            Dir::Forward => remove_subtree(doc, *parent, *index, subtree),
            Dir::Backward => insert_subtree(doc, *parent, *index, subtree),
        },
        EditOp::MoveNode { node, from, to } => {
            let ((take_parent, take_index), (put_parent, put_index)) = if dir == Dir::Forward {
                (*from, *to)
            } else {
                (*to, *from)
            };
            take_child(doc, take_parent, take_index, *node)?;
            put_child(doc, put_parent, put_index, *node)
        }
        EditOp::ReorderChildren {
            parent,
            before,
            after,
        } => {
            let list = list_mut(doc, *parent)?;
            *list = if dir == Dir::Forward {
                after.clone()
            } else {
                before.clone()
            };
            Ok(())
        }
        EditOp::SetName { before, after } => {
            file.meta.name = if dir == Dir::Forward {
                after.clone()
            } else {
                before.clone()
            };
            Ok(())
        }
    }
}

/// Approximate payload size of an op, for the undo byte budget (ADR-004 #9).
#[must_use]
pub fn op_bytes(op: &EditOp) -> usize {
    match op {
        EditOp::SetField { before, after, .. } => 96 + value_bytes(before) + value_bytes(after),
        EditOp::InsertNode { subtree, .. } | EditOp::RemoveNode { subtree, .. } => {
            64 + subtree.iter().map(|(_, n)| n.approx_bytes()).sum::<usize>()
        }
        EditOp::MoveNode { .. } => 96,
        EditOp::ReorderChildren { before, after, .. } => 64 + 16 * (before.len() + after.len()),
        EditOp::SetName { before, after } => 64 + before.len() + after.len(),
    }
}

fn value_bytes(v: &FieldValue) -> usize {
    match v {
        FieldValue::Text(t) => t.len(),
        _ => 0,
    }
}

/// Every node id an op touches (affected-set publication, ADR-004 #12).
pub fn affected_ids(op: &EditOp, out: &mut std::collections::BTreeSet<NodeId>) {
    let parent_id = |p: &Parent| match p {
        Parent::Page(id) | Parent::Node(id) => *id,
    };
    match op {
        EditOp::SetField { node, .. } => {
            out.insert(*node);
        }
        EditOp::InsertNode {
            parent, subtree, ..
        }
        | EditOp::RemoveNode {
            parent, subtree, ..
        } => {
            out.insert(parent_id(parent));
            out.extend(subtree.iter().map(|(id, _)| *id));
        }
        EditOp::MoveNode { node, from, to } => {
            out.insert(*node);
            out.insert(parent_id(&from.0));
            out.insert(parent_id(&to.0));
        }
        EditOp::ReorderChildren { parent, .. } => {
            out.insert(parent_id(parent));
        }
        EditOp::SetName { .. } => {}
    }
}
