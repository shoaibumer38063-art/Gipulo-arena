//! The interaction state machine. See the crate docs for the contract.

use crate::group::{axis_aligned, group_box, group_frame, resize_group, rotate_group};
use crate::snap::{
    Guide, SNAP_ACTIVATE_PX, SNAP_RELEASE_PX, SnapHold, candidate_boxes, guide_for, lines,
    snap_value, transform_box,
};
use gip_coords::viewport::screen_to_viewport;
use gip_coords::{Camera, Mat2, Vec2, ViewportRect, normalize_deg};
use gip_document::{Document, NodeId, Transform};
use gip_geometry::trs_of;

/// Pixels the pointer must travel after pressing before a gesture becomes a drag (CAN-INPUT-2).
pub const DRAG_THRESHOLD_PX: f64 = 3.0;

/// Screen-space hit tolerance for zero-size objects (CAN-GEOM-5). [PROPOSED] value.
pub const HIT_TOLERANCE_PX: f64 = 8.0;

/// Handle activation half-size in screen pixels: a 12 px clickable square around the 8 px visual (D2).
pub const HANDLE_SLOP_PX: f64 = 6.0;

/// Outer radius of a rotate zone, in screen pixels, measured from the corner (CAN-TR-4 [DS]: r≈14,
/// with the zone just outside the corner handle).
pub const ROTATE_RADIUS_PX: f64 = 20.0;

/// Rotation increment with Shift (CAN-TR-4).
pub const ROTATE_STEP_DEG: f64 = 15.0;

/// The active tool (CAN-MODE-1). Pan is a gesture (middle button), not a tool.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Tool {
    /// Click and drag to select, move, resize, and rotate objects.
    Select,
    /// Drag to create a rectangle.
    Rectangle,
}

/// The camera and canvas placement the machine converts pointer input with.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct View {
    /// Current camera.
    pub camera: Camera,
    /// Canvas placement in client pixels.
    pub rect: ViewportRect,
}

/// Modifier keys that change a gesture while it runs (CAN-TR-3, CAN-TR-4).
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct Modifiers {
    /// Shift: proportional corner resize; 15° rotation steps.
    pub shift: bool,
    /// Alt / ⌥: resize from the center.
    pub alt: bool,
    /// ⌘ / Ctrl: turns snapping off for this gesture (input-system §modifiers).
    pub snap_off: bool,
}

/// The eight resize handles, in draw-list order.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Handle {
    /// Top-left corner.
    NW,
    /// Top edge midpoint.
    N,
    /// Top-right corner.
    NE,
    /// Right edge midpoint.
    E,
    /// Bottom-right corner.
    SE,
    /// Bottom edge midpoint.
    S,
    /// Bottom-left corner.
    SW,
    /// Left edge midpoint.
    W,
}

impl Handle {
    /// All handles in the order the boundary reports them.
    pub const ALL: [Handle; 8] = [
        Handle::NW,
        Handle::N,
        Handle::NE,
        Handle::E,
        Handle::SE,
        Handle::S,
        Handle::SW,
        Handle::W,
    ];

    /// Which edges the handle moves, as `(-1 | 0 | 1, -1 | 0 | 1)` for the object's (x, y).
    #[must_use]
    pub fn axes(self) -> (i8, i8) {
        match self {
            Handle::NW => (-1, -1),
            Handle::N => (0, -1),
            Handle::NE => (1, -1),
            Handle::E => (1, 0),
            Handle::SE => (1, 1),
            Handle::S => (0, 1),
            Handle::SW => (-1, 1),
            Handle::W => (-1, 0),
        }
    }
}

/// The axis a flip mirrors across (CAN-TR-5).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Axis {
    /// Mirror left ↔ right.
    Horizontal,
    /// Mirror top ↔ bottom.
    Vertical,
}

/// An axis-aligned box `(x, y, w, h)` in world units.
pub type Rect4 = (f64, f64, f64, f64);

/// The mapping from the unit square to world space for a transform (includes rotation and flips).
#[must_use]
pub fn unit_mapping(t: &Transform) -> Mat2 {
    Mat2::from_trs(trs_of(t))
}

/// World center of a transform's box.
#[must_use]
pub fn center_of(t: &Transform) -> Vec2 {
    unit_mapping(t).apply(Vec2::new(0.5, 0.5))
}

/// World positions of the eight handles, in [`Handle::ALL`] order. Positions follow rotation.
#[must_use]
pub fn handle_points(t: &Transform) -> [Vec2; 8] {
    let m = unit_mapping(t);
    let mut out = [Vec2::ZERO; 8];
    for (slot, handle) in out.iter_mut().zip(Handle::ALL) {
        let (hx, vy) = handle.axes();
        *slot = m.apply(Vec2::new(
            (f64::from(hx) + 1.0) / 2.0,
            (f64::from(vy) + 1.0) / 2.0,
        ));
    }
    out
}

/// The single node that shows handles: exactly one selected, visible, and unlocked. Rotation and
/// flips are allowed; a flipped object resizes in its mirrored local frame (task 023).
#[must_use]
pub fn resize_target(doc: &Document, selection: &[NodeId]) -> Option<NodeId> {
    if selection.len() != 1 {
        return None;
    }
    let id = selection[0];
    let node = doc.nodes().get(&id)?;
    (node.visible && !node.locked).then_some(id)
}

/// A node's transform with a gesture preview applied (world positions for moves).
#[must_use]
pub fn effective_transform(preview: &Preview, id: NodeId, mut t: Transform) -> Transform {
    match preview {
        Preview::Moving { items } => {
            if let Some((_, (x, y))) = items.iter().find(|(n, _)| *n == id) {
                t.x = *x;
                t.y = *y;
            }
        }
        Preview::Transforming { node, transform } if *node == id => t = *transform,
        Preview::TransformingMany { items } => {
            if let Some((_, tr)) = items.iter().find(|(n, _)| *n == id) {
                t = *tr;
            }
        }
        _ => {}
    }
    t
}

/// The frame a selection is edited in. One object: the object itself (rotation and flips kept).
/// Several objects: the axis-aligned group box. `resizable` says whether handles are shown and
/// resize works: always for one object; for a group only when every member is axis-aligned.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SelectionFrame {
    /// The frame transform (for a group: unrotated, unflipped, the group box).
    pub transform: Transform,
    /// Whether resize handles are offered on this frame.
    pub resizable: bool,
}

/// The selection frame, with a gesture preview applied. `None` when nothing editable is selected
/// (empty, a hidden or locked member, or a missing node).
#[must_use]
pub fn selection_frame(
    doc: &Document,
    selection: &[NodeId],
    preview: &Preview,
) -> Option<SelectionFrame> {
    match selection {
        [id] => {
            let node = doc.nodes().get(id)?;
            (node.visible && !node.locked).then(|| SelectionFrame {
                transform: effective_transform(preview, *id, node.transform),
                resizable: true,
            })
        }
        ids if ids.len() >= 2 => {
            let mut ts = Vec::with_capacity(ids.len());
            for id in ids {
                let node = doc.nodes().get(id)?;
                if !node.visible || node.locked {
                    return None;
                }
                ts.push(effective_transform(preview, *id, node.transform));
            }
            Some(SelectionFrame {
                transform: group_frame(group_box(&ts)),
                resizable: ts.iter().all(axis_aligned),
            })
        }
        _ => None,
    }
}

/// The box after a handle drag in an unrotated frame (Slice 1 math). Start and total delta only
/// (CAN-TR-2). Dimensions clamp at zero [D1]. `proportional` (Shift) keeps the aspect on corners.
/// `from_center` (Alt) keeps the center fixed.
#[must_use]
pub fn resize_box(start: Rect4, handle: Handle, delta: Vec2, mods: Modifiers) -> Rect4 {
    let (x0, y0, w0, h0) = start;
    let (hx, vy) = handle.axes();
    let scale = if mods.alt { 2.0 } else { 1.0 };
    let mut w = w0;
    let mut h = h0;
    if hx != 0 {
        w = (w0 + f64::from(hx) * delta.x * scale).max(0.0);
    }
    if vy != 0 {
        h = (h0 + f64::from(vy) * delta.y * scale).max(0.0);
    }
    let corner = hx != 0 && vy != 0;
    if mods.shift && corner && w0 > 0.0 && h0 > 0.0 {
        let s = (w / w0).max(h / h0);
        w = w0 * s;
        h = h0 * s;
    }
    let x = if mods.alt {
        x0 + w0 / 2.0 - w / 2.0
    } else if hx < 0 {
        x0 + w0 - w
    } else {
        x0
    };
    let y = if mods.alt {
        y0 + h0 / 2.0 - h / 2.0
    } else if vy < 0 {
        y0 + h0 - h
    } else {
        y0
    };
    (x, y, w, h)
}

/// Repositions `t` so that its center lands on `center`. Used by rotation and flips, which
/// keep the object's center fixed.
#[must_use]
pub fn keep_center(mut t: Transform, center: Vec2) -> Transform {
    t.x = 0.0;
    t.y = 0.0;
    let offset = center_of(&t);
    t.x = center.x - offset.x;
    t.y = center.y - offset.y;
    t
}

/// Resize a transform by a handle drag. The delta is in world units. For a rotated object the
/// delta is taken in the object's own frame, and the opposite anchor (the center with Alt) stays
/// fixed in world space (CAN-TR-3). Flipped objects are not resized (see [`resize_target`]).
#[must_use]
pub fn resize_transform(
    start: Transform,
    handle: Handle,
    delta: Vec2,
    mods: Modifiers,
) -> Transform {
    let (sin, cos) = start.rotation_deg.to_radians().sin_cos();
    // Inverse rotation: the delta in the object's unrotated frame.
    let local = Vec2::new(
        cos * delta.x + sin * delta.y,
        -sin * delta.x + cos * delta.y,
    );
    // A flip mirrors the local axis: the handle's unit-space edge moves in the opposite direction
    // within the positive (unflipped) box. The anchor below stays in unit space, so it is unchanged.
    let sign_x = if start.flip_h { -1.0 } else { 1.0 };
    let sign_y = if start.flip_v { -1.0 } else { 1.0 };
    let local = Vec2::new(local.x * sign_x, local.y * sign_y);
    let (_, _, w, h) = resize_box((0.0, 0.0, start.w, start.h), handle, local, mods);
    let (hx, vy) = handle.axes();
    // Anchor fraction inside the box: the opposite edge (or the center).
    let frac = |v: i8| -> f64 {
        if mods.alt || v == 0 {
            0.5
        } else if v > 0 {
            0.0
        } else {
            1.0
        }
    };
    let (fx, fy) = (frac(hx), frac(vy));
    let anchor_world = unit_mapping(&start).apply(Vec2::new(fx, fy));
    let mut t = start;
    t.w = w;
    t.h = h;
    t.x = 0.0;
    t.y = 0.0;
    let offset = unit_mapping(&t).apply(Vec2::new(fx, fy));
    t.x = anchor_world.x - offset.x;
    t.y = anchor_world.y - offset.y;
    t
}

/// Rotate a transform about its center by the pointer's angular change from `start_angle`
/// (radians, measured about `center`). Shift snaps the change to 15° steps (CAN-TR-4). The
/// result is normalized to −180…180° [D2].
#[must_use]
pub fn rotate_transform(
    start: Transform,
    center: Vec2,
    start_angle: f64,
    pointer: Vec2,
    shift: bool,
) -> Transform {
    let pointer_angle = (pointer.y - center.y).atan2(pointer.x - center.x);
    let mut delta = normalize_deg((pointer_angle - start_angle).to_degrees());
    if shift {
        delta = (delta / ROTATE_STEP_DEG).round() * ROTATE_STEP_DEG;
    }
    let mut t = start;
    t.rotation_deg = normalize_deg(start.rotation_deg + delta);
    keep_center(t, center)
}

/// Flip a transform about its center (CAN-TR-5). The flip flag toggles; the box stays centered.
#[must_use]
pub fn flip_transform(start: Transform, axis: Axis) -> Transform {
    let center = center_of(&start);
    let mut t = start;
    match axis {
        Axis::Horizontal => t.flip_h = !t.flip_h,
        Axis::Vertical => t.flip_v = !t.flip_v,
    }
    keep_center(t, center)
}

/// One node's move, as a history-ready pair of positions (world units).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct MoveItem {
    /// Node moved.
    pub node: NodeId,
    /// Position at gesture start.
    pub from: (f64, f64),
    /// Position at gesture end.
    pub to: (f64, f64),
}

/// One node's transform change, for resize, rotate, and flip.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct TransformItem {
    /// Node changed.
    pub node: NodeId,
    /// Transform at gesture start.
    pub from: Transform,
    /// Transform at gesture end.
    pub to: Transform,
}

/// A document change the machine asks the session to commit as ONE history entry.
#[derive(Debug, Clone, PartialEq)]
pub enum Commit {
    /// Create a rectangle at this world rect (normalized, positive size).
    CreateRect {
        /// Left.
        x: f64,
        /// Top.
        y: f64,
        /// Width (> 0).
        w: f64,
        /// Height (> 0).
        h: f64,
    },
    /// Move one or more nodes.
    Move {
        /// Per-node positions.
        items: Vec<MoveItem>,
    },
    /// Resize, rotate, or flip one or more nodes.
    Transform {
        /// Per-node transforms.
        items: Vec<TransformItem>,
    },
    /// Delete nodes.
    Delete {
        /// Nodes to remove.
        nodes: Vec<NodeId>,
    },
}

/// What a pointer event asks the session to do.
#[derive(Debug, Clone, PartialEq)]
pub enum Effect {
    /// Pan the camera by a client-pixel delta (CAN-CAM-1: never a history entry).
    Pan {
        /// Horizontal delta.
        dx: f64,
        /// Vertical delta.
        dy: f64,
    },
    /// Commit a document change as one history entry.
    Commit(Commit),
}

/// Transient geometry the shell should draw while a gesture is in progress.
#[derive(Debug, Clone, PartialEq)]
pub enum Preview {
    /// No gesture preview.
    None,
    /// Several nodes being resized or rotated, at their previewed transforms.
    TransformingMany {
        /// Previewed transforms, one per node.
        items: Vec<(NodeId, Transform)>,
    },
    /// A rectangle being drawn (world coordinates, normalized).
    Creating {
        /// Left.
        x: f64,
        /// Top.
        y: f64,
        /// Width.
        w: f64,
        /// Height.
        h: f64,
    },
    /// The marquee rectangle being dragged (world units, normalized).
    Marquee {
        /// Left.
        x: f64,
        /// Top.
        y: f64,
        /// Width.
        w: f64,
        /// Height.
        h: f64,
    },
    /// Nodes being dragged, at their previewed (not yet committed) positions.
    Moving {
        /// Previewed positions.
        items: Vec<(NodeId, (f64, f64))>,
    },
    /// A node being resized or rotated, at its previewed transform.
    Transforming {
        /// Node transformed.
        node: NodeId,
        /// Previewed transform.
        transform: Transform,
    },
}

/// What a press on a control does.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Control {
    /// Resize from this handle.
    Resize(Handle),
    /// Rotate about the center, starting from this corner.
    Rotate(Handle),
    /// Resize the group box from this handle (several objects).
    GroupResize(Handle),
    /// Rotate the group about its center, starting from this corner (several objects).
    GroupRotate(Handle),
}

#[derive(Debug, Clone, PartialEq)]
enum State {
    Idle,
    /// Button down, threshold not yet crossed. `control` is set when the press landed on a control.
    Pressed {
        client: Vec2,
        world: Vec2,
        control: Option<(NodeId, Control)>,
        /// Shift was held at press (additive selection).
        shift: bool,
        /// The press landed on empty canvas (no object under it).
        empty_press: bool,
    },
    /// Drag-selecting: a rectangle selects every object it touches (task 026).
    Marquee {
        start_world: Vec2,
        current_world: Vec2,
        /// Selection when the marquee started (restored on Escape).
        original: Vec<NodeId>,
        /// Shift was held: the marquee adds to `original`.
        shift: bool,
    },
    /// Dragging selected nodes.
    Moving {
        start_world: Vec2,
        items: Vec<(NodeId, (f64, f64))>,
        delta: Vec2,
    },
    /// Dragging a resize handle.
    Resizing {
        node: NodeId,
        start: Transform,
        handle: Handle,
        start_world: Vec2,
        current: Transform,
    },
    /// Dragging a rotate zone.
    Rotating {
        node: NodeId,
        start: Transform,
        center: Vec2,
        start_angle: f64,
        current: Transform,
    },
    /// Resizing several objects through the group box.
    GroupResizing {
        starts: Vec<(NodeId, Transform)>,
        group: (f64, f64, f64, f64),
        handle: Handle,
        start_world: Vec2,
        current: Vec<Transform>,
    },
    /// Rotating several objects about the group center.
    GroupRotating {
        starts: Vec<(NodeId, Transform)>,
        pivot: Vec2,
        start_angle: f64,
        current: Vec<Transform>,
    },
    /// Drawing a rectangle.
    Creating {
        start_world: Vec2,
        current_world: Vec2,
    },
    /// Middle-button pan.
    Panning {
        last_client: Vec2,
    },
}

/// The interaction state machine.
#[derive(Debug, Clone, PartialEq)]
pub struct Interaction {
    tool: Tool,
    state: State,
    selection: Vec<NodeId>,
    hover: Option<NodeId>,
    cursor: &'static str,
    snap_hold: SnapHold,
    guides: Vec<Guide>,
}

impl Default for Interaction {
    fn default() -> Self {
        Self::new()
    }
}

impl Interaction {
    /// Idle machine with the Select tool and nothing selected.
    #[must_use]
    pub fn new() -> Self {
        Self {
            tool: Tool::Select,
            state: State::Idle,
            selection: Vec::new(),
            hover: None,
            cursor: "default",
            snap_hold: SnapHold::default(),
            guides: Vec::new(),
        }
    }

    /// Snap guides for the gesture in progress (world units). Empty when nothing is snapped.
    #[must_use]
    pub fn guides(&self) -> Vec<Guide> {
        self.guides.clone()
    }

    fn clear_snap(&mut self) {
        self.snap_hold = SnapHold::default();
        self.guides.clear();
    }

    /// Active tool.
    #[must_use]
    pub fn tool(&self) -> Tool {
        self.tool
    }

    /// Switches tool. Ignored while a gesture is in progress; returns whether it changed.
    pub fn set_tool(&mut self, tool: Tool) -> bool {
        if self.state != State::Idle {
            return false;
        }
        self.tool = tool;
        self.cursor = tool_cursor(tool);
        true
    }

    /// Selected nodes in selection order.
    #[must_use]
    pub fn selection(&self) -> &[NodeId] {
        &self.selection
    }

    /// Node under the pointer while idle.
    #[must_use]
    pub fn hover(&self) -> Option<NodeId> {
        self.hover
    }

    /// The cursor the shell should show: the action the next press would start (CAN-CUR-1).
    #[must_use]
    pub fn cursor(&self) -> &'static str {
        self.cursor
    }

    /// Whether a gesture is in progress.
    #[must_use]
    pub fn is_gesturing(&self) -> bool {
        self.state != State::Idle
    }

    /// Current gesture preview for painting.
    #[must_use]
    pub fn preview(&self) -> Preview {
        match &self.state {
            State::Marquee {
                start_world,
                current_world,
                ..
            } => {
                let (x, y, w, h) = normalized(*start_world, *current_world);
                Preview::Marquee { x, y, w, h }
            }
            State::Creating {
                start_world,
                current_world,
            } => {
                let (x, y, w, h) = normalized(*start_world, *current_world);
                Preview::Creating { x, y, w, h }
            }
            State::Moving { items, delta, .. } => Preview::Moving {
                items: items
                    .iter()
                    .map(|(id, (x, y))| (*id, (x + delta.x, y + delta.y)))
                    .collect(),
            },
            State::Resizing { node, current, .. } | State::Rotating { node, current, .. } => {
                Preview::Transforming {
                    node: *node,
                    transform: *current,
                }
            }
            State::GroupResizing {
                starts, current, ..
            }
            | State::GroupRotating {
                starts, current, ..
            } => Preview::TransformingMany {
                items: starts
                    .iter()
                    .zip(current)
                    .map(|((id, _), t)| (*id, *t))
                    .collect(),
            },
            _ => Preview::None,
        }
    }

    /// Replaces the selection (used after the session commits a create).
    pub fn set_selection(&mut self, ids: &[NodeId]) {
        self.selection = ids.to_vec();
    }

    /// Drops selection and hover entries for nodes that no longer exist (after undo or delete).
    pub fn prune(&mut self, doc: &Document) {
        let nodes = doc.nodes();
        self.selection.retain(|id| nodes.contains_key(id));
        if self.hover.is_some_and(|id| !nodes.contains_key(&id)) {
            self.hover = None;
        }
    }

    /// Pointer pressed. Returns effects (usually none; pan starts here, selection changes here).
    pub fn pointer_down(
        &mut self,
        doc: &Document,
        view: View,
        client: Vec2,
        button: u8,
        shift: bool,
    ) -> Vec<Effect> {
        if self.state != State::Idle {
            return Vec::new();
        }
        self.clear_snap();
        if button == 1 {
            self.state = State::Panning {
                last_client: client,
            };
            self.cursor = "grabbing";
            return Vec::new();
        }
        if button != 0 {
            return Vec::new();
        }
        let world = to_world(view, client);
        match self.tool {
            Tool::Rectangle => {
                self.state = State::Pressed {
                    client,
                    world,
                    control: None,
                    shift,
                    empty_press: false,
                };
            }
            Tool::Select => {
                if let Some(control) = self.control_at(doc, view, world) {
                    // A press on a handle or rotate zone leaves the selection alone.
                    self.state = State::Pressed {
                        client,
                        world,
                        control: Some(control),
                        shift,
                        empty_press: false,
                    };
                    return Vec::new();
                }
                let hit = hit_test(doc, view, world);
                // Empty canvas: the selection is only changed once the drag becomes a marquee,
                // or on release without a drag (see pointer_up).
                if hit.is_some() {
                    self.apply_click_selection(hit, shift);
                }
                self.state = State::Pressed {
                    client,
                    world,
                    control: None,
                    shift,
                    empty_press: hit.is_none(),
                };
            }
        }
        Vec::new()
    }

    /// Pointer moved (buttons not required; hover is tracked while idle).
    pub fn pointer_move(
        &mut self,
        doc: &Document,
        view: View,
        client: Vec2,
        mods: Modifiers,
    ) -> Vec<Effect> {
        match self.state.clone() {
            State::Idle => {
                let world = to_world(view, client);
                self.hover = if self.tool == Tool::Select {
                    hit_test(doc, view, world)
                } else {
                    None
                };
                self.cursor = self.hover_cursor(doc, view, world);
                Vec::new()
            }
            State::Pressed {
                client: press_client,
                world: press_world,
                control,
                shift: press_shift,
                empty_press,
            } => {
                if client.distance_sq(press_client).sqrt() < DRAG_THRESHOLD_PX {
                    return Vec::new();
                }
                let world = to_world(view, client);
                if let Some((node, control)) = control {
                    self.start_control_gesture(doc, node, control, press_world);
                    let world = self.snapped_control_world(doc, view.camera.zoom, world, mods);
                    self.update_control(world, mods);
                    return Vec::new();
                }
                match self.tool {
                    Tool::Rectangle => {
                        self.state = State::Creating {
                            start_world: press_world,
                            current_world: world,
                        };
                        self.cursor = "crosshair";
                    }
                    Tool::Select => {
                        // A drag from a selected node moves the selection. A drag from empty
                        // canvas is a marquee. A drag from any other object does nothing.
                        let items = self.selected_positions(doc);
                        if empty_press {
                            let original = self.selection.clone();
                            if !press_shift {
                                self.selection.clear();
                            }
                            self.state = State::Marquee {
                                start_world: press_world,
                                current_world: world,
                                original,
                                shift: press_shift,
                            };
                            self.update_marquee(doc, world);
                        } else if items.is_empty()
                            || !self.pressed_on_selection(doc, view, press_world)
                        {
                            self.state = State::Idle;
                        } else {
                            self.state = State::Moving {
                                start_world: press_world,
                                items,
                                delta: Vec2::ZERO,
                            };
                            self.cursor = "move";
                            self.update_move(doc, view.camera.zoom, world, press_world, mods);
                        }
                    }
                }
                Vec::new()
            }
            State::Moving { start_world, .. } => {
                self.update_move(
                    doc,
                    view.camera.zoom,
                    to_world(view, client),
                    start_world,
                    mods,
                );
                Vec::new()
            }
            State::Resizing { .. }
            | State::Rotating { .. }
            | State::GroupResizing { .. }
            | State::GroupRotating { .. } => {
                let world =
                    self.snapped_control_world(doc, view.camera.zoom, to_world(view, client), mods);
                self.update_control(world, mods);
                Vec::new()
            }
            State::Marquee { .. } => {
                self.update_marquee(doc, to_world(view, client));
                Vec::new()
            }
            State::Creating { start_world, .. } => {
                let raw = to_world(view, client);
                let current_world =
                    self.snapped_create_world(doc, view.camera.zoom, start_world, raw, mods);
                self.state = State::Creating {
                    start_world,
                    current_world,
                };
                Vec::new()
            }
            State::Panning { last_client } => {
                self.state = State::Panning {
                    last_client: client,
                };
                vec![Effect::Pan {
                    dx: client.x - last_client.x,
                    dy: client.y - last_client.y,
                }]
            }
        }
    }

    /// Pointer released. Commits the gesture if it changed the document.
    pub fn pointer_up(&mut self, _doc: &Document, _view: View, _client: Vec2) -> Vec<Effect> {
        self.clear_snap();
        let state = std::mem::replace(&mut self.state, State::Idle);
        self.cursor = tool_cursor(self.tool);
        match state {
            State::Moving { items, delta, .. } => {
                if delta == Vec2::ZERO {
                    return Vec::new();
                }
                let items = items
                    .into_iter()
                    .map(|(node, from)| MoveItem {
                        node,
                        from,
                        to: (from.0 + delta.x, from.1 + delta.y),
                    })
                    .collect();
                vec![Effect::Commit(Commit::Move { items })]
            }
            State::Resizing {
                node,
                start,
                current,
                ..
            }
            | State::Rotating {
                node,
                start,
                current,
                ..
            } => {
                if current == start {
                    Vec::new()
                } else {
                    vec![Effect::Commit(Commit::Transform {
                        items: vec![TransformItem {
                            node,
                            from: start,
                            to: current,
                        }],
                    })]
                }
            }
            State::GroupResizing {
                starts, current, ..
            }
            | State::GroupRotating {
                starts, current, ..
            } => {
                let mut items = Vec::new();
                for ((node, from), to) in starts.iter().zip(current.iter()) {
                    if from != to {
                        items.push(TransformItem {
                            node: *node,
                            from: *from,
                            to: *to,
                        });
                    }
                }
                if items.is_empty() {
                    Vec::new()
                } else {
                    vec![Effect::Commit(Commit::Transform { items })]
                }
            }
            State::Creating {
                start_world,
                current_world,
            } => {
                let (x, y, w, h) = normalized(start_world, current_world);
                if w > 0.0 && h > 0.0 {
                    vec![Effect::Commit(Commit::CreateRect { x, y, w, h })]
                } else {
                    Vec::new()
                }
            }
            State::Pressed {
                control: None,
                shift: false,
                empty_press: true,
                ..
            } if self.tool == Tool::Select => {
                // A click on empty canvas (no drag) clears the selection.
                self.selection.clear();
                Vec::new()
            }
            _ => Vec::new(),
        }
    }

    /// Cancels the gesture in progress without a history entry (CAN-MODE-4).
    pub fn cancel(&mut self) {
        self.clear_snap();
        if let State::Marquee { original, .. } = &self.state {
            self.selection = original.clone();
        }
        self.state = State::Idle;
        self.cursor = tool_cursor(self.tool);
    }

    /// Escape: cancels the gesture if one is running, otherwise clears the selection.
    pub fn escape(&mut self) {
        if self.is_gesturing() {
            self.cancel();
        } else {
            self.selection.clear();
        }
    }

    /// Delete the selection: returns the commit when the machine is idle and something is
    /// selected. The session clears the selection after the commit succeeds.
    pub fn delete_selection(&mut self) -> Vec<Effect> {
        if self.is_gesturing() || self.selection.is_empty() {
            return Vec::new();
        }
        vec![Effect::Commit(Commit::Delete {
            nodes: self.selection.clone(),
        })]
    }

    /// Flip the selection about each object's center (CAN-TR-5). One history entry. Returns
    /// nothing while a gesture runs or when nothing is selected.
    pub fn flip_selection(&mut self, doc: &Document, axis: Axis) -> Vec<Effect> {
        if self.is_gesturing() || self.selection.is_empty() {
            return Vec::new();
        }
        let items: Vec<TransformItem> = self
            .selection
            .iter()
            .filter_map(|id| {
                let from = doc.nodes().get(id)?.transform;
                (!doc.nodes().get(id)?.locked).then(|| TransformItem {
                    node: *id,
                    from,
                    to: flip_transform(from, axis),
                })
            })
            .collect();
        if items.is_empty() {
            Vec::new()
        } else {
            vec![Effect::Commit(Commit::Transform { items })]
        }
    }

    /// Moves the marquee corner and recomputes the selection from the touched objects.
    fn update_marquee(&mut self, doc: &Document, world: Vec2) {
        let State::Marquee {
            start_world,
            current_world,
            original,
            shift,
        } = &mut self.state
        else {
            return;
        };
        *current_world = world;
        let (x, y, w, h) = normalized(*start_world, world);
        let hits = marquee_hits(doc, (x, y, w, h));
        let mut next: Vec<NodeId> = if *shift { original.clone() } else { Vec::new() };
        for id in hits {
            if !next.contains(&id) {
                next.push(id);
            }
        }
        self.selection = next;
    }

    fn apply_click_selection(&mut self, hit: Option<NodeId>, shift: bool) {
        match (hit, shift) {
            (Some(id), true) => {
                if let Some(pos) = self.selection.iter().position(|s| *s == id) {
                    self.selection.remove(pos);
                } else {
                    self.selection.push(id);
                }
            }
            (Some(id), false) => {
                if !self.selection.contains(&id) {
                    self.selection = vec![id];
                }
            }
            (None, false) => self.selection.clear(),
            (None, true) => {}
        }
    }

    fn pressed_on_selection(&self, doc: &Document, view: View, press_world: Vec2) -> bool {
        hit_test(doc, view, press_world).is_some_and(|id| self.selection.contains(&id))
    }

    fn selected_positions(&self, doc: &Document) -> Vec<(NodeId, (f64, f64))> {
        self.selection
            .iter()
            .filter_map(|id| {
                doc.nodes()
                    .get(id)
                    .filter(|n| !n.locked)
                    .map(|n| (*id, (n.transform.x, n.transform.y)))
            })
            .collect()
    }

    /// The control under `world` for the single resize target: corner and edge handles first
    /// (CAN-HOVER-2), then rotate zones just outside the corners.
    fn control_at(&self, doc: &Document, view: View, world: Vec2) -> Option<(NodeId, Control)> {
        let frame = selection_frame(doc, &self.selection, &Preview::None)?;
        let single = self.selection.len() == 1;
        // For a group, the NodeId is a placeholder (the first member); the controls act on the group.
        let id = *self.selection.first()?;
        let t = frame.transform;
        let zoom = view.camera.zoom;
        let points = handle_points(&t);
        let slop = HANDLE_SLOP_PX / zoom;
        let handle_hit = if frame.resizable {
            Handle::ALL
                .iter()
                .zip(points)
                .find(|(_, p)| (p.x - world.x).abs() <= slop && (p.y - world.y).abs() <= slop)
                .map(|(h, _)| *h)
        } else {
            None
        };
        if let Some(handle) = handle_hit {
            let control = if single {
                Control::Resize(handle)
            } else {
                Control::GroupResize(handle)
            };
            return Some((id, control));
        }
        let center = center_of(&t);
        let (inner, outer) = (HANDLE_SLOP_PX / zoom, ROTATE_RADIUS_PX / zoom);
        for corner in [Handle::NW, Handle::NE, Handle::SE, Handle::SW] {
            let p = points[Handle::ALL.iter().position(|h| *h == corner)?];
            let off = world - p;
            let dist = off.length_sq().sqrt();
            let outward = (p - center).dot(off) > 0.0;
            if dist > inner && dist <= outer && outward {
                let control = if single {
                    Control::Rotate(corner)
                } else {
                    Control::GroupRotate(corner)
                };
                return Some((id, control));
            }
        }
        None
    }

    /// Cursor for hovering a control or the body while idle.
    fn hover_cursor(&self, doc: &Document, view: View, world: Vec2) -> &'static str {
        match self.control_at(doc, view, world) {
            Some((id, Control::Resize(handle))) => doc
                .nodes()
                .get(&id)
                .map_or("default", |n| resize_cursor(&n.transform, handle)),
            Some((_, Control::GroupResize(handle))) => {
                selection_frame(doc, &self.selection, &Preview::None)
                    .map_or("default", |f| resize_cursor(&f.transform, handle))
            }
            Some((_, Control::Rotate(_) | Control::GroupRotate(_))) => "grab",
            None => {
                if self.tool == Tool::Rectangle {
                    "crosshair"
                } else if hit_test(doc, view, world).is_some() {
                    "move"
                } else {
                    "default"
                }
            }
        }
    }

    fn start_control_gesture(
        &mut self,
        doc: &Document,
        node: NodeId,
        control: Control,
        press_world: Vec2,
    ) {
        match control {
            Control::Resize(handle) => {
                let Some(start) = doc.nodes().get(&node).map(|n| n.transform) else {
                    self.state = State::Idle;
                    return;
                };
                self.cursor = resize_cursor(&start, handle);
                self.state = State::Resizing {
                    node,
                    start,
                    handle,
                    start_world: press_world,
                    current: start,
                };
            }
            Control::Rotate(_) => {
                let Some(start) = doc.nodes().get(&node).map(|n| n.transform) else {
                    self.state = State::Idle;
                    return;
                };
                self.cursor = "grabbing";
                let center = center_of(&start);
                self.state = State::Rotating {
                    node,
                    start,
                    center,
                    start_angle: (press_world.y - center.y).atan2(press_world.x - center.x),
                    current: start,
                };
            }
            Control::GroupResize(handle) => {
                let starts = self.group_starts(doc);
                if starts.is_empty() {
                    self.state = State::Idle;
                    return;
                }
                let ts: Vec<Transform> = starts.iter().map(|(_, t)| *t).collect();
                let group = group_box(&ts);
                self.cursor = resize_cursor(&group_frame(group), handle);
                self.state = State::GroupResizing {
                    starts,
                    group,
                    handle,
                    start_world: press_world,
                    current: ts,
                };
            }
            Control::GroupRotate(_) => {
                let starts = self.group_starts(doc);
                if starts.is_empty() {
                    self.state = State::Idle;
                    return;
                }
                let ts: Vec<Transform> = starts.iter().map(|(_, t)| *t).collect();
                let pivot = center_of(&group_frame(group_box(&ts)));
                self.cursor = "grabbing";
                self.state = State::GroupRotating {
                    starts,
                    pivot,
                    start_angle: (press_world.y - pivot.y).atan2(press_world.x - pivot.x),
                    current: ts,
                };
            }
        }
    }

    /// The selected nodes' start transforms, in selection order.
    fn group_starts(&self, doc: &Document) -> Vec<(NodeId, Transform)> {
        self.selection
            .iter()
            .filter_map(|id| doc.nodes().get(id).map(|n| (*id, n.transform)))
            .collect()
    }

    fn update_move(
        &mut self,
        doc: &Document,
        zoom: f64,
        world: Vec2,
        start_world: Vec2,
        mods: Modifiers,
    ) {
        let raw = world - start_world;
        let State::Moving { items, .. } = &self.state else {
            return;
        };
        let moved: Vec<Transform> = items
            .iter()
            .filter_map(|(id, (x, y))| {
                doc.nodes().get(id).map(|n| Transform {
                    x: x + raw.x,
                    y: y + raw.y,
                    ..n.transform
                })
            })
            .collect();
        let ids: Vec<NodeId> = items.iter().map(|(id, _)| *id).collect();
        let mut delta = raw;
        self.guides.clear();
        if mods.snap_off {
            self.snap_hold = SnapHold::default();
        } else if !moved.is_empty() {
            let moving = group_box(&moved);
            let boxes = candidate_boxes(doc, &ids);
            let dx = self.snap_axis(true, &lines(moving.0, moving.2), &boxes, zoom, moving);
            let dy = self.snap_axis(false, &lines(moving.1, moving.3), &boxes, zoom, moving);
            delta = Vec2::new(raw.x + dx, raw.y + dy);
        }
        if let State::Moving { delta: d, .. } = &mut self.state {
            *d = delta;
        }
    }

    /// Snaps one axis of the gesture. `values` are the moving lines on that axis. Returns the
    /// correction to add, and updates the hold and the guides.
    fn snap_axis(
        &mut self,
        vertical: bool,
        values: &[f64],
        boxes: &[(f64, f64, f64, f64)],
        zoom: f64,
        moving: (f64, f64, f64, f64),
    ) -> f64 {
        let targets: Vec<f64> = boxes
            .iter()
            .flat_map(|b| {
                if vertical {
                    lines(b.0, b.2)
                } else {
                    lines(b.1, b.3)
                }
            })
            .collect();
        let hold = if vertical {
            self.snap_hold.x
        } else {
            self.snap_hold.y
        };
        let found = snap_value(
            values,
            &targets,
            SNAP_ACTIVATE_PX / zoom,
            SNAP_RELEASE_PX / zoom,
            hold,
        );
        let (correction, target) = match found {
            Some((d, t)) => (d, Some(t)),
            None => (0.0, None),
        };
        if vertical {
            self.snap_hold.x = target;
        } else {
            self.snap_hold.y = target;
        }
        if let Some(t) = target {
            self.guides.push(guide_for(vertical, t, moving, boxes));
        }
        correction
    }

    /// The control world point after snapping the moving edges of a resize (CAN-SNAP-7). Rotation
    /// does not snap. Snapping needs an axis-aligned frame, which every group has.
    fn snapped_control_world(
        &mut self,
        doc: &Document,
        zoom: f64,
        world: Vec2,
        mods: Modifiers,
    ) -> Vec2 {
        let (ids, start_box, flip, handle, start_world) = match &self.state {
            State::Resizing {
                node,
                start,
                handle,
                start_world,
                ..
            } => {
                if start.rotation_deg.abs() >= 1e-9 {
                    return world;
                }
                (
                    vec![*node],
                    transform_box(start),
                    (start.flip_h, start.flip_v),
                    *handle,
                    *start_world,
                )
            }
            State::GroupResizing {
                starts,
                group,
                handle,
                start_world,
                ..
            } => (
                starts.iter().map(|(id, _)| *id).collect(),
                *group,
                (false, false),
                *handle,
                *start_world,
            ),
            _ => return world,
        };
        if mods.snap_off {
            self.clear_snap();
            return world;
        }
        let raw = world - start_world;
        let boxes = candidate_boxes(doc, &ids);
        let (bx, by, bw, bh) = start_box;
        let moving = (bx + raw.x, by + raw.y, bw, bh);
        self.guides.clear();
        let (hx, vy) = handle.axes();
        let mut correction = Vec2::ZERO;
        if hx != 0 {
            // The grabbed vertical edge: world right unless flipped (a flip mirrors the unit axis).
            let right = (hx > 0) != flip.0;
            let edge = if right { bx + bw + raw.x } else { bx + raw.x };
            correction.x = self.snap_axis(true, &[edge], &boxes, zoom, moving);
        } else {
            self.snap_hold.x = None;
        }
        if vy != 0 {
            let bottom = (vy > 0) != flip.1;
            let edge = if bottom { by + bh + raw.y } else { by + raw.y };
            correction.y = self.snap_axis(false, &[edge], &boxes, zoom, moving);
        } else {
            self.snap_hold.y = None;
        }
        world + correction
    }

    /// The creation corner after snapping both axes to other objects (CAN-SNAP-7).
    fn snapped_create_world(
        &mut self,
        doc: &Document,
        zoom: f64,
        start_world: Vec2,
        raw: Vec2,
        mods: Modifiers,
    ) -> Vec2 {
        self.guides.clear();
        if mods.snap_off {
            self.clear_snap();
            return raw;
        }
        let boxes = candidate_boxes(doc, &[]);
        let moving = normalized(start_world, raw);
        let dx = self.snap_axis(true, &[raw.x], &boxes, zoom, moving);
        let dy = self.snap_axis(false, &[raw.y], &boxes, zoom, moving);
        Vec2::new(raw.x + dx, raw.y + dy)
    }

    fn update_control(&mut self, world: Vec2, mods: Modifiers) {
        match &mut self.state {
            State::Resizing {
                start,
                handle,
                start_world,
                current,
                ..
            } => {
                *current = resize_transform(*start, *handle, world - *start_world, mods);
            }
            State::Rotating {
                start,
                center,
                start_angle,
                current,
                ..
            } => {
                *current = rotate_transform(*start, *center, *start_angle, world, mods.shift);
            }
            State::GroupResizing {
                starts,
                group,
                handle,
                start_world,
                current,
            } => {
                let ts: Vec<Transform> = starts.iter().map(|(_, t)| *t).collect();
                *current = resize_group(&ts, *group, *handle, world - *start_world, mods);
            }
            State::GroupRotating {
                starts,
                pivot,
                start_angle,
                current,
            } => {
                let ts: Vec<Transform> = starts.iter().map(|(_, t)| *t).collect();
                *current = rotate_group(&ts, *pivot, *start_angle, world, mods.shift);
            }
            _ => {}
        }
    }
}

fn tool_cursor(tool: Tool) -> &'static str {
    match tool {
        Tool::Select => "default",
        Tool::Rectangle => "crosshair",
    }
}

/// Resize cursor for a handle, following the object's axes (CAN-CUR-2): the handle's direction in
/// world space, folded to a line (0–180°) and quantized to the eight-direction set.
#[must_use]
pub fn resize_cursor(t: &Transform, handle: Handle) -> &'static str {
    let m = unit_mapping(t);
    let (hx, vy) = handle.axes();
    let origin = m.apply(Vec2::new(0.5, 0.5));
    let tip = m.apply(Vec2::new(
        0.5 + f64::from(hx) * 0.5,
        0.5 + f64::from(vy) * 0.5,
    ));
    let d = tip - origin;
    let angle = d.y.atan2(d.x).to_degrees().rem_euclid(180.0);
    match ((angle / 45.0).round() as i64).rem_euclid(4) {
        0 => "ew-resize",
        1 => "nwse-resize",
        2 => "ns-resize",
        _ => "nesw-resize",
    }
}

/// Client point to world coordinates through the view camera.
fn to_world(view: View, client: Vec2) -> Vec2 {
    view.camera
        .viewport_to_world(screen_to_viewport(client, view.rect))
}

fn normalized(a: Vec2, b: Vec2) -> (f64, f64, f64, f64) {
    let x = a.x.min(b.x);
    let y = a.y.min(b.y);
    (x, y, (a.x - b.x).abs(), (a.y - b.y).abs())
}

/// Visible, unlocked nodes on the page whose world box touches `rect` (task 026, touch selects).
#[must_use]
pub fn marquee_hits(doc: &Document, rect: Rect4) -> Vec<NodeId> {
    let Some(page) = doc.pages().first() else {
        return Vec::new();
    };
    let (rx, ry, rw, rh) = rect;
    page.children
        .iter()
        .filter(|id| {
            let Some(node) = doc.nodes().get(id) else {
                return false;
            };
            if !node.visible || node.locked {
                return false;
            }
            let (x, y, w, h) = transform_box(&node.transform);
            x <= rx + rw && rx <= x + w && y <= ry + rh && ry <= y + h
        })
        .copied()
        .collect()
}

/// Topmost visible, unlocked node whose rectangle contains `world` (CAN-SEL-1, CAN-SEL-2).
/// A zero-width or zero-height object is hit within [`HIT_TOLERANCE_PX`] of its degenerate
/// outline (CAN-GEOM-5).
#[must_use]
pub fn hit_test(doc: &Document, view: View, world: Vec2) -> Option<NodeId> {
    let page = doc.pages().first()?;
    let nodes = doc.nodes();
    let tolerance_world = HIT_TOLERANCE_PX / view.camera.zoom;
    page.children.iter().rev().copied().find(|id| {
        let Some(node) = nodes.get(id) else {
            return false;
        };
        if !node.visible || node.locked {
            return false;
        }
        let t = node.transform;
        let m = unit_mapping(&t);
        if t.w > 0.0 && t.h > 0.0 {
            m.world_to_local(world)
                .is_some_and(|l| (0.0..=1.0).contains(&l.x) && (0.0..=1.0).contains(&l.y))
        } else {
            degenerate_distance(m, t.w > 0.0, t.h > 0.0, world) <= tolerance_world
        }
    })
}

/// Distance from `world` to the outline of a zero-size object. The unit square collapses along
/// each zero dimension, leaving a segment (or a point when both are zero).
fn degenerate_distance(m: Mat2, has_w: bool, has_h: bool, world: Vec2) -> f64 {
    let origin = m.apply(Vec2::new(0.0, 0.0));
    let far = match (has_w, has_h) {
        (false, false) => return origin.distance_sq(world).sqrt(),
        (false, true) => m.apply(Vec2::new(0.0, 1.0)),
        (true, false) => m.apply(Vec2::new(1.0, 0.0)),
        (true, true) => return f64::INFINITY,
    };
    let ab = far - origin;
    let len_sq = ab.length_sq();
    let t = if len_sq == 0.0 {
        0.0
    } else {
        ((world - origin).dot(ab) / len_sq).clamp(0.0, 1.0)
    };
    (origin + ab.scale(t)).distance_sq(world).sqrt()
}

#[cfg(test)]
mod transform_tests {
    use super::*;

    fn base() -> Transform {
        Transform {
            x: 100.0,
            y: 100.0,
            w: 100.0,
            h: 50.0,
            rotation_deg: 0.0,
            flip_h: false,
            flip_v: false,
        }
    }

    /// Point at `r` from the origin, `deg` degrees clockwise on screen (y down).
    fn polar(deg: f64, r: f64) -> Vec2 {
        let a = deg.to_radians();
        Vec2::new(r * a.cos(), r * a.sin())
    }

    fn close(a: Vec2, b: Vec2) -> bool {
        (a.x - b.x).abs() < 1e-9 && (a.y - b.y).abs() < 1e-9
    }

    #[test]
    fn rotation_follows_the_pointer_angle_about_the_center() {
        let start = base();
        let center = center_of(&start);
        let start_angle = 0.0;
        // Pointer at 37° about the center, no snap.
        let p = center + polar(37.0, 50.0);
        let t = rotate_transform(start, center, start_angle, p, false);
        assert!((t.rotation_deg - 37.0).abs() < 1e-9);
        assert!(close(center_of(&t), center), "center stays put");
    }

    #[test]
    fn shift_snaps_the_change_to_fifteen_degrees() {
        let start = base();
        let center = center_of(&start);
        let p = center + polar(37.0, 50.0);
        let t = rotate_transform(start, center, 0.0, p, true);
        assert!((t.rotation_deg - 30.0).abs() < 1e-9, "37° snaps to 30°");
    }

    #[test]
    fn rotation_is_normalized_into_minus_180_to_180() {
        let mut start = base();
        start.rotation_deg = 170.0;
        let center = center_of(&start);
        let p = center + polar(30.0, 50.0);
        let t = rotate_transform(start, center, 0.0, p, false);
        assert!(
            (t.rotation_deg - (-160.0)).abs() < 1e-9,
            "got {}",
            t.rotation_deg
        );
    }

    #[test]
    fn flip_twice_restores_the_transform_exactly() {
        let mut start = base();
        start.rotation_deg = 33.0;
        let once = flip_transform(start, Axis::Horizontal);
        let twice = flip_transform(once, Axis::Horizontal);
        assert_eq!(twice, start);
        assert_eq!(center_of(&once), center_of(&start), "flip stays centered");
    }

    #[test]
    fn resize_of_a_rotated_object_keeps_the_opposite_edge_fixed_in_world() {
        let mut start = base();
        start.rotation_deg = 90.0;
        let before = handle_points(&start)[7];
        // Drag the E handle along the object's local x axis (world +y after a 90° turn).
        let end = resize_transform(start, Handle::E, Vec2::new(0.0, 20.0), Modifiers::default());
        assert!(
            (end.w - 120.0).abs() < 1e-9,
            "local width grows by 20: {}",
            end.w
        );
        let after = handle_points(&end)[7];
        assert!(close(before, after), "opposite edge fixed in world");
    }

    #[test]
    fn resize_cursor_follows_the_objects_axes() {
        let mut t = base();
        assert_eq!(resize_cursor(&t, Handle::E), "ew-resize");
        assert_eq!(resize_cursor(&t, Handle::NW), "nwse-resize");
        t.rotation_deg = 90.0;
        // After a quarter turn the local horizontal axis is vertical on screen.
        assert_eq!(resize_cursor(&t, Handle::E), "ns-resize");
    }

    #[test]
    fn resize_of_a_flipped_object_moves_the_handle_that_was_grabbed() {
        let mut start = base();
        start.flip_h = true; // box spans [x - w, x] = [0, 100] in world; E handle is on the left
        let before = handle_points(&start);
        let east = Handle::ALL.iter().position(|h| *h == Handle::E).unwrap();
        let west = 7;
        // Drag the grabbed (world-left) edge further left by 30.
        let end = resize_transform(
            start,
            Handle::E,
            Vec2::new(-30.0, 0.0),
            Modifiers::default(),
        );
        assert!((end.w - 130.0).abs() < 1e-9, "width grows by 30: {}", end.w);
        assert!(end.flip_h, "flip flag is kept");
        let after = handle_points(&end);
        assert!(
            close(before[west], after[west]),
            "opposite edge fixed in world"
        );
        assert!(
            (after[east].x - (before[east].x - 30.0)).abs() < 1e-9,
            "grabbed edge follows the pointer"
        );
    }

    #[test]
    fn resize_of_a_flipped_and_rotated_object_keeps_the_anchor_fixed() {
        let mut start = base();
        start.flip_v = true;
        start.rotation_deg = 40.0;
        let before = handle_points(&start);
        let nw = Handle::ALL.iter().position(|h| *h == Handle::NW).unwrap();
        let end = resize_transform(
            start,
            Handle::SE,
            Vec2::new(12.0, -7.0),
            Modifiers::default(),
        );
        assert!(end.flip_v);
        assert!(
            close(before[nw], handle_points(&end)[nw]),
            "opposite corner stays fixed in world"
        );
        assert!(end.w >= 0.0 && end.h >= 0.0);
    }
}
