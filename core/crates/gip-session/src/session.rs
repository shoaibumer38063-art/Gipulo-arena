//! Editor session view state (Slice 0a, task 020): the viewport rectangle and the camera.
//!
//! Every conversion happens here, through `gip-coords` (CAN-COORD-2). The shell sends raw
//! client-pixel pointer positions and receives core-computed viewport positions; it never
//! performs zoom or pan arithmetic itself. The session never touches document data
//! (CAN-CAM-1): panning and zooming are not history entries.

use gip_coords::viewport::screen_to_viewport;
use gip_coords::{Camera, Vec2, ViewportRect};

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

/// The editor's view state: canvas placement plus camera.
#[derive(Debug, Clone, PartialEq)]
pub struct Session {
    rect: ViewportRect,
    camera: Camera,
}

impl Session {
    /// A session with an empty viewport and the default camera (100 %, origin at top-left).
    #[must_use]
    pub fn new() -> Self {
        Self {
            rect: ViewportRect::new(0.0, 0.0, 0.0, 0.0),
            camera: Camera::DEFAULT,
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
