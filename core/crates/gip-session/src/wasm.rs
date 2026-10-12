//! wasm-bindgen boundary surface — the Rust source of truth for the generated bindings.
//!
//! Transport ONLY: nothing in this module may contain editor logic (Boundary Duplication Law,
//! engine-core.md §1). Every method forwards to [`crate::Session`]. Generated output lives in
//! `bindings/pkg/` and is never hand-edited.

#![allow(unreachable_pub)] // wasm-bindgen requires pub items inside this private module.

use wasm_bindgen::prelude::*;

use crate::{PROTOCOL_VERSION, RectOp, Session, ViewSnapshot};
use gip_interaction::Tool;

/// Floats per rectangle in [`Engine::rects`]: 4 corners (x, y) then a flag bitmask.
/// Flags: 1 = selected, 2 = hovered, 4 = creation preview.
#[wasm_bindgen]
#[must_use]
pub fn rect_stride() -> usize {
    9
}

/// Returns the boundary protocol version for the shell↔core handshake (engine-core.md §10).
#[wasm_bindgen]
#[must_use]
pub fn protocol_version() -> u32 {
    PROTOCOL_VERSION
}

/// The view state the shell paints. Positions are viewport pixels, converted by the core.
#[wasm_bindgen]
#[derive(Debug, Clone, Copy)]
pub struct ViewState {
    /// Zoom factor (1.0 = 100 %).
    #[wasm_bindgen(readonly)]
    pub zoom: f64,
    /// Zoom percentage (100.0 = 100 %), computed by the core.
    #[wasm_bindgen(readonly)]
    pub zoom_percent: f64,
    /// Camera x in world units.
    #[wasm_bindgen(readonly)]
    pub camera_x: f64,
    /// Camera y in world units.
    #[wasm_bindgen(readonly)]
    pub camera_y: f64,
    /// World origin x in viewport pixels.
    #[wasm_bindgen(readonly)]
    pub origin_x: f64,
    /// World origin y in viewport pixels.
    #[wasm_bindgen(readonly)]
    pub origin_y: f64,
    /// Viewport width in CSS pixels.
    #[wasm_bindgen(readonly)]
    pub width: f64,
    /// Viewport height in CSS pixels.
    #[wasm_bindgen(readonly)]
    pub height: f64,
}

impl From<ViewSnapshot> for ViewState {
    fn from(s: ViewSnapshot) -> Self {
        Self {
            zoom: s.zoom,
            zoom_percent: s.zoom_percent,
            camera_x: s.camera_x,
            camera_y: s.camera_y,
            origin_x: s.origin.x,
            origin_y: s.origin.y,
            width: s.width,
            height: s.height,
        }
    }
}

/// The editor engine as seen from the shell. One instance per editor view.
#[wasm_bindgen]
#[derive(Debug, Default)]
pub struct Engine {
    session: Session,
}

#[wasm_bindgen]
impl Engine {
    /// Creates an engine with an empty viewport and the default camera.
    #[wasm_bindgen(constructor)]
    #[must_use]
    pub fn new() -> Engine {
        Engine {
            session: Session::new(),
        }
    }

    /// Records the canvas client rect. Returns `false` for invalid sizes (nothing changes).
    pub fn set_viewport(&mut self, x: f64, y: f64, width: f64, height: f64) -> bool {
        self.session.set_viewport(x, y, width, height)
    }

    /// Zooms by `factor` around a client-pixel point. Returns `false` for invalid input.
    pub fn zoom_at(&mut self, client_x: f64, client_y: f64, factor: f64) -> bool {
        self.session.zoom_at(client_x, client_y, factor)
    }

    /// Pans by a pixel delta. Returns `false` for invalid input.
    pub fn pan_by(&mut self, dx: f64, dy: f64) -> bool {
        self.session.pan_by(dx, dy)
    }

    /// The current view state for painting.
    #[must_use]
    pub fn view(&self) -> ViewState {
        self.session.snapshot().into()
    }

    /// Pointer pressed (client pixels). `button` 0 = primary (select/draw), 1 = middle (pan).
    /// Returns `false` for non-finite input.
    pub fn pointer_down(&mut self, client_x: f64, client_y: f64, button: u8, shift: bool) -> bool {
        self.session.pointer_down(client_x, client_y, button, shift)
    }

    /// Pointer moved (client pixels). Returns `false` for non-finite input.
    pub fn pointer_move(&mut self, client_x: f64, client_y: f64) -> bool {
        self.session.pointer_move(client_x, client_y)
    }

    /// Pointer released (client pixels). Commits the gesture if it changed the document.
    pub fn pointer_up(&mut self, client_x: f64, client_y: f64) -> bool {
        self.session.pointer_up(client_x, client_y)
    }

    /// Cancels the gesture in progress without a history entry (Escape or pointercancel).
    pub fn cancel(&mut self) {
        self.session.cancel();
    }

    /// Escape: cancels a running gesture, otherwise clears the selection.
    pub fn escape(&mut self) {
        self.session.escape();
    }

    /// Sets the tool: `"select"` or `"rectangle"`. Returns `false` for unknown names or during
    /// a gesture.
    pub fn set_tool(&mut self, name: &str) -> bool {
        let tool = match name {
            "select" => Tool::Select,
            "rectangle" => Tool::Rectangle,
            _ => return false,
        };
        self.session.set_tool(tool)
    }

    /// The active tool: `"select"` or `"rectangle"`.
    #[must_use]
    pub fn tool(&self) -> String {
        match self.session.tool() {
            Tool::Select => "select",
            Tool::Rectangle => "rectangle",
        }
        .to_owned()
    }

    /// Deletes the selected objects as one history entry.
    pub fn delete_selection(&mut self) -> bool {
        self.session.delete_selection()
    }

    /// Undo the last committed gesture.
    pub fn undo(&mut self) -> bool {
        self.session.undo()
    }

    /// Redo the last undone gesture.
    pub fn redo(&mut self) -> bool {
        self.session.redo()
    }

    /// Whether undo would change anything.
    #[must_use]
    pub fn can_undo(&self) -> bool {
        self.session.can_undo()
    }

    /// Whether redo would change anything.
    #[must_use]
    pub fn can_redo(&self) -> bool {
        self.session.can_redo()
    }

    /// Number of objects on the page.
    #[must_use]
    pub fn object_count(&self) -> usize {
        self.session.object_count()
    }

    /// Number of selected objects.
    #[must_use]
    pub fn selection_count(&self) -> usize {
        self.session.selection_count()
    }

    /// Flat draw list, `rect_stride()` floats per rectangle, bottom-most first. Each record:
    /// four viewport-pixel corners (x0, y0, x1, y1, x2, y2, x3, y3) then a flag bitmask
    /// (1 selected, 2 hovered, 4 creation preview).
    #[must_use]
    pub fn rects(&self) -> Vec<f64> {
        let mut out = Vec::new();
        for op in self.session.rects() {
            out.extend(corner_floats(&op));
            out.push(flags(&op));
        }
        out
    }
}

fn corner_floats(op: &RectOp) -> [f64; 8] {
    let c = op.corners;
    [
        c[0].x, c[0].y, c[1].x, c[1].y, c[2].x, c[2].y, c[3].x, c[3].y,
    ]
}

fn flags(op: &RectOp) -> f64 {
    f64::from(u8::from(op.selected) | (u8::from(op.hovered) << 1) | (u8::from(op.preview) << 2))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn wasm_surface_exposes_protocol_version() {
        assert_eq!(protocol_version(), PROTOCOL_VERSION);
    }
}
