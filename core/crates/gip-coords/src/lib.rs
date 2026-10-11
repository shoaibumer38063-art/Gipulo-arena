//! # gip-coords — coordinate spaces, conversions, matrices, camera math
//!
//! ONE authoritative module for all coordinate spaces and conversions (CAN-COORD-2: a second
//! conversion implementation anywhere is a critical violation). Every pointer position,
//! overlay anchor, ruler tick, snap candidate, and export region passes through here.
//! Contract: `docs/architecture/coordinate-system.md`; conventions: `ADR-002` (PROPOSED —
//! binding as implementation default).
//!
//! **Implemented (coordinates phase, 2026-10-11):** the eight canonical spaces ·
//! [`Vec2`] · [`Mat2`] (column-major 6-tuple, CSS/SVG/Canvas2D order) · [`Trs`] canonical
//! transform fields with `T·R·S` composition and matrix decomposition · [`Camera`]
//! (pan/zoom, `zoom_around`) · viewport/screen/device conversions · angle normalization ·
//! the single epsilon module (CAN-NUM-5).
//!
//! **Deferred, deliberately:** `zoomToFit`/`zoomToSelection` (need geometry bounds — land
//! with the geometry phase; the camera's fit math takes a bounds rect) · the viewport
//! registry's DOM observation (shell measures; core receives numbers) · momentum panning
//! (off in v1, CAN-CAM-4).

#![forbid(unsafe_code)]

pub mod angle;
pub mod camera;
mod eps;
pub mod mat2;
pub mod trs;
pub mod vec2;
pub mod viewport;

pub use angle::normalize_deg;
pub use camera::{Camera, MAX_ZOOM, MIN_ZOOM};
pub use eps::{EPS_ANGLE, EPS_MATRIX, EPS_REL, EPS_ROUNDTRIP};
pub use mat2::Mat2;
pub use trs::Trs;
pub use vec2::Vec2;
pub use viewport::ViewportRect;

/// The eight canonical coordinate spaces (CAN-COORD-1).
///
/// Names are law: they must match `docs/architecture/coordinate-system.md` §2 exactly, because
/// diagnostics, tests, and docs all use this shared vocabulary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Space {
    /// Client pixels of the browser window.
    Screen,
    /// Pixels relative to the canvas element.
    Viewport,
    /// Infinite page space — the canonical editing space.
    World,
    /// Relative to the parent node's transform.
    Parent,
    /// The node's own untransformed space.
    Local,
    /// Pre-transform path coordinates.
    Path,
    /// Text shaping/run space (text engine internal).
    Text,
    /// Physical pixels (viewport × DPR) — renderer-internal only.
    Device,
}

impl Space {
    /// All eight spaces, in canonical documentation order.
    pub const ALL: [Space; 8] = [
        Space::Screen,
        Space::Viewport,
        Space::World,
        Space::Parent,
        Space::Local,
        Space::Path,
        Space::Text,
        Space::Device,
    ];

    /// Canonical lowercase name (matches documentation and serialized diagnostics).
    #[must_use]
    pub const fn name(self) -> &'static str {
        match self {
            Space::Screen => "screen",
            Space::Viewport => "viewport",
            Space::World => "world",
            Space::Parent => "parent",
            Space::Local => "local",
            Space::Path => "path",
            Space::Text => "text",
            Space::Device => "device",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::BTreeSet;

    #[test]
    fn eight_canonical_spaces_exist_with_unique_names() {
        let names: BTreeSet<&str> = Space::ALL.iter().map(|s| s.name()).collect();
        assert_eq!(
            Space::ALL.len(),
            8,
            "CAN-COORD-1 defines exactly eight spaces"
        );
        assert_eq!(names.len(), 8, "space names must be unique");
        // Spot-check the two most confusable pair members explicitly.
        assert!(names.contains("screen"));
        assert!(names.contains("device"));
    }
}
