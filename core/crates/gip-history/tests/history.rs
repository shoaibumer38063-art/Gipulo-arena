//! Transaction and undo/redo laws (undo-redo.md §6/§8, ADR-004): gesture = one entry, rollback
//! is exact, undo/redo restore the document exactly, ids are never reused, budget and dirty
//! tracking behave as documented.

use gip_document::{FieldKey, FieldValue, GipFile, Node, NodeKind, Transform};
use gip_history::{History, IntentLabel, Parent, TxError};

const SALT: u32 = 0x0012_3456;

/// A history with one page, one frame, and one rectangle child.
fn fixture() -> (History, gip_document::NodeId, gip_document::NodeId) {
    let file = GipFile::new("Doc", SALT);
    let page_id = file.document().pages()[0].id;
    let mut h = History::new(file).unwrap();
    let (frame, rect) = h
        .transact(IntentLabel::new("add-frame"), |tx| {
            let frame = tx.alloc_id()?;
            let rect = tx.alloc_id()?;
            let mut frame_node = Node::new(NodeKind::Frame, "Frame", Transform::ZERO);
            frame_node.children = vec![rect];
            tx.insert(
                Parent::Page(page_id),
                0,
                vec![
                    (frame, frame_node),
                    (
                        rect,
                        Node::new(NodeKind::Rectangle, "Rect", Transform::ZERO),
                    ),
                ],
            )?;
            Ok((frame, rect))
        })
        .unwrap()
        .0;
    (h, frame, rect)
}

#[test]
fn a_thousand_drag_events_are_one_entry_and_one_undo() {
    let (mut h, _, rect) = fixture();
    let before = h.document().clone();
    h.transact(IntentLabel::new("move"), |tx| {
        for i in 0..1000 {
            tx.set_field(rect, FieldKey::X, FieldValue::Number(f64::from(i)))?;
        }
        Ok(())
    })
    .unwrap();
    assert_eq!(h.undo_label().map(IntentLabel::as_str), Some("move"));
    h.undo().unwrap();
    assert_eq!(
        h.document(),
        &before,
        "one undo restores the exact pre-gesture document"
    );
    assert_eq!(h.undo_label().map(IntentLabel::as_str), Some("add-frame"));
}

#[test]
fn rejected_mutator_rolls_back_every_write_and_records_nothing() {
    let (mut h, _, rect) = fixture();
    let before = h.document().clone();
    let depth_before = h.can_undo();
    let result = h.transact(IntentLabel::new("broken"), |tx| {
        tx.set_field(rect, FieldKey::X, FieldValue::Number(99.0))?;
        tx.set_field(rect, FieldKey::Name, FieldValue::Text("gone".into()))?;
        Err::<(), _>("boom".into())
    });
    assert_eq!(result, Err(TxError::Rejected("boom".into())));
    assert_eq!(h.document(), &before);
    assert_eq!(h.can_undo(), depth_before);
    assert_eq!(h.undo_label().map(IntentLabel::as_str), Some("add-frame"));
}

#[test]
fn invariant_violation_rolls_back_atomically() {
    let (mut h, _, rect) = fixture();
    let before = h.document().clone();
    let result = h.transact(IntentLabel::new("resize"), |tx| {
        tx.set_field(rect, FieldKey::X, FieldValue::Number(5.0))?;
        tx.set_field(rect, FieldKey::W, FieldValue::Number(-1.0))?; // negative width is invalid
        Ok(())
    });
    assert!(matches!(result, Err(TxError::Invalid(_))), "got {result:?}");
    assert_eq!(h.document(), &before);
}

#[test]
fn no_op_transaction_records_no_entry() {
    let (mut h, _, rect) = fixture();
    let (_, changes) = h
        .transact(IntentLabel::new("noop"), |tx| {
            assert!(tx.document().nodes().contains_key(&rect));
            Ok(())
        })
        .unwrap();
    assert!(!changes.recorded);
    assert_eq!(h.undo_label().map(IntentLabel::as_str), Some("add-frame"));
}

#[test]
fn undo_then_redo_restores_exact_document_and_affected_set() {
    let (mut h, frame, rect) = fixture();
    let before = h.document().clone();
    h.transact(IntentLabel::new("nudge"), |tx| {
        tx.set_field(rect, FieldKey::Y, FieldValue::Number(12.5))?;
        tx.set_field(rect, FieldKey::Rotation, FieldValue::Number(-45.0))?;
        Ok(())
    })
    .unwrap();
    let after = h.document().clone();
    let undo = h.undo().unwrap().unwrap();
    assert!(undo.affected.contains(&rect));
    assert_eq!(h.document(), &before);
    let redo = h.redo().unwrap().unwrap();
    assert_eq!(h.document(), &after);
    assert!(redo.affected.contains(&rect));
    assert!(!redo.affected.contains(&frame));
}

#[test]
fn remove_subtree_is_atomic_and_undo_restores_children_in_place() {
    let (mut h, frame, rect) = fixture();
    let before = h.document().clone();
    h.transact(IntentLabel::new("delete"), |tx| tx.remove(frame))
        .unwrap();
    assert!(h.document().nodes().is_empty());
    h.undo().unwrap();
    assert_eq!(
        h.document(),
        &before,
        "subtree (frame + rect) restored exactly"
    );
    assert!(h.document().nodes().contains_key(&rect));
}

#[test]
fn move_node_between_parents_and_reorder_undo_exactly() {
    let (mut h, frame, rect) = fixture();
    let page_id = h.document().pages()[0].id;
    let before = h.document().clone();
    h.transact(IntentLabel::new("move-out"), |tx| {
        tx.move_node(rect, (Parent::Page(page_id), 0))
    })
    .unwrap();
    assert!(h.document().nodes()[&frame].children.is_empty());
    assert_eq!(h.document().pages()[0].children, vec![rect, frame]);
    h.undo().unwrap();
    assert_eq!(h.document(), &before);
}

#[test]
fn reorder_children_undo_exactly() {
    let (mut h, frame, rect) = fixture();
    let a = h
        .transact(IntentLabel::new("add-two"), |tx| {
            let a = tx.alloc_id()?;
            let b = tx.alloc_id()?;
            tx.insert(
                Parent::Node(frame),
                1,
                vec![(a, Node::new(NodeKind::Ellipse, "A", Transform::ZERO))],
            )?;
            tx.insert(
                Parent::Node(frame),
                2,
                vec![(b, Node::new(NodeKind::Ellipse, "B", Transform::ZERO))],
            )?;
            Ok(vec![rect, a, b])
        })
        .unwrap()
        .0;
    let before = h.document().clone();
    h.transact(IntentLabel::new("reorder"), |tx| {
        tx.reorder(Parent::Node(frame), vec![a[2], a[0], a[1]])
    })
    .unwrap();
    assert_eq!(
        h.document().nodes()[&frame].children,
        vec![a[2], a[0], a[1]]
    );
    h.undo().unwrap();
    assert_eq!(h.document(), &before);
}

#[test]
fn ids_are_never_reused_even_after_undo() {
    let (mut h, _, rect) = fixture();
    let first = h
        .transact(IntentLabel::new("probe"), |tx| {
            let id = tx.alloc_id()?;
            Ok(id)
        })
        .unwrap()
        .0;
    h.undo().ok();
    let second = h
        .transact(IntentLabel::new("probe"), |tx| tx.alloc_id())
        .unwrap()
        .0;
    assert_ne!(first, second);
    assert_ne!(first, rect);
}

#[test]
fn new_edit_after_undo_truncates_redo_tail() {
    let (mut h, _, rect) = fixture();
    h.transact(IntentLabel::new("a"), |tx| {
        tx.set_field(rect, FieldKey::X, FieldValue::Number(1.0))
    })
    .unwrap();
    h.undo().unwrap();
    assert!(h.can_redo());
    h.transact(IntentLabel::new("b"), |tx| {
        tx.set_field(rect, FieldKey::X, FieldValue::Number(2.0))
    })
    .unwrap();
    assert!(!h.can_redo());
    assert_eq!(h.redo_label(), None);
}

#[test]
fn undo_and_redo_labels_follow_the_stacks() {
    let (mut h, _, rect) = fixture();
    h.transact(IntentLabel::new("first"), |tx| {
        tx.set_field(rect, FieldKey::X, FieldValue::Number(1.0))
    })
    .unwrap();
    assert_eq!(h.undo_label().map(IntentLabel::as_str), Some("first"));
    h.undo().unwrap();
    assert_eq!(h.redo_label().map(IntentLabel::as_str), Some("first"));
    assert!(h.undo().unwrap().is_some(), "add-frame is undone next");
    assert_eq!(h.undo_label(), None);
    assert!(h.undo().unwrap().is_none());
}

#[test]
fn rename_is_undoable() {
    let (mut h, _, _) = fixture();
    h.transact(IntentLabel::new("rename"), |tx| tx.set_name("Renamed"))
        .unwrap();
    assert_eq!(h.file().meta().name, "Renamed");
    h.undo().unwrap();
    assert_eq!(h.file().meta().name, "Doc");
}

#[test]
fn dirty_tracks_position_relative_to_save_point() {
    let (mut h, _, rect) = fixture();
    h.mark_saved();
    assert!(!h.is_dirty());
    h.transact(IntentLabel::new("edit"), |tx| {
        tx.set_field(rect, FieldKey::X, FieldValue::Number(3.0))
    })
    .unwrap();
    assert!(h.is_dirty());
    h.mark_saved();
    h.undo().unwrap();
    assert!(h.is_dirty(), "undoing past the save point is dirty");
}

#[test]
fn budget_drops_oldest_entries_and_keeps_the_newest() {
    let (h, _, rect) = fixture();
    let mut h = History::with_budget(h.into_file(), 2_000).unwrap();
    for i in 0..50 {
        h.transact(IntentLabel::new(format!("step-{i}")), |tx| {
            tx.set_field(rect, FieldKey::X, FieldValue::Number(f64::from(i)))
        })
        .unwrap();
    }
    let mut steps = 0;
    while h.undo().unwrap().is_some() {
        steps += 1;
    }
    assert!(steps < 50, "oldest entries were dropped");
    assert!(steps >= 1, "the newest entry is always kept");
    // Steps set X = 0..49. Undoing `steps` entries lands on the value before the oldest kept
    // step, which is 49 - steps: the documented `canUndo` boundary, not a silent reset.
    assert_eq!(
        h.document().nodes()[&rect].field(FieldKey::X),
        FieldValue::Number(f64::from(49 - steps))
    );
}

#[test]
fn nested_helpers_fold_into_one_entry() {
    fn resize_with_reflow(
        tx: &mut gip_history::Tx<'_>,
        rect: gip_document::NodeId,
    ) -> Result<(), String> {
        tx.set_field(rect, FieldKey::W, FieldValue::Number(40.0))?;
        tx.set_field(rect, FieldKey::H, FieldValue::Number(20.0))?;
        Ok(())
    }
    let (mut h, _, rect) = fixture();
    h.transact(IntentLabel::new("resize"), |tx| {
        resize_with_reflow(tx, rect)
    })
    .unwrap();
    h.undo().unwrap();
    assert_eq!(
        h.undo_label().map(IntentLabel::as_str),
        Some("add-frame"),
        "resize was one entry"
    );
}
