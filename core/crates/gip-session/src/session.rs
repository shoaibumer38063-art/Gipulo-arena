//! Editor session (task 020): view state (CAN-CAM), document history, and the interaction
//! machine, combined behind one facade.
//!
//! Every conversion happens here or in `gip-coords` (CAN-COORD-2). The shell sends raw
//! client-pixel pointer positions and receives core-computed viewport positions; it never
//! performs zoom, pan, hit-test, or move arithmetic itself. Panning and zooming are not
//! history entries (CAN-CAM-1). Each committed gesture is exactly one history entry; a
//! cancelled gesture leaves none (CAN-MODE-4).

use gip_coords::viewport::screen_to_viewport;
use gip_coords::{Camera, Mat2, Vec2, ViewportRect};
use gip_document::{FieldKey, FieldValue, GipFile, Node, NodeId, NodeKind, Transform};
use gip_geometry::trs_of;
use gip_history::{History, IntentLabel, Parent};
use gip_interaction::{Commit, Effect, Interaction, Preview, Tool, View};

/// Document salt for documents created by the session (Slice 0; a real salt comes with file I/O).
const NEW_DOC_SALT: u32 = 1;

/// One drawable rectangle, as viewport-pixel corners (clockwise from the local origin).
#[derive(Debug, Clone, PartialEq)]
pub struct RectOp {
    /// Node id; `None` for the in-progress creation preview.
    pub id: Option<NodeId>,
    /// Corners in viewport pixels.
    pub corners: [Vec2; 4],
    /// Node is selected (selection outline, CAN-SEL-5).
    pub selected: bool,
    /// Pointer is over the node while idle (hover outline, CAN-SEL-6).
    pub hovered: bool,
    /// Creation preview, not a document node.
    pub preview: bool,
}

/// Snapshot of the view for the shell to paint. All positions are viewport pixels (relative to
/// the canvas element's top-left), already converted by the core.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ViewSnapshot {
    /// Zoom as a factor: 1.0 = 100 %.
    pub zoom: f64,
    /// Zoom as a percentage, computed by the core (display input for the shell).
    pub zoom_percent: f64,
    /// World x of the viewport origin (camera position).
    pub camera_x: f64,
    /// World y of the viewport origin (camera position).
    pub camera_y: f64,
    /// Where the world origin (0, 0) lands, in viewport pixels.
    pub origin: Vec2,
    /// Viewport width in CSS pixels.
    pub width: f64,
    /// Viewport height in CSS pixels.
    pub height: f64,
}

/// The editor session: view, document history, and interaction.
#[derive(Debug)]
pub struct Session {
    rect: ViewportRect,
    camera: Camera,
    history: History,
    machine: Interaction,
}

impl Session {
    /// A session with an empty viewport, the default camera, and an empty page.
    #[must_use]
    pub fn new() -> Self {
        let file = GipFile::new("Untitled", NEW_DOC_SALT);
        let history = History::new(file).expect("a fresh GipFile is valid");
        Self {
            rect: ViewportRect::new(0.0, 0.0, 0.0, 0.0),
            camera: Camera::DEFAULT,
            history,
            machine: Interaction::new(),
        }
    }

    /// Records the canvas element's client rect after mount or resize. Returns `false` (and
    /// changes nothing) for non-finite or negative sizes.
    pub fn set_viewport(&mut self, x: f64, y: f64, width: f64, height: f64) -> bool {
        let valid =
            [x, y, width, height].iter().all(|v| v.is_finite()) && width >= 0.0 && height >= 0.0;
        if valid {
            self.rect = ViewportRect::new(x, y, width, height);
        }
        valid
    }

    /// Zooms by `factor` around a client-pixel point, keeping that point fixed (CAN-CAM-2).
    /// Returns `false` (and changes nothing) for a non-finite point or a non-positive factor.
    pub fn zoom_at(&mut self, client_x: f64, client_y: f64, factor: f64) -> bool {
        let valid =
            client_x.is_finite() && client_y.is_finite() && factor.is_finite() && factor > 0.0;
        if valid {
            let anchor = screen_to_viewport(Vec2::new(client_x, client_y), self.rect);
            self.camera.zoom_around_viewport(anchor, factor);
        }
        valid
    }

    /// Pans by a pixel delta (the same units as pointer movement). Returns `false` for
    /// non-finite deltas.
    pub fn pan_by(&mut self, dx: f64, dy: f64) -> bool {
        let valid = dx.is_finite() && dy.is_finite();
        if valid {
            self.camera.pan_by_viewport(Vec2::new(dx, dy));
        }
        valid
    }

    /// The current view, ready for painting.
    #[must_use]
    pub fn snapshot(&self) -> ViewSnapshot {
        ViewSnapshot {
            zoom: self.camera.zoom,
            zoom_percent: self.camera.zoom_percent(),
            camera_x: self.camera.x,
            camera_y: self.camera.y,
            origin: self.camera.world_to_viewport(Vec2::new(0.0, 0.0)),
            width: self.rect.width,
            height: self.rect.height,
        }
    }

    /// Pointer pressed at a client-pixel position. Returns `false` for non-finite input.
    pub fn pointer_down(&mut self, client_x: f64, client_y: f64, button: u8, shift: bool) -> bool {
        let Some(client) = finite(client_x, client_y) else {
            return false;
        };
        let view = self.view();
        let effects =
            self.machine
                .pointer_down(self.history.document(), view, client, button, shift);
        self.apply(effects);
        true
    }

    /// Pointer moved. Returns `false` for non-finite input.
    pub fn pointer_move(&mut self, client_x: f64, client_y: f64) -> bool {
        let Some(client) = finite(client_x, client_y) else {
            return false;
        };
        let view = self.view();
        let effects = self
            .machine
            .pointer_move(self.history.document(), view, client);
        self.apply(effects);
        true
    }

    /// Pointer released. Commits the gesture if it changed the document.
    pub fn pointer_up(&mut self, client_x: f64, client_y: f64) -> bool {
        let Some(client) = finite(client_x, client_y) else {
            return false;
        };
        let view = self.view();
        let effects = self
            .machine
            .pointer_up(self.history.document(), view, client);
        self.apply(effects);
        true
    }

    /// Cancels the gesture in progress. No history entry is made (CAN-MODE-4).
    pub fn cancel(&mut self) {
        self.machine.cancel();
    }

    /// Escape: cancels a running gesture, otherwise clears the selection.
    pub fn escape(&mut self) {
        self.machine.escape();
    }

    /// Switches the active tool. Returns `false` while a gesture is in progress.
    pub fn set_tool(&mut self, tool: Tool) -> bool {
        self.machine.set_tool(tool)
    }

    /// Active tool.
    #[must_use]
    pub fn tool(&self) -> Tool {
        self.machine.tool()
    }

    /// Deletes the selected nodes as one history entry. Returns `false` if nothing was deleted.
    pub fn delete_selection(&mut self) -> bool {
        let effects = self.machine.delete_selection();
        let before = self.history.document().nodes().len();
        self.apply(effects);
        self.history.document().nodes().len() < before
    }

    /// Undoes the last committed gesture. Returns `false` when there is nothing to undo.
    pub fn undo(&mut self) -> bool {
        let done = matches!(self.history.undo(), Ok(Some(_)));
        if done {
            self.machine.prune(self.history.document());
        }
        done
    }

    /// Redoes the last undone gesture. Returns `false` when there is nothing to redo.
    pub fn redo(&mut self) -> bool {
        let done = matches!(self.history.redo(), Ok(Some(_)));
        if done {
            self.machine.prune(self.history.document());
        }
        done
    }

    /// Serializes the document (validated; ADR-003). Used by round-trip tests and, later, by
    /// file I/O.
    pub fn save_json(&self) -> Result<String, String> {
        gip_document::to_json(self.history.file()).map_err(|e| e.to_string())
    }

    /// Whether `undo` would change anything.
    #[must_use]
    pub fn can_undo(&self) -> bool {
        self.history.can_undo()
    }

    /// Whether `redo` would change anything.
    #[must_use]
    pub fn can_redo(&self) -> bool {
        self.history.can_redo()
    }

    /// Number of nodes in the document's page.
    #[must_use]
    pub fn object_count(&self) -> usize {
        self.history
            .document()
            .pages()
            .first()
            .map_or(0, |p| p.children.len())
    }

    /// Number of selected nodes.
    #[must_use]
    pub fn selection_count(&self) -> usize {
        self.machine.selection().len()
    }

    /// Drawable rectangles in paint order (bottom first), in viewport pixels, plus the
    /// creation preview when one is in progress.
    #[must_use]
    pub fn rects(&self) -> Vec<RectOp> {
        let doc = self.history.document();
        let camera = self.camera;
        let preview = self.machine.preview();
        let moved = |id: NodeId| -> Option<(f64, f64)> {
            match &preview {
                Preview::Moving { items } => items.iter().find(|(n, _)| *n == id).map(|(_, p)| *p),
                _ => None,
            }
        };
        let mut out = Vec::new();
        let Some(page) = doc.pages().first() else {
            return out;
        };
        for id in &page.children {
            let Some(node) = doc.nodes().get(id) else {
                continue;
            };
            if !node.visible {
                continue;
            }
            let mut transform: Transform = node.transform;
            if let Some((x, y)) = moved(*id) {
                transform.x = x;
                transform.y = y;
            }
            let m = Mat2::from_trs(trs_of(&transform));
            out.push(RectOp {
                id: Some(*id),
                corners: unit_corners().map(|p| camera.world_to_viewport(m.apply(p))),
                selected: self.machine.selection().contains(id),
                hovered: self.machine.hover() == Some(*id),
                preview: false,
            });
        }
        if let Preview::Creating { x, y, w, h } = preview {
            let corners = [
                Vec2::new(x, y),
                Vec2::new(x + w, y),
                Vec2::new(x + w, y + h),
                Vec2::new(x, y + h),
            ];
            out.push(RectOp {
                id: None,
                corners: corners.map(|p| camera.world_to_viewport(p)),
                selected: false,
                hovered: false,
                preview: true,
            });
        }
        out
    }

    fn view(&self) -> View {
        View {
            camera: self.camera,
            rect: self.rect,
        }
    }

    fn apply(&mut self, effects: Vec<Effect>) {
        for effect in effects {
            match effect {
                Effect::Pan { dx, dy } => {
                    self.pan_by(dx, dy);
                }
                Effect::Commit(commit) => self.commit(commit),
            }
        }
    }

    fn commit(&mut self, commit: Commit) {
        match commit {
            Commit::CreateRect { x, y, w, h } => {
                let result = self
                    .history
                    .transact(IntentLabel::new("create-rectangle"), |tx| {
                        let page = tx.document().pages().first().ok_or("no page")?;
                        let (page_id, index) = (page.id, page.children.len());
                        let id = tx.alloc_id()?;
                        let node = Node::new(
                            NodeKind::Rectangle,
                            format!("Rectangle {}", index + 1),
                            Transform {
                                x,
                                y,
                                w,
                                h,
                                rotation_deg: 0.0,
                                flip_h: false,
                                flip_v: false,
                            },
                        );
                        tx.insert(Parent::Page(page_id), index, vec![(id, node)])?;
                        Ok(id)
                    });
                if let Ok((id, _)) = result {
                    self.machine.set_selection(&[id]);
                }
            }
            Commit::Move { items } => {
                let _ = self.history.transact(IntentLabel::new("move"), |tx| {
                    for item in &items {
                        tx.set_field(item.node, FieldKey::X, FieldValue::Number(item.to.0))?;
                        tx.set_field(item.node, FieldKey::Y, FieldValue::Number(item.to.1))?;
                    }
                    Ok(())
                });
            }
            Commit::Delete { nodes } => {
                let result = self.history.transact(IntentLabel::new("delete"), |tx| {
                    for node in &nodes {
                        tx.remove(*node)?;
                    }
                    Ok(())
                });
                if result.is_ok() {
                    self.machine.set_selection(&[]);
                }
            }
        }
    }
}

fn finite(x: f64, y: f64) -> Option<Vec2> {
    (x.is_finite() && y.is_finite()).then(|| Vec2::new(x, y))
}

fn unit_corners() -> [Vec2; 4] {
    [
        Vec2::new(0.0, 0.0),
        Vec2::new(1.0, 0.0),
        Vec2::new(1.0, 1.0),
        Vec2::new(0.0, 1.0),
    ]
}

impl Default for Session {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn approx(a: f64, b: f64) -> bool {
        (a - b).abs() < 1e-9
    }

    #[test]
    fn zoom_percent_is_supplied_by_the_core() {
        let mut s = Session::new();
        assert!(approx(s.snapshot().zoom_percent, 100.0));
        assert!(s.zoom_at(0.0, 0.0, 2.0));
        assert!(approx(s.snapshot().zoom_percent, 200.0));
    }

    #[test]
    fn default_view_is_100_percent_with_origin_at_viewport_origin() {
        let s = Session::new().snapshot();
        assert!(approx(s.zoom, 1.0));
        assert!(approx(s.origin.x, 0.0) && approx(s.origin.y, 0.0));
    }

    #[test]
    fn pointer_zoom_keeps_the_anchor_point_fixed() {
        let mut s = Session::new();
        assert!(s.set_viewport(100.0, 50.0, 800.0, 600.0));
        // Client point (300, 200) is viewport (200, 150); its world point must not move.
        let before = Camera::DEFAULT.viewport_to_world(Vec2::new(200.0, 150.0));
        for factor in [1.1, 1.1, 0.5, 3.0, 1.25] {
            assert!(s.zoom_at(300.0, 200.0, factor));
            let cam = Camera::new(
                s.snapshot().camera_x,
                s.snapshot().camera_y,
                s.snapshot().zoom,
            );
            let world = cam.viewport_to_world(Vec2::new(200.0, 150.0));
            assert!(
                approx(world.x, before.x) && approx(world.y, before.y),
                "anchor drifted"
            );
        }
    }

    #[test]
    fn zoom_is_clamped_to_the_camera_range() {
        let mut s = Session::new();
        assert!(s.set_viewport(0.0, 0.0, 800.0, 600.0));
        for _ in 0..200 {
            assert!(s.zoom_at(10.0, 10.0, 1.5));
        }
        assert!(approx(s.snapshot().zoom, gip_coords::MAX_ZOOM));
        for _ in 0..400 {
            assert!(s.zoom_at(10.0, 10.0, 0.5));
        }
        assert!(approx(s.snapshot().zoom, gip_coords::MIN_ZOOM));
    }

    #[test]
    fn pan_moves_the_content_with_the_pointer() {
        // Dragging the view by (+30, -12) moves the world origin by the same pixels: the
        // content follows the pointer (CAN-CAM-4).
        let mut s = Session::new();
        assert!(s.pan_by(30.0, -12.0));
        let snap = s.snapshot();
        assert!(
            approx(snap.origin.x, 30.0),
            "origin follows the pointer in x"
        );
        assert!(
            approx(snap.origin.y, -12.0),
            "origin follows the pointer in y"
        );
    }

    #[test]
    fn pan_is_exact_at_any_zoom() {
        let mut s = Session::new();
        assert!(s.zoom_at(0.0, 0.0, 4.0));
        assert!(s.pan_by(40.0, 0.0));
        // A 40 px drag is 40 screen px regardless of zoom: origin moves by exactly 40.
        assert!(approx(s.snapshot().origin.x, 40.0));
    }

    #[test]
    fn invalid_input_is_rejected_and_changes_nothing() {
        let mut s = Session::new();
        assert!(s.set_viewport(0.0, 0.0, 800.0, 600.0));
        let before = s.snapshot();
        assert!(!s.set_viewport(0.0, 0.0, -1.0, 600.0));
        assert!(!s.set_viewport(f64::NAN, 0.0, 10.0, 10.0));
        assert!(!s.zoom_at(f64::INFINITY, 0.0, 1.1));
        assert!(!s.zoom_at(0.0, 0.0, 0.0));
        assert!(!s.zoom_at(0.0, 0.0, -2.0));
        assert!(!s.pan_by(f64::NAN, 0.0));
        assert_eq!(s.snapshot(), before);
    }

    #[test]
    fn client_points_are_converted_through_the_viewport_rect() {
        let mut s = Session::new();
        assert!(s.set_viewport(100.0, 50.0, 800.0, 600.0));
        // Client (150, 80) is viewport (50, 30): a zoom anchored there keeps origin math consistent.
        assert!(s.zoom_at(150.0, 80.0, 2.0));
        let snap = s.snapshot();
        let expected_origin = Camera::new(snap.camera_x, snap.camera_y, snap.zoom)
            .world_to_viewport(Vec2::new(0.0, 0.0));
        assert!(
            approx(snap.origin.x, expected_origin.x) && approx(snap.origin.y, expected_origin.y)
        );
    }
}

#[cfg(test)]
mod interaction_tests {
    use super::*;
    use gip_coords::Vec2;

    fn session() -> Session {
        let mut s = Session::new();
        assert!(s.set_viewport(0.0, 0.0, 800.0, 600.0));
        s
    }

    /// Press at `from`, move through a midpoint past the threshold, release at `to`.
    fn drag(s: &mut Session, from: (f64, f64), to: (f64, f64)) {
        assert!(s.pointer_down(from.0, from.1, 0, false));
        let mid = ((from.0 + to.0) / 2.0, (from.1 + to.1) / 2.0);
        assert!(s.pointer_move(mid.0, mid.1));
        assert!(s.pointer_move(to.0, to.1));
        assert!(s.pointer_up(to.0, to.1));
    }

    fn draw_rect(s: &mut Session, from: (f64, f64), to: (f64, f64)) {
        assert!(s.set_tool(Tool::Rectangle));
        drag(s, from, to);
        assert!(s.set_tool(Tool::Select));
    }

    fn first_corner(s: &Session) -> Vec2 {
        s.rects()[0].corners[0]
    }

    fn approx(a: f64, b: f64) -> bool {
        (a - b).abs() < 1e-9
    }

    #[test]
    fn drag_with_rectangle_tool_creates_one_entry_and_selects_it() {
        let mut s = session();
        draw_rect(&mut s, (100.0, 100.0), (200.0, 150.0));
        assert_eq!(s.object_count(), 1);
        assert_eq!(s.selection_count(), 1);
        assert!(s.can_undo());
        let r = &s.rects()[0];
        assert!(r.selected && !r.preview);
        assert!(approx(r.corners[0].x, 100.0) && approx(r.corners[2].x, 200.0));
        assert!(approx(r.corners[2].y, 150.0));
    }

    #[test]
    fn click_below_the_threshold_creates_nothing() {
        let mut s = session();
        assert!(s.set_tool(Tool::Rectangle));
        assert!(s.pointer_down(50.0, 50.0, 0, false));
        assert!(s.pointer_move(52.0, 51.0)); // 2.2 px: below the 3 px threshold
        assert!(s.pointer_up(52.0, 51.0));
        assert_eq!(s.object_count(), 0);
        assert!(!s.can_undo());
    }

    #[test]
    fn zero_area_drag_creates_nothing() {
        let mut s = session();
        assert!(s.set_tool(Tool::Rectangle));
        drag(&mut s, (50.0, 50.0), (50.0, 200.0));
        assert_eq!(s.object_count(), 0);
    }

    #[test]
    fn escape_during_creation_leaves_no_history_entry() {
        let mut s = session();
        assert!(s.set_tool(Tool::Rectangle));
        assert!(s.pointer_down(10.0, 10.0, 0, false));
        assert!(s.pointer_move(120.0, 90.0));
        assert_eq!(
            s.rects().len(),
            1,
            "creation preview is visible mid-gesture"
        );
        s.escape();
        assert!(s.pointer_up(120.0, 90.0));
        assert_eq!(s.object_count(), 0);
        assert!(!s.can_undo());
    }

    #[test]
    fn move_is_one_entry_with_exact_delta() {
        let mut s = session();
        draw_rect(&mut s, (100.0, 100.0), (200.0, 150.0));
        drag(&mut s, (150.0, 120.0), (190.0, 140.0));
        assert!(approx(first_corner(&s).x, 140.0));
        assert!(approx(first_corner(&s).y, 120.0));
        // One undo reverts exactly the move; the object is still there, back at its drawn spot.
        assert!(s.undo());
        assert_eq!(s.object_count(), 1);
        assert!(approx(first_corner(&s).x, 100.0) && approx(first_corner(&s).y, 100.0));
        assert!(s.undo(), "second undo reverts the create");
        assert_eq!(s.object_count(), 0);
    }

    #[test]
    fn move_at_zoom_converts_pixels_to_world_units() {
        let mut s = session();
        draw_rect(&mut s, (100.0, 100.0), (200.0, 150.0));
        assert!(s.zoom_at(0.0, 0.0, 2.0)); // 200 %
        // Pointer moves 40 screen px = 20 world units.
        let c0 = s.rects()[0].corners[0];
        drag(
            &mut s,
            (c0.x + 10.0, c0.y + 10.0),
            (c0.x + 50.0, c0.y + 10.0),
        );
        let moved = s.rects()[0].corners[0];
        assert!(
            approx(moved.x - c0.x, 40.0),
            "40 px at 200 % moved {}",
            moved.x - c0.x
        );
    }

    #[test]
    fn moving_is_previewed_but_not_committed_until_release() {
        let mut s = session();
        draw_rect(&mut s, (100.0, 100.0), (200.0, 150.0));
        assert!(s.pointer_down(150.0, 120.0, 0, false));
        assert!(s.pointer_move(190.0, 120.0));
        assert!(approx(first_corner(&s).x, 140.0), "preview shows the drag");
        assert!(
            s.undo(),
            "the create entry is still there, and is the only one"
        );
        assert_eq!(s.object_count(), 0);
    }

    #[test]
    fn cancel_during_move_restores_exact_position_with_no_entry() {
        let mut s = session();
        draw_rect(&mut s, (100.0, 100.0), (200.0, 150.0));
        assert!(s.pointer_down(150.0, 120.0, 0, false));
        assert!(s.pointer_move(190.0, 160.0));
        s.cancel();
        assert!(s.pointer_up(190.0, 160.0));
        assert!(approx(first_corner(&s).x, 100.0) && approx(first_corner(&s).y, 100.0));
        // Only the create is in history: undoing it leaves nothing else to undo.
        assert!(s.undo());
        assert!(!s.can_undo());
    }

    #[test]
    fn repeated_moves_do_not_drift() {
        let mut s = session();
        draw_rect(&mut s, (100.0, 100.0), (200.0, 150.0));
        for i in 0..1000 {
            let d = if i % 2 == 0 { 1.0 } else { -1.0 };
            let x = first_corner(&s).x;
            drag(&mut s, (x + 5.0, 105.0), (x + 5.0 + 7.0 * d, 105.0));
        }
        assert!(approx(first_corner(&s).x, 100.0));
    }

    #[test]
    fn shift_click_toggles_membership() {
        let mut s = session();
        draw_rect(&mut s, (100.0, 100.0), (150.0, 150.0));
        draw_rect(&mut s, (300.0, 100.0), (350.0, 150.0));
        assert_eq!(s.selection_count(), 1);
        assert!(s.pointer_down(120.0, 120.0, 0, true)); // shift-click the first
        assert!(s.pointer_up(120.0, 120.0));
        assert_eq!(s.selection_count(), 2);
        assert!(s.pointer_down(120.0, 120.0, 0, true)); // shift-click it again
        assert!(s.pointer_up(120.0, 120.0));
        assert_eq!(s.selection_count(), 1);
    }

    #[test]
    fn click_on_empty_canvas_clears_selection() {
        let mut s = session();
        draw_rect(&mut s, (100.0, 100.0), (150.0, 150.0));
        assert!(s.pointer_down(500.0, 500.0, 0, false));
        assert!(s.pointer_up(500.0, 500.0));
        assert_eq!(s.selection_count(), 0);
    }

    #[test]
    fn delete_is_one_entry_and_undo_restores() {
        let mut s = session();
        draw_rect(&mut s, (100.0, 100.0), (150.0, 150.0));
        assert!(s.delete_selection());
        assert_eq!(s.object_count(), 0);
        assert_eq!(s.selection_count(), 0);
        assert!(s.undo());
        assert_eq!(s.object_count(), 1);
        assert!(s.redo());
        assert_eq!(s.object_count(), 0);
    }

    #[test]
    fn redo_is_cleared_by_a_new_gesture() {
        let mut s = session();
        draw_rect(&mut s, (100.0, 100.0), (150.0, 150.0));
        assert!(s.undo());
        assert!(s.can_redo());
        draw_rect(&mut s, (300.0, 300.0), (350.0, 350.0));
        assert!(!s.can_redo());
    }

    #[test]
    fn topmost_object_wins_the_hit() {
        let mut s = session();
        draw_rect(&mut s, (100.0, 100.0), (200.0, 200.0));
        draw_rect(&mut s, (150.0, 150.0), (250.0, 250.0));
        assert!(s.pointer_down(160.0, 160.0, 0, false)); // inside both
        assert!(s.pointer_up(160.0, 160.0));
        let sel: Vec<bool> = s.rects().iter().map(|r| r.selected).collect();
        assert_eq!(
            sel,
            vec![false, true],
            "the later (top) rectangle is selected"
        );
    }

    #[test]
    fn rotated_rectangle_hit_uses_its_true_shape() {
        let mut s = session();
        // A 100×100 square rotated 45° about its top-left corner (at 100,100).
        let node = Node::new(
            NodeKind::Rectangle,
            "tilted",
            Transform {
                x: 100.0,
                y: 100.0,
                w: 100.0,
                h: 100.0,
                rotation_deg: 45.0,
                flip_h: false,
                flip_v: false,
            },
        );
        let page = s.history.document().pages()[0].id;
        let id = s.history.transact(IntentLabel::new("test"), |tx| {
            let id = tx.alloc_id()?;
            tx.insert(Parent::Page(page), 0, vec![(id, node)])?;
            Ok(id)
        });
        assert!(id.is_ok());
        // The axis-aligned corner (190, 190) is outside the tilted square.
        assert!(s.pointer_down(190.0, 190.0, 0, false));
        assert!(s.pointer_up(190.0, 190.0));
        assert_eq!(s.selection_count(), 0);
        // Straight "below" the top corner, inside the tilted square, is a hit.
        assert!(s.pointer_down(100.0, 130.0, 0, false));
        assert!(s.pointer_up(100.0, 130.0));
        assert_eq!(s.selection_count(), 1);
    }

    #[test]
    fn middle_button_pans_without_a_history_entry() {
        let mut s = session();
        assert!(s.pointer_down(100.0, 100.0, 1, false));
        assert!(s.pointer_move(130.0, 88.0));
        assert!(s.pointer_up(130.0, 88.0));
        let snap = s.snapshot();
        assert!(approx(snap.origin.x, 30.0) && approx(snap.origin.y, -12.0));
        assert!(!s.can_undo());
    }

    #[test]
    fn escape_clears_selection_when_idle() {
        let mut s = session();
        draw_rect(&mut s, (100.0, 100.0), (150.0, 150.0));
        s.escape();
        assert_eq!(s.selection_count(), 0);
    }

    #[test]
    fn non_finite_pointer_input_is_rejected() {
        let mut s = session();
        assert!(!s.pointer_down(f64::NAN, 0.0, 0, false));
        assert!(!s.pointer_move(0.0, f64::INFINITY));
        assert!(!s.pointer_up(f64::NAN, f64::NAN));
        assert!(!s.set_tool(Tool::Select) || s.tool() == Tool::Select);
    }

    #[test]
    fn negative_direction_drag_normalizes_to_the_same_box() {
        let mut s = session();
        draw_rect(&mut s, (200.0, 260.0), (100.0, 100.0));
        assert_eq!(s.object_count(), 1);
        assert!(approx(first_corner(&s).x, 100.0) && approx(first_corner(&s).y, 100.0));
    }

    #[test]
    fn select_drag_below_threshold_does_not_move() {
        let mut s = session();
        draw_rect(&mut s, (100.0, 100.0), (200.0, 150.0));
        let before = s.rects();
        assert!(s.pointer_down(150.0, 120.0, 0, false));
        assert!(s.pointer_move(151.0, 121.0)); // 1.4 px: below the 3 px threshold
        assert!(s.pointer_up(151.0, 121.0));
        assert_eq!(s.rects(), before);
    }

    #[test]
    fn cancelled_move_leaves_the_draw_list_bitwise_equal() {
        let mut s = session();
        draw_rect(&mut s, (100.0, 100.0), (200.0, 150.0));
        let before = s.rects();
        assert!(s.pointer_down(150.0, 120.0, 0, false));
        assert!(s.pointer_move(190.0, 160.0));
        s.cancel();
        assert!(s.pointer_up(190.0, 160.0));
        assert_eq!(s.rects(), before);
    }

    #[test]
    fn redo_restores_the_same_node_id() {
        let mut s = session();
        draw_rect(&mut s, (100.0, 100.0), (200.0, 150.0));
        let id = s.rects()[0].id;
        assert!(id.is_some());
        assert!(s.undo());
        assert!(s.redo());
        assert_eq!(s.rects()[0].id, id);
    }

    #[test]
    fn edited_document_saves_and_reloads_stably() {
        let mut s = session();
        draw_rect(&mut s, (100.0, 100.0), (200.0, 150.0));
        draw_rect(&mut s, (300.0, 100.0), (340.0, 180.0));
        drag(&mut s, (150.0, 120.0), (170.0, 130.0));
        let saved = s.save_json().expect("a valid document saves");
        let reloaded = gip_document::from_json(&saved).expect("saved text reloads");
        let resaved = gip_document::to_json(&reloaded).expect("reloaded document validates");
        assert_eq!(saved, resaved, "save → load → save is stable");
    }
}
