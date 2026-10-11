//! Canonical vector path DATA (ADR-006, VG1). Plain values only — every operation on these
//! types lives in `gip-geometry` (VG2). Keeping the data here keeps this crate dependency-free
//! (layer-1 law) while the geometry engine depends on it, never the reverse.
//!
//! Path payloads carry no `ext` map: they are versioned with the file as a whole and unknown keys
//! are rejected (ADR-003 #9 amendment — Vertex is a hot, Copy value type).
//!
//! Model: a `Path` holds ordered `Subpath`s; a subpath holds ordered `Vertex`es. Each vertex is
//! an anchor plus ABSOLUTE in/out handle positions. Segments are implied between consecutive
//! vertices (plus one closing segment last→first when `closed`) and are always cubic Béziers.
//!
//! Straight segment (data definition, ADR-006 #6): the start vertex's `handle_out` equals its
//! `anchor` AND the end vertex's `handle_in` equals its `anchor`, exactly.

/// A 2D point in path-local coordinates, `[x, y]`, f64 (CAN-NUM-2: never rounded when stored).
pub type Point = [f64; 2];

use serde::{Deserialize, Serialize};

/// Fill rule, explicit per path and consumed identically by every consumer (VG5).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum FillRule {
    /// Non-zero winding (default).
    #[default]
    NonZero,
    /// Even-odd crossing parity.
    EvenOdd,
}

/// Designer intent for a vertex: corner (independent handles) or smooth (aligned handles).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum PointType {
    /// Corner point — handles move independently.
    #[default]
    Corner,
    /// Smooth point — handles stay on one line through the anchor.
    Smooth,
}

/// How the two handles of a vertex relate when one is edited (VG1 handle mode).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum HandleMode {
    /// Opposite handle matches direction and length.
    Mirrored,
    /// Opposite handle matches direction only.
    Asymmetric,
    /// Handles are independent.
    #[default]
    Disconnected,
}

/// An anchor with its absolute incoming and outgoing control points.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct Vertex {
    /// The anchor (on-curve point).
    pub anchor: Point,
    /// Control point of the segment arriving at this vertex.
    pub handle_in: Point,
    /// Control point of the segment leaving this vertex.
    pub handle_out: Point,
    /// Corner or smooth.
    pub point_type: PointType,
    /// Handle relationship mode.
    pub handle_mode: HandleMode,
}

impl Vertex {
    /// A corner vertex with both handles collapsed onto the anchor (no curvature).
    #[must_use]
    pub const fn corner(anchor: Point) -> Self {
        Self {
            anchor,
            handle_in: anchor,
            handle_out: anchor,
            point_type: PointType::Corner,
            handle_mode: HandleMode::Disconnected,
        }
    }
}

/// An ordered run of vertices. `closed` is explicit data, never inferred.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct Subpath {
    /// Vertices in drawing order.
    pub vertices: Vec<Vertex>,
    /// Whether a closing segment joins the last vertex back to the first.
    pub closed: bool,
}

/// A complete vector path: subpaths plus the fill rule.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct Path {
    /// Subpaths in drawing order.
    pub subpaths: Vec<Subpath>,
    /// Fill rule shared by all subpaths.
    pub fill_rule: FillRule,
}

impl Path {
    /// Checks that every coordinate is finite. Serialization refuses non-finite values because
    /// the JSON writer would silently emit `null` for NaN/Inf (ADR-003 #3).
    ///
    /// # Errors
    /// Returns a reason string naming the first non-finite coordinate.
    pub fn validate_finite(&self) -> Result<(), String> {
        let ok = |p: &Point| p[0].is_finite() && p[1].is_finite();
        let all_ok = self
            .subpaths
            .iter()
            .flat_map(|s| &s.vertices)
            .all(|v| ok(&v.anchor) && ok(&v.handle_in) && ok(&v.handle_out));
        if all_ok {
            Ok(())
        } else {
            Err("path contains a non-finite coordinate".into())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn corner_vertex_has_collapsed_handles() {
        let v = Vertex::corner([3.0, -4.0]);
        assert_eq!(v.handle_in, v.anchor);
        assert_eq!(v.handle_out, v.anchor);
        assert_eq!(v.point_type, PointType::Corner);
    }

    #[test]
    fn defaults_are_the_documented_defaults() {
        assert_eq!(FillRule::default(), FillRule::NonZero);
        assert_eq!(HandleMode::default(), HandleMode::Disconnected);
        assert!(
            !Subpath::default().closed,
            "open is the default; closed is explicit"
        );
    }
}
