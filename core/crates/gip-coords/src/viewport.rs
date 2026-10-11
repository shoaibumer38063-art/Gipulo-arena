//! Viewport geometry and screen-space conversions (CAN-COORD-3, composite law).
//!
//! The shell measures the DOM (canvas element rect, device pixel ratio) and hands the
//! *numbers* down (engine-core.md §3: coordinates cross as `f64` values only); every
//! conversion is computed here. The canonical chain is:
//! `device = viewport × DPR` (renderer only) → `screen→viewport` (element offset) →
//! `viewport→world` (camera).
//!
//! No partial math anywhere else (CAN-COORD-2): composites call the chain.

use crate::camera::Camera;
use crate::vec2::Vec2;

/// The canvas element's client rect (CSS pixels, document coordinates of its top-left).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ViewportRect {
    /// Left edge in client coordinates.
    pub x: f64,
    /// Top edge in client coordinates.
    pub y: f64,
    /// Width in CSS pixels.
    pub width: f64,
    /// Height in CSS pixels.
    pub height: f64,
}

impl ViewportRect {
    /// Creates a rect from measured values.
    #[must_use]
    pub const fn new(x: f64, y: f64, width: f64, height: f64) -> Self {
        Self {
            x,
            y,
            width,
            height,
        }
    }

    /// The bottom-right corner in client coordinates.
    #[must_use]
    pub const fn bottom_right(self) -> Vec2 {
        Vec2::new(self.x + self.width, self.y + self.height)
    }

    /// The size as a vector.
    #[must_use]
    pub const fn size(self) -> Vec2 {
        Vec2::new(self.width, self.height)
    }
}

/// Client (screen) point → viewport point (relative to the canvas element).
#[must_use]
pub fn screen_to_viewport(screen: Vec2, rect: ViewportRect) -> Vec2 {
    Vec2::new(screen.x - rect.x, screen.y - rect.y)
}

/// Viewport point → client (screen) point.
#[must_use]
pub fn viewport_to_screen(viewport: Vec2, rect: ViewportRect) -> Vec2 {
    Vec2::new(viewport.x + rect.x, viewport.y + rect.y)
}

/// Viewport point → physical device pixels (renderer-internal only; ADR-001 core-side).
#[must_use]
pub fn viewport_to_device(viewport: Vec2, dpr: f64) -> Vec2 {
    Vec2::new(viewport.x * dpr, viewport.y * dpr)
}

/// Physical device pixels → viewport point (renderer-internal only).
#[must_use]
pub fn device_to_viewport(device: Vec2, dpr: f64) -> Vec2 {
    Vec2::new(device.x / dpr, device.y / dpr)
}

/// Client (screen) point → world point (composite: element offset then camera).
#[must_use]
pub fn screen_to_world(screen: Vec2, rect: ViewportRect, camera: Camera) -> Vec2 {
    camera.viewport_to_world(screen_to_viewport(screen, rect))
}

/// World point → client (screen) point (composite: camera then element offset).
#[must_use]
pub fn world_to_screen(world: Vec2, rect: ViewportRect, camera: Camera) -> Vec2 {
    viewport_to_screen(camera.world_to_viewport(world), rect)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::EPS_ROUNDTRIP;

    #[test]
    fn rect_helpers_are_consistent() {
        let rect = ViewportRect::new(240.0, 40.0, 800.0, 600.0);
        assert_eq!(rect.bottom_right(), Vec2::new(1040.0, 640.0));
        assert_eq!(rect.size(), Vec2::new(800.0, 600.0));
    }

    #[test]
    fn screen_viewport_roundtrip_with_nonzero_offsets() {
        let rect = ViewportRect::new(240.5, 40.25, 800.0, 600.0);
        let screen = Vec2::new(1000.0, 700.0);
        let back = viewport_to_screen(screen_to_viewport(screen, rect), rect);
        assert!(back.approx_eq(screen, EPS_ROUNDTRIP));
    }

    #[test]
    fn composite_screen_world_roundtrip() {
        let rect = ViewportRect::new(240.0, 40.0, 800.0, 600.0);
        let camera = Camera::new(-500.0, 250.0, 2.5);
        let world = Vec2::new(123.456, -789.012);
        let screen = world_to_screen(world, rect, camera);
        let back = screen_to_world(screen, rect, camera);
        assert!(back.approx_eq(world, EPS_ROUNDTRIP));
    }

    #[test]
    fn device_conversion_is_exact_and_invertible() {
        for dpr in [1.0, 1.5, 2.0, 3.0] {
            let vp = Vec2::new(640.0, 360.0);
            let dev = viewport_to_device(vp, dpr);
            assert!((dev.x - vp.x * dpr).abs() <= EPS_ROUNDTRIP);
            assert!(device_to_viewport(dev, dpr).approx_eq(vp, EPS_ROUNDTRIP));
        }
    }
}
