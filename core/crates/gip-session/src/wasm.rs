//! wasm-bindgen boundary surface — the Rust source of truth for the generated bindings.
//!
//! Transport ONLY: nothing in this module may contain editor logic (Boundary Duplication Law,
//! engine-core.md §1). Every method forwards to [`crate::Session`]. Generated output lives in
//! `bindings/pkg/` and is never hand-edited.

#![allow(unreachable_pub)] // wasm-bindgen requires pub items inside this private module.

use wasm_bindgen::prelude::*;

use crate::{PROTOCOL_VERSION, Session, ViewSnapshot};

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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn wasm_surface_exposes_protocol_version() {
        assert_eq!(protocol_version(), PROTOCOL_VERSION);
    }
}
