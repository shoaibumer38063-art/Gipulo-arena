//! Camera store (CAN-CAM): pan/zoom state and world⇄viewport math.
//!
//! Semantics: `(x, y)` is the **world coordinate of the viewport origin** (top-left); `zoom`
//! is viewport pixels per world unit. So `viewport = (world − camera) · zoom` and
//! `world = viewport / zoom + camera`. The camera never reads or writes document data
//! (CAN-CAM-1) and stores no derived screen positions (CAN-COORD-4).
//!
//! Screen-space inputs are converted to viewport space first (CAN-COORD-3 composite law);
//! this API is viewport-space so the core stays DOM-free (engine-core.md §3).

use crate::vec2::Vec2;

/// Minimum zoom: 2% (CAN-CAM-3 [PROPOSED], confirmed by ADR-002 as documentation default).
pub const MIN_ZOOM: f64 = 0.02;

/// Maximum zoom: 4096% (CAN-CAM-3 [PROPOSED]).
pub const MAX_ZOOM: f64 = 40.96;

/// The camera: world position of the viewport origin + zoom factor.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Camera {
    /// World x of the viewport origin.
    pub x: f64,
    /// World y of the viewport origin.
    pub y: f64,
    /// Viewport pixels per world unit (`MIN_ZOOM..=MAX_ZOOM` for camera commands).
    pub zoom: f64,
}

impl Camera {
    /// The default camera: world origin at the viewport origin, 100% zoom.
    pub const DEFAULT: Self = Self::new(0.0, 0.0, 1.0);

    /// Creates a camera. Callers use [`Camera::clamped`] at input edges; this constructor
    /// is raw so tests can build out-of-range states explicitly.
    #[must_use]
    pub const fn new(x: f64, y: f64, zoom: f64) -> Self {
        Self { x, y, zoom }
    }

    /// The camera with `zoom` clamped into `MIN_ZOOM..=MAX_ZOOM` (CAN-CAM-3).
    #[must_use]
    pub fn clamped(self) -> Self {
        Self::new(self.x, self.y, self.zoom.clamp(MIN_ZOOM, MAX_ZOOM))
    }

    /// True when all fields are finite and `zoom > 0`.
    #[must_use]
    pub fn is_valid(self) -> bool {
        self.x.is_finite() && self.y.is_finite() && self.zoom.is_finite() && self.zoom > 0.0
    }

    /// World → viewport: `(world − camera) · zoom`.
    #[must_use]
    pub fn world_to_viewport(self, world: Vec2) -> Vec2 {
        Vec2::new(
            (world.x - self.x) * self.zoom,
            (world.y - self.y) * self.zoom,
        )
    }

    /// Viewport → world: `viewport / zoom + camera`.
    #[must_use]
    pub fn viewport_to_world(self, viewport: Vec2) -> Vec2 {
        Vec2::new(
            viewport.x / self.zoom + self.x,
            viewport.y / self.zoom + self.y,
        )
    }

    /// Slides the camera by a viewport-space delta (screen-constant hand/middle-drag pan).
    pub fn pan_by_viewport(&mut self, delta: Vec2) {
        self.x -= delta.x / self.zoom;
        self.y -= delta.y / self.zoom;
    }

    /// Slides the camera by a world-space delta.
    pub fn pan_by_world(&mut self, delta: Vec2) {
        self.x += delta.x;
        self.y += delta.y;
    }

    /// Zoom by `factor` about a viewport point: the world point currently under that point
    /// stays under it (CAN-CAM-2, tolerance ≤ 0.5 screen px). The zoom is clamped; the
    /// camera position is then derived from the *clamped* zoom, so the invariance holds at
    /// the limits too.
    pub fn zoom_around_viewport(&mut self, viewport_point: Vec2, factor: f64) {
        let new_zoom = (self.zoom * factor).clamp(MIN_ZOOM, MAX_ZOOM);
        let world = self.viewport_to_world(viewport_point);
        self.zoom = new_zoom;
        self.x = world.x - viewport_point.x / new_zoom;
        self.y = world.y - viewport_point.y / new_zoom;
    }

    /// Sets zoom to a clamped absolute value about a viewport point (keyboard zoom stops,
    /// `⌘0` reset etc. — UI presets are a shell concern; this is the math).
    pub fn set_zoom_around_viewport(&mut self, viewport_point: Vec2, zoom: f64) {
        if self.zoom > 0.0 {
            self.zoom_around_viewport(viewport_point, zoom / self.zoom);
        }
    }

    /// Zoom as a percentage (display/formatting input; the shell formats, this supplies).
    #[must_use]
    pub fn zoom_percent(self) -> f64 {
        self.zoom * 100.0
    }
}

impl Default for Camera {
    fn default() -> Self {
        Self::DEFAULT
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::EPS_ROUNDTRIP;

    #[test]
    fn world_viewport_mapping_is_inverse() {
        let cam = Camera::new(-120.5, 44.25, 3.0);
        let world = Vec2::new(1000.75, -333.125);
        let back = cam.viewport_to_world(cam.world_to_viewport(world));
        assert!(back.approx_eq(world, EPS_ROUNDTRIP));
    }

    #[test]
    fn pointer_zoom_keeps_world_point_under_cursor() {
        let mut cam = Camera::new(10.0, -5.0, 1.5);
        let cursor = Vec2::new(640.0, 360.0);
        let anchor = cam.viewport_to_world(cursor);
        for factor in [1.2, 0.5, 2.0, 0.8, 1.05] {
            cam.zoom_around_viewport(cursor, factor);
            let still = cam.world_to_viewport(anchor);
            assert!(still.approx_eq(cursor, 0.5), "anchor drifted to {still:?}");
        }
    }

    #[test]
    fn zoom_clamps_at_limits_without_drifting_the_anchor() {
        let mut cam = Camera::new(0.0, 0.0, 1.0);
        let cursor = Vec2::new(300.0, 200.0);
        let anchor = cam.viewport_to_world(cursor);
        for _ in 0..200 {
            cam.zoom_around_viewport(cursor, 2.0);
        }
        assert!(cam.zoom <= MAX_ZOOM);
        assert!(cam.world_to_viewport(anchor).approx_eq(cursor, 0.5));
        for _ in 0..200 {
            cam.zoom_around_viewport(cursor, 0.5);
        }
        assert!(cam.zoom >= MIN_ZOOM);
        assert!(cam.world_to_viewport(anchor).approx_eq(cursor, 0.5));
    }

    #[test]
    fn pan_is_screen_constant_regardless_of_zoom() {
        let mut cam = Camera::new(0.0, 0.0, 4.0);
        let p = Vec2::new(50.0, 60.0);
        let before = cam.world_to_viewport(p);
        cam.pan_by_viewport(Vec2::new(20.0, -15.0));
        let after = cam.world_to_viewport(p);
        assert!((after.x - before.x - 20.0).abs() <= EPS_ROUNDTRIP);
        assert!((after.y - before.y + 15.0).abs() <= EPS_ROUNDTRIP);
    }
}
