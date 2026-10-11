//! Canonical transform values (ADR-002): decomposed fields — the ONLY stored form of "where
//! a node sits" — plus their composition and decomposition math.
//!
//! The seven fields are the canonical representation (ADR-002 §1): `x, y` (parent space),
//! `w, h` (extent along the node's own axes, stored ≥ 0), `rotation_deg` (canonical range
//! `(−180, 180]`), `flip_h`, `flip_v` (booleans — ADR-002 §6). Matrices are derived caches;
//! production paths decompose from these fields, never from raw matrices (CAN-TR-6).
//!
//! **Canonical math frame:** [`Trs::normalized`] converts any field combination into the
//! comparison frame used by decomposition (`flip_h == false`, mirror expressed as `flip_v`
//! plus a 180° rotation when needed). The *stored* form keeps the user-authored flags —
//! flipping horizontally is a first-class user action (flip_h) and must not be silently
//! rewritten into "rotation 180 + flip_v" in the data or the panels.
//!
//! **Storage mirror (document phase):** `.gip` (ADR-003 #11) stores these seven values
//! explicitly. `gip-document` cannot depend on this crate (dependency law), so
//! `document::Transform` mirrors this struct field-for-field and the single adapter lives
//! with the transform system; a drift test pins the field sets when the document phase lands.

use crate::angle::normalize_deg;
use crate::mat2::Mat2;

/// Canonical decomposed transform fields (`T·R·S` parameters), all `f64` except the flips.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Trs {
    /// Translation x in parent space.
    pub x: f64,
    /// Translation y in parent space.
    pub y: f64,
    /// Width along the node's own x axis (stored ≥ 0; negative input means "mirrored").
    pub w: f64,
    /// Height along the node's own y axis (stored ≥ 0).
    pub h: f64,
    /// Rotation in degrees about the top-left origin (canonical range `(−180, 180]`).
    pub rotation_deg: f64,
    /// Horizontal mirror flag.
    pub flip_h: bool,
    /// Vertical mirror flag.
    pub flip_v: bool,
}

impl Trs {
    /// Creates raw fields (no normalization — see [`Trs::normalized`]).
    #[must_use]
    pub const fn new(
        x: f64,
        y: f64,
        w: f64,
        h: f64,
        rotation_deg: f64,
        flip_h: bool,
        flip_v: bool,
    ) -> Self {
        Self {
            x,
            y,
            w,
            h,
            rotation_deg,
            flip_h,
            flip_v,
        }
    }

    /// The canonical math-frame normalization (ADR-002 §3/§5/§6):
    ///
    /// 1. negative sizes are folded into flips (`w < 0` ⇒ `w = |w|`, `flip_h` toggled);
    /// 2. the rotation is normalized into `(−180, 180]`;
    /// 3. the mirror is expressed in the `flip_v` frame: `flip_v ^= flip_h`, `flip_h = false`,
    ///    with +180° added when a horizontal fold occurred.
    ///
    /// This is the form decomposition returns; it is NOT the stored form (see module docs).
    #[must_use]
    pub fn normalized(mut self) -> Self {
        if self.w < 0.0 {
            self.w = -self.w;
            self.flip_h = !self.flip_h;
        }
        if self.h < 0.0 {
            self.h = -self.h;
            self.flip_v = !self.flip_v;
        }
        let mut rotation = self.rotation_deg;
        if self.flip_h {
            rotation += 180.0;
        }
        let flip_v = self.flip_h ^ self.flip_v;
        Self {
            x: self.x,
            y: self.y,
            w: self.w,
            h: self.h,
            rotation_deg: normalize_deg(rotation),
            flip_h: false,
            flip_v,
        }
    }

    /// Builds the canonical local matrix `T(x, y) · R(θ) · S(sx, sy)` (ADR-002 §3).
    /// Fields are used as given; pass [`Trs::normalized`] first at untrusted input edges.
    #[must_use]
    pub fn to_matrix(&self) -> Mat2 {
        Mat2::from_trs(*self)
    }

    /// Field-wise approximate equality (units within `eps_units`, angle within `eps_angle`).
    #[must_use]
    pub fn approx_eq(self, other: Self, eps_units: f64, eps_angle: f64) -> bool {
        (self.x - other.x).abs() <= eps_units
            && (self.y - other.y).abs() <= eps_units
            && (self.w - other.w).abs() <= eps_units
            && (self.h - other.h).abs() <= eps_units
            && (self.rotation_deg - other.rotation_deg).abs() <= eps_angle
            && self.flip_h == other.flip_h
            && self.flip_v == other.flip_v
    }
}

impl Default for Trs {
    fn default() -> Self {
        Self::new(0.0, 0.0, 0.0, 0.0, 0.0, false, false)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::EPS_MATRIX;

    #[test]
    fn ninety_degree_rotation_has_exact_snapped_entries() {
        let m = Trs::new(10.0, 20.0, 4.0, 2.0, 90.0, false, false).to_matrix();
        // Expected [[0, -2, 10],[4, 0, 20]] — snapped commits are exact within EPS_MATRIX.
        assert!(m.approx_eq(Mat2::new(0.0, 4.0, -2.0, 0.0, 10.0, 20.0), EPS_MATRIX));
    }

    #[test]
    fn zero_crossing_normalization_folds_negative_sizes_into_flips() {
        // Negative sizes (resize past zero — ADR-002 §7) are legal INPUTS to normalization;
        // the invariant is matrix equivalence plus a canonical field frame.
        let cases = [
            Trs::new(0.0, 0.0, -10.0, 5.0, 0.0, false, false),
            Trs::new(0.0, 0.0, -10.0, -5.0, 0.0, false, false),
            Trs::new(0.0, 0.0, 10.0, -5.0, 30.0, false, false),
            Trs::new(0.0, 0.0, -10.0, 5.0, -120.0, true, false),
        ];
        for raw in cases {
            let norm = raw.normalized();
            assert!(
                norm.w >= 0.0 && norm.h >= 0.0,
                "sizes must be stored non-negative"
            );
            assert!(!norm.flip_h, "canonical math frame uses flip_v only");
            assert!(norm.rotation_deg > -180.0 && norm.rotation_deg <= 180.0);
            assert!(
                norm.to_matrix().approx_eq(raw.to_matrix(), EPS_MATRIX),
                "normalization changed the transform: {raw:?} → {norm:?}"
            );
        }
        // Single-axis mirror: (−10, 5) ≡ 180° with flip_v (matrix-identical).
        let t = Trs::new(0.0, 0.0, -10.0, 5.0, 0.0, false, false).normalized();
        assert!(t.flip_v && !t.flip_h);
        assert!((t.rotation_deg - 180.0).abs() <= crate::EPS_ANGLE);
        // Both axes: (−10, −5) ≡ 180° with no mirror.
        let t2 = Trs::new(0.0, 0.0, -10.0, -5.0, 0.0, false, false).normalized();
        assert!(!t2.flip_h && !t2.flip_v);
        assert!((t2.rotation_deg - 180.0).abs() <= crate::EPS_ANGLE);
    }

    #[test]
    fn canonical_frame_moves_horizontal_flip_into_rotation_plus_flip_v() {
        let t = Trs::new(0.0, 0.0, 10.0, 5.0, 30.0, true, false).normalized();
        assert!(!t.flip_h);
        assert!(t.flip_v);
        assert!((t.rotation_deg - normalize_deg(30.0 + 180.0)).abs() <= crate::EPS_ANGLE);
        // Both flips ≡ +180° with no mirror.
        let both = Trs::new(0.0, 0.0, 10.0, 5.0, 30.0, true, true).normalized();
        assert!(!both.flip_h && !both.flip_v);
        assert!((both.rotation_deg - normalize_deg(30.0 + 180.0)).abs() <= crate::EPS_ANGLE);
    }

    #[test]
    fn decompose_is_none_for_singular_and_skewed_matrices() {
        assert!(Mat2::scale(0.0, 4.0).decompose().is_none());
        // Shear: columns not orthogonal.
        let skew = Mat2::new(1.0, 0.0, 0.5, 1.0, 0.0, 0.0);
        assert!(skew.decompose().is_none());
    }

    #[test]
    fn decompose_returns_canonical_math_frame() {
        let t = Trs::new(5.0, 7.0, 3.0, 9.0, 120.0, true, true).normalized();
        let d = t.to_matrix().decompose().expect("decomposable");
        assert!(d.approx_eq(t, crate::EPS_ROUNDTRIP, crate::EPS_ANGLE));
    }
}
