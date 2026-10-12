//! The interaction state machine. See the crate docs for the contract.

use gip_coords::viewport::screen_to_viewport;
use gip_coords::{Camera, Mat2, Vec2, ViewportRect};
use gip_document::{Document, NodeId};
use gip_geometry::trs_of;

/// Pixels the pointer must travel after pressing before a gesture becomes a drag (CAN-INPUT-2).
pub const DRAG_THRESHOLD_PX: f64 = 3.0;

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
}

#[derive(Debug, Clone, PartialEq)]
enum State {
    Idle,
    /// Button down, threshold not yet crossed.
    Pressed {
        client: Vec2,
        world: Vec2,
    },
    /// Dragging selected nodes.
    Moving {
        start_world: Vec2,
        items: Vec<(NodeId, (f64, f64))>,
        delta: Vec2,
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
                self.state = State::Pressed { client, world };
            }
            Tool::Select => {
                let hit = hit_test(doc, view, world);
                self.apply_click_selection(hit, shift);
                self.state = State::Pressed { client, world };
            }
        }
        Vec::new()
    }

    /// Pointer moved (buttons not required; hover is tracked while idle).
    pub fn pointer_move(&mut self, doc: &Document, view: View, client: Vec2) -> Vec<Effect> {
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
            } => {
                if client.distance_sq(press_client).sqrt() < DRAG_THRESHOLD_PX {
                    return Vec::new();
                }
                let world = to_world(view, client);
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

    fn update_move(&mut self, world: Vec2, start_world: Vec2) {
        if let State::Moving { delta, .. } = &mut self.state {
            *delta = world - start_world;
        }
    }
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
#[must_use]
pub fn hit_test(doc: &Document, _view: View, world: Vec2) -> Option<NodeId> {
    let page = doc.pages().first()?;
    let nodes = doc.nodes();
    page.children.iter().rev().copied().find(|id| {
        let Some(node) = nodes.get(id) else {
            return false;
        };
        if !node.visible || node.locked {
            return false;
        }
        let m = Mat2::from_trs(trs_of(&node.transform));
        m.world_to_local(world)
            .is_some_and(|l| (0.0..=1.0).contains(&l.x) && (0.0..=1.0).contains(&l.y))
    })
}
