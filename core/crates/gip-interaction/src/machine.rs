//! The interaction state machine. See the crate docs for the contract.

use gip_coords::viewport::screen_to_viewport;
use gip_coords::{Camera, Mat2, Vec2, ViewportRect};
use gip_document::{Document, NodeId, Transform};
use gip_geometry::trs_of;

/// Pixels the pointer must travel after pressing before a gesture becomes a drag (CAN-INPUT-2).
pub const DRAG_THRESHOLD_PX: f64 = 3.0;

/// Screen-space hit tolerance for zero-size objects (CAN-GEOM-5). [PROPOSED] value; the rule's
/// point-handle equivalent is 16 px, so the radius is half that.
pub const HIT_TOLERANCE_PX: f64 = 8.0;

/// Handle activation half-size in screen pixels (Slice 1, D2: a 12 px clickable square around the
/// 8 px visual).
pub const HANDLE_SLOP_PX: f64 = 6.0;

/// The active tool (CAN-MODE-1). Pan is a gesture (middle button), not a tool.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Tool {
    /// Click and drag to select and move objects.
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

/// Modifier keys that change a gesture while it runs (CAN-TR-3).
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct Modifiers {
    /// Shift: proportional corner resize.
    pub shift: bool,
    /// Alt / ⌥: resize from the center.
    pub alt: bool,
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

    /// Which edges the handle moves: `(-1 | 0 | 1, -1 | 0 | 1)` for (x, y). Zero means no move.
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

/// An axis-aligned box `(x, y, w, h)` in world units.
pub type Rect4 = (f64, f64, f64, f64);

/// World positions of the eight handles of an unrotated transform, in [`Handle::ALL`] order.
#[must_use]
pub fn handle_points(t: &Transform) -> [Vec2; 8] {
    let mut out = [Vec2::ZERO; 8];
    for (slot, handle) in out.iter_mut().zip(Handle::ALL) {
        let (hx, vy) = handle.axes();
        *slot = Vec2::new(
            t.x + (f64::from(hx) + 1.0) / 2.0 * t.w,
            t.y + (f64::from(vy) + 1.0) / 2.0 * t.h,
        );
    }
    out
}

/// The single node that shows handles: exactly one selected, unrotated, unflipped, visible, and
/// unlocked (Slice 1 scope; rotated and flipped resize is deferred).
#[must_use]
pub fn resize_target(doc: &Document, selection: &[NodeId]) -> Option<NodeId> {
    if selection.len() != 1 {
        return None;
    }
    let id = selection[0];
    let node = doc.nodes().get(&id)?;
    let t = node.transform;
    (node.visible && !node.locked && t.rotation_deg == 0.0 && !t.flip_h && !t.flip_v).then_some(id)
}

/// The box after a handle drag (CAN-TR-2: `start` and the total `delta` only, never accumulated).
///
/// - A corner moves two edges; an edge moves one (CAN-TR-3). The opposite edge stays fixed.
/// - Dimensions clamp at zero: the box never mirrors or goes negative [D1: clamp].
/// - `proportional` (Shift) keeps the start aspect for corner handles only.
/// - `from_center` (Alt) keeps the center fixed.
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
    /// Resize one node (Slice 1).
    Resize {
        /// Node resized.
        node: NodeId,
        /// Box at gesture start.
        from: Rect4,
        /// Box at gesture end.
        to: Rect4,
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
    /// Nodes being dragged, at their previewed (not yet committed) positions.
    Moving {
        /// Previewed positions.
        items: Vec<(NodeId, (f64, f64))>,
    },
    /// A node being resized, at its previewed box.
    Resizing {
        /// Node resized.
        node: NodeId,
        /// Previewed box.
        rect: Rect4,
    },
}

#[derive(Debug, Clone, PartialEq)]
enum State {
    Idle,
    /// Button down, threshold not yet crossed. `handle` is set when the press landed on a handle.
    Pressed {
        client: Vec2,
        world: Vec2,
        handle: Option<(NodeId, Handle)>,
    },
    /// Dragging selected nodes.
    Moving {
        start_world: Vec2,
        items: Vec<(NodeId, (f64, f64))>,
        delta: Vec2,
    },
    /// Dragging a handle.
    Resizing {
        node: NodeId,
        start: Rect4,
        handle: Handle,
        start_world: Vec2,
        current: Rect4,
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
        }
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

    /// Whether a gesture is in progress.
    #[must_use]
    pub fn is_gesturing(&self) -> bool {
        self.state != State::Idle
    }

    /// Current gesture preview for painting.
    #[must_use]
    pub fn preview(&self) -> Preview {
        match &self.state {
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
            State::Resizing { node, current, .. } => Preview::Resizing {
                node: *node,
                rect: *current,
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
        if button == 1 {
            self.state = State::Panning {
                last_client: client,
            };
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
                    handle: None,
                };
            }
            Tool::Select => {
                if let Some(handle) = self.handle_at(doc, view, world) {
                    // A press on a handle leaves the selection alone and starts a resize.
                    self.state = State::Pressed {
                        client,
                        world,
                        handle: Some(handle),
                    };
                    return Vec::new();
                }
                let hit = hit_test(doc, view, world);
                self.apply_click_selection(hit, shift);
                self.state = State::Pressed {
                    client,
                    world,
                    handle: None,
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
                self.hover = if self.tool == Tool::Select {
                    hit_test(doc, view, to_world(view, client))
                } else {
                    None
                };
                Vec::new()
            }
            State::Pressed {
                client: press_client,
                world: press_world,
                handle,
            } => {
                if client.distance_sq(press_client).sqrt() < DRAG_THRESHOLD_PX {
                    return Vec::new();
                }
                let world = to_world(view, client);
                if let Some((node, handle)) = handle {
                    if let Some(start) = node_box(doc, node) {
                        self.state = State::Resizing {
                            node,
                            start,
                            handle,
                            start_world: press_world,
                            current: start,
                        };
                        self.update_resize(world, mods);
                    } else {
                        self.state = State::Idle;
                    }
                    return Vec::new();
                }
                match self.tool {
                    Tool::Rectangle => {
                        self.state = State::Creating {
                            start_world: press_world,
                            current_world: world,
                        };
                    }
                    Tool::Select => {
                        // Dragging starts only from a press on a selected node. Marquee
                        // selection is out of scope for Slice 0.
                        let items = self.selected_positions(doc);
                        if items.is_empty() || !self.pressed_on_selection(doc, view, press_world) {
                            self.state = State::Idle;
                        } else {
                            self.state = State::Moving {
                                start_world: press_world,
                                items,
                                delta: Vec2::ZERO,
                            };
                            self.update_move(world, press_world);
                        }
                    }
                }
                Vec::new()
            }
            State::Moving { start_world, .. } => {
                self.update_move(to_world(view, client), start_world);
                Vec::new()
            }
            State::Resizing { .. } => {
                self.update_resize(to_world(view, client), mods);
                Vec::new()
            }
            State::Creating { start_world, .. } => {
                self.state = State::Creating {
                    start_world,
                    current_world: to_world(view, client),
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
        let state = std::mem::replace(&mut self.state, State::Idle);
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
            } => {
                if current == start {
                    Vec::new()
                } else {
                    vec![Effect::Commit(Commit::Resize {
                        node,
                        from: start,
                        to: current,
                    })]
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
            _ => Vec::new(),
        }
    }

    /// Cancels the gesture in progress without a history entry (CAN-MODE-4).
    pub fn cancel(&mut self) {
        self.state = State::Idle;
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

    /// The handle under `world`, if the single resize target has one (within the activation
    /// square, [`HANDLE_SLOP_PX`] screen pixels).
    fn handle_at(&self, doc: &Document, view: View, world: Vec2) -> Option<(NodeId, Handle)> {
        let id = resize_target(doc, &self.selection)?;
        let t = doc.nodes().get(&id)?.transform;
        let tol = HANDLE_SLOP_PX / view.camera.zoom;
        Handle::ALL
            .iter()
            .zip(handle_points(&t))
            .find(|(_, p)| (p.x - world.x).abs() <= tol && (p.y - world.y).abs() <= tol)
            .map(|(handle, _)| (id, *handle))
    }

    fn update_move(&mut self, world: Vec2, start_world: Vec2) {
        if let State::Moving { delta, .. } = &mut self.state {
            *delta = world - start_world;
        }
    }

    fn update_resize(&mut self, world: Vec2, mods: Modifiers) {
        if let State::Resizing {
            start,
            handle,
            start_world,
            current,
            ..
        } = &mut self.state
        {
            *current = resize_box(*start, *handle, world - *start_world, mods);
        }
    }
}

/// The unrotated box of a node, or `None` if it does not exist.
fn node_box(doc: &Document, id: NodeId) -> Option<Rect4> {
    let t = doc.nodes().get(&id)?.transform;
    Some((t.x, t.y, t.w, t.h))
}

fn to_world(view: View, client: Vec2) -> Vec2 {
    view.camera
        .viewport_to_world(screen_to_viewport(client, view.rect))
}

fn normalized(a: Vec2, b: Vec2) -> (f64, f64, f64, f64) {
    let x = a.x.min(b.x);
    let y = a.y.min(b.y);
    (x, y, (a.x - b.x).abs(), (a.y - b.y).abs())
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
        let m = Mat2::from_trs(trs_of(&t));
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
