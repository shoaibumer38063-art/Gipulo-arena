//! # gip-geometry — the vector/geometry engine
//!
//! Single owner of geometric truth computation: segment math, path measurement, bounds, hit
//! geometry, flattening, and the editing operations (VG2). Rendering, selection, hit-testing,
//! snapping, layout, export, and codegen consume it; none re-implements geometry.
//! Contract: `docs/architecture/vector-engine.md`. Path data model: `ADR-006` (PROPOSED,
//! binding as implementation default); path data types live in `gip-document::path`.
//!
//! **Purity law:** pure functions over document data and explicit query parameters. No DOM,
//! GPU, camera, or UI state. Tolerances are caller parameters (VG3); the only comparison ε
//! comes from the single constants module in `gip-coords` (CAN-NUM-5).
//!
//! **Implemented (geometry phase, task 010):** cubic math (eval, split, exact extrema, flatten,
//! nearest point) · path segment enumeration · tight bounds · flattening · point-in-fill (both
//! fill rules, open-as-closed) · distance-to-path · transform, reverse, close/open, insert
//! vertex (shape-exact), delete vertex.
//!
//! **Deferred (ADR-gated or later phase):** the spatial index (vector-engine §8.2) · booleans
//! (§8.3, algorithm ADR) · outline stroke / offset · stroke & effects bounds kinds (need stroke
//! and effect data) · text run geometry (CAN-TEXT-1) · selection/transform/visual bounds
//! composition (needs the transform/selection phases) · RVS stretch (layout-engine, ADR-005).

#![forbid(unsafe_code)]

pub mod bridge;
pub mod cubic;
pub mod hit;
pub mod index;
pub mod ops;
pub mod segments;

pub use bridge::{transform_of, trs_of};
pub use cubic::Cubic;
pub use hit::{Nearest, contains_point, distance_to_path};
pub use index::SegmentIndex;
pub use ops::GeomError;
pub use segments::{
    Segment, flatten_subpath, segment_count, segment_cubic, segments, tight_bounds,
};

use gip_coords::Vec2;

/// The bounds vocabulary (CAN-GEOM-2). Exactly these six kinds exist; each consumer must ask
/// for the kind its contract names (selection uses `Selection`/`Transform`, export and effect
/// caching use `Visual`, precise picking uses `Hit`, measurement guides use `Geometric`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BoundsKind {
    /// Geometry of the path/shape itself — no stroke, no effects.
    Geometrical,
    /// Geometric bounds expanded by stroke width (per stroke alignment).
    Stroke,
    /// Geometric/stroke bounds expanded by effects (shadows, blur spread, …).
    Visual,
    /// Bounds of the selection as presented (includes rotation framing per CAN-SEL-6).
    Selection,
    /// Bounds used by transform operations (oriented where applicable).
    Transform,
    /// Bounds used for hit-testing candidate filtering (precision comes from path queries —
    /// CAN-GEOM-4: a hit inside the bbox but outside the path is NOT a hit).
    Hit,
}

impl BoundsKind {
    /// All six kinds, in canonical documentation order.
    pub const ALL: [BoundsKind; 6] = [
        BoundsKind::Geometrical,
        BoundsKind::Stroke,
        BoundsKind::Visual,
        BoundsKind::Selection,
        BoundsKind::Transform,
        BoundsKind::Hit,
    ];

    /// Canonical lowercase name (matches documentation and diagnostics).
    #[must_use]
    pub const fn name(self) -> &'static str {
        match self {
            BoundsKind::Geometrical => "geometric",
            BoundsKind::Stroke => "stroke",
            BoundsKind::Visual => "visual",
            BoundsKind::Selection => "selection",
            BoundsKind::Transform => "transform",
            BoundsKind::Hit => "hit",
        }
    }
}

/// An axis-aligned rectangle in some space (min/max corners, inclusive).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct BoundsRect {
    /// Minimum corner.
    pub min: Vec2,
    /// Maximum corner.
    pub max: Vec2,
}

impl BoundsRect {
    /// Creates a rectangle from its corners (no reordering — callers pass min ≤ max).
    #[must_use]
    pub const fn new(min: Vec2, max: Vec2) -> Self {
        Self { min, max }
    }

    /// Smallest rectangle containing both.
    #[must_use]
    pub fn union(self, other: Self) -> Self {
        Self::new(
            self.min.min_components(other.min),
            self.max.max_components(other.max),
        )
    }

    /// Width and height.
    #[must_use]
    pub fn size(self) -> Vec2 {
        self.max - self.min
    }

    /// True when `p` lies inside or on the boundary.
    #[must_use]
    pub fn contains(self, p: Vec2) -> bool {
        p.x >= self.min.x && p.x <= self.max.x && p.y >= self.min.y && p.y <= self.max.y
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::BTreeSet;

    #[test]
    fn six_bounds_kinds_are_defined() {
        assert_eq!(
            BoundsKind::ALL.len(),
            6,
            "CAN-GEOM-2 defines exactly six bounds kinds"
        );
        let names: BTreeSet<&str> = BoundsKind::ALL.iter().map(|k| k.name()).collect();
        assert_eq!(names.len(), 6);
        assert_eq!(BoundsKind::Geometrical.name(), "geometric");
        assert_eq!(BoundsKind::Hit.name(), "hit");
    }

    #[test]
    fn bounds_union_and_containment() {
        let a = BoundsRect::new(Vec2::new(0.0, 0.0), Vec2::new(2.0, 2.0));
        let b = BoundsRect::new(Vec2::new(-1.0, 1.0), Vec2::new(1.0, 5.0));
        let u = a.union(b);
        assert_eq!(u.min, Vec2::new(-1.0, 0.0));
        assert_eq!(u.max, Vec2::new(2.0, 5.0));
        assert!(u.contains(Vec2::new(2.0, 5.0)) && !u.contains(Vec2::new(3.0, 0.0)));
    }
}
