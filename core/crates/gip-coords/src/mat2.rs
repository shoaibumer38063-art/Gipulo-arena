//! The ONE 2D affine matrix type (ADR-002 §2) — derived, never stored as document truth.
//!
//! Convention (verified against W3C CSS Transforms Level 1 §14 and MDN `matrix()`): the six
//! values are **column-major** with a column-vector convention — identical to CSS `matrix()`,
//! SVG `matrix()`, Canvas2D `setTransform()` and `DOMMatrix`, so handing values to any
//! browser/rendering API is a copy, never a re-ordering (ADR-002 §2).
//!
//! ```text
//!         | a  c  tx |
//! Mat2 =  | b  d  ty |      p' = Mat2 · p
//!         | 0  0  1  |
//! ```
//!
//! Composition order: `parent * local` (apply the right operand first). The canonical
//! node composition `T·R·S` is built by [`Mat2::from_trs`] / [`crate::Trs::to_matrix`];
//! ancestor chains accumulate by left-multiplication (root→leaf), per CAN-COORD-3.

use std::ops::Mul;

use crate::vec2::Vec2;
use crate::{EPS_MATRIX, Trs};

/// A 2D affine transform in column-major order (see module docs for the layout).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Mat2 {
    /// Linear component m00 (x scale/rotation term).
    pub a: f64,
    /// Linear component m10.
    pub b: f64,
    /// Linear component m01.
    pub c: f64,
    /// Linear component m11.
    pub d: f64,
    /// Translation x.
    pub tx: f64,
    /// Translation y.
    pub ty: f64,
}

impl Mat2 {
    /// The identity transform.
    pub const IDENTITY: Self = Self::new(1.0, 0.0, 0.0, 1.0, 0.0, 0.0);

    /// Creates a matrix from its six column-major values.
    #[must_use]
    pub const fn new(a: f64, b: f64, c: f64, d: f64, tx: f64, ty: f64) -> Self {
        Self { a, b, c, d, tx, ty }
    }

    /// Pure translation.
    #[must_use]
    pub const fn translation(tx: f64, ty: f64) -> Self {
        Self::new(1.0, 0.0, 0.0, 1.0, tx, ty)
    }

    /// Rotation about the local origin by `deg` degrees (canonical convention, ADR-002 §5).
    /// `deg` must already be normalized when it comes from stored data; trig handles any value.
    #[must_use]
    pub fn rotation_deg(deg: f64) -> Self {
        let (sin, cos) = deg.to_radians().sin_cos();
        Self::new(cos, sin, -sin, cos, 0.0, 0.0)
    }

    /// Axis scale (negative components express mirrors — canonical data uses flip flags
    /// instead; see [`crate::Trs`]).
    #[must_use]
    pub const fn scale(sx: f64, sy: f64) -> Self {
        Self::new(sx, 0.0, 0.0, sy, 0.0, 0.0)
    }

    /// Composes the canonical node transform `T(x, y) · R(θ) · S(sx, sy)` with the local
    /// origin at the node's top-left (ADR-002 §3–4: CSS `transform-origin: 0 0`).
    ///
    /// The `Trs` fields must already be in stored form (`w, h ≥ 0`, rotation normalized);
    /// this function mirrors them faithfully and performs no hidden repair.
    #[must_use]
    pub fn from_trs(trs: Trs) -> Self {
        let (sin, cos) = trs.rotation_deg.to_radians().sin_cos();
        let sx = if trs.flip_h { -trs.w } else { trs.w };
        let sy = if trs.flip_v { -trs.h } else { trs.h };
        Self::new(cos * sx, sin * sx, -sin * sy, cos * sy, trs.x, trs.y)
    }

    /// Decomposes a matrix back into canonical fields.
    ///
    /// Returns `None` for singular matrices (zero-size nodes are legal and simply have no
    /// unique decomposition — ADR-002 §7) and for matrices with skew (not representable in
    /// v1 — ADR-002 §8). The returned fields are in the canonical math frame
    /// (`flip_h == false`; see [`Trs::normalized`]). Production paths always work from
    /// canonical fields (CAN-TR-6 for the transform system); this exists for tests, import
    /// flows and diagnostics — never as a mid-gesture round-trip.
    #[must_use]
    pub fn decompose(self) -> Option<Trs> {
        if !self.is_finite() {
            return None;
        }
        let w = self.a.hypot(self.b);
        let h = self.c.hypot(self.d);
        if w == 0.0 || h == 0.0 {
            return None; // singular: no unique angle/size decomposition
        }
        // Columns must be orthogonal (no skew). Relative check: absolute tolerance would be
        // scale-dependent (EPS_REL; CAN-NUM-5).
        let dot = self.a * self.c + self.b * self.d;
        if dot.abs() > crate::EPS_REL * w * h {
            return None;
        }
        let rotation_deg = crate::angle::normalize_deg(self.b.atan2(self.a).to_degrees());
        Some(Trs {
            x: self.tx,
            y: self.ty,
            w,
            h,
            rotation_deg,
            flip_h: false,
            flip_v: self.determinant() < 0.0,
        })
    }

    /// Applies the transform to a point: `p' = Mat2 · p`.
    #[must_use]
    pub fn apply(self, p: Vec2) -> Vec2 {
        Vec2::new(
            self.a * p.x + self.c * p.y + self.tx,
            self.b * p.x + self.d * p.y + self.ty,
        )
    }

    /// Maps a local-space point into world space via this matrix (identity alias of
    /// [`Mat2::apply`], named for the canonical conversion vocabulary).
    #[must_use]
    pub fn local_to_world(self, p: Vec2) -> Vec2 {
        self.apply(p)
    }

    /// Maps a world-space point into local space. `None` when the matrix is singular
    /// (zero-size node).
    #[must_use]
    pub fn world_to_local(self, p: Vec2) -> Option<Vec2> {
        self.invert().map(|inv| inv.apply(p))
    }

    /// The 2×2 determinant (`a·d − b·c`); its sign is the orientation (flip state) of the
    /// linear part.
    #[must_use]
    pub fn determinant(self) -> f64 {
        self.a * self.d - self.b * self.c
    }

    /// The inverse transform, or `None` when the linear part is singular
    /// (`det == 0`, including zero-size nodes) or non-finite.
    #[must_use]
    pub fn invert(self) -> Option<Self> {
        let det = self.determinant();
        if !det.is_finite() || det == 0.0 {
            return None;
        }
        let inv_det = 1.0 / det;
        let a = self.d * inv_det;
        let b = -self.b * inv_det;
        let c = -self.c * inv_det;
        let d = self.a * inv_det;
        Some(Self::new(
            a,
            b,
            c,
            d,
            -(a * self.tx + c * self.ty),
            -(b * self.tx + d * self.ty),
        ))
    }

    /// True when every entry is finite.
    #[must_use]
    pub fn is_finite(self) -> bool {
        self.a.is_finite()
            && self.b.is_finite()
            && self.c.is_finite()
            && self.d.is_finite()
            && self.tx.is_finite()
            && self.ty.is_finite()
    }

    /// Entry-wise approximate equality (max absolute difference ≤ `eps`).
    #[must_use]
    pub fn approx_eq(self, other: Self, eps: f64) -> bool {
        (self.a - other.a).abs() <= eps
            && (self.b - other.b).abs() <= eps
            && (self.c - other.c).abs() <= eps
            && (self.d - other.d).abs() <= eps
            && (self.tx - other.tx).abs() <= eps
            && (self.ty - other.ty).abs() <= eps
    }

    /// Convenience: entry-wise equality within the canonical matrix tolerance
    /// ([`EPS_MATRIX`]).
    #[must_use]
    pub fn approx_eq_default(self, other: Self) -> bool {
        self.approx_eq(other, EPS_MATRIX)
    }
}

impl Default for Mat2 {
    fn default() -> Self {
        Self::IDENTITY
    }
}

impl Mul for Mat2 {
    type Output = Self;

    /// Matrix composition: `(m1 * m2).apply(p) == m1.apply(m2.apply(p))`.
    /// Chains accumulate root→leaf by left multiplication (CAN-COORD-3).
    fn mul(self, rhs: Self) -> Self {
        Self::new(
            self.a * rhs.a + self.c * rhs.b,
            self.b * rhs.a + self.d * rhs.b,
            self.a * rhs.c + self.c * rhs.d,
            self.b * rhs.c + self.d * rhs.d,
            self.a * rhs.tx + self.c * rhs.ty + self.tx,
            self.b * rhs.tx + self.d * rhs.ty + self.ty,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::EPS_MATRIX;

    #[test]
    fn identity_is_neutral() {
        let m = Mat2::from_trs(Trs::new(3.0, -4.0, 10.0, 5.0, 33.0, false, false));
        assert!(m.approx_eq_default(m * Mat2::IDENTITY));
        assert!(m.approx_eq_default(Mat2::IDENTITY * m));
    }

    #[test]
    fn rotation_90_exact_convention() {
        let m = Mat2::rotation_deg(90.0);
        // [[0, -1, 0],[1, 0, 0]] — column-major 6-tuple.
        assert!(m.approx_eq_default(Mat2::new(0.0, 1.0, -1.0, 0.0, 0.0, 0.0)));
        assert!(
            m.apply(Vec2::new(1.0, 0.0))
                .approx_eq(Vec2::new(0.0, 1.0), EPS_MATRIX)
        );
    }

    #[test]
    fn composition_matches_sequential_application() {
        let m1 = Mat2::translation(5.0, -2.0);
        let m2 = Mat2::rotation_deg(30.0);
        let p = Vec2::new(1.5, 7.25);
        let composed = (m1 * m2).apply(p);
        let sequential = m1.apply(m2.apply(p));
        assert!(composed.approx_eq(sequential, EPS_MATRIX));
    }

    #[test]
    fn invert_roundtrips_and_translates_inversely() {
        let m = Mat2::from_trs(Trs::new(12.0, 8.0, 40.0, 20.0, 37.5, true, false));
        let inv = m.invert().expect("non-singular");
        assert!(
            inv.apply(m.apply(Vec2::new(3.0, 4.0)))
                .approx_eq(Vec2::new(3.0, 4.0), EPS_MATRIX)
        );
        assert!(
            m.apply(Vec2::new(0.0, 0.0))
                .approx_eq(Vec2::new(12.0, 8.0), EPS_MATRIX)
        );
    }

    #[test]
    fn singular_matrices_have_no_inverse() {
        assert!(Mat2::scale(0.0, 5.0).invert().is_none());
        assert!(Mat2::scale(5.0, 0.0).invert().is_none());
        assert!(Mat2::scale(0.0, 0.0).invert().is_none());
        assert!(Mat2::new(1.0, 1.0, 1.0, 1.0, 0.0, 0.0).invert().is_none()); // collinear
        assert!(
            Mat2::new(f64::NAN, 0.0, 0.0, 1.0, 0.0, 0.0)
                .invert()
                .is_none()
        );
    }
}
