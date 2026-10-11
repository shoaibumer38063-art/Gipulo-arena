//! Cubic Bézier math — the one segment primitive (ADR-006 #1). Evaluation, de Casteljau
//! splitting, exact extrema for tight bounds, deterministic flattening, nearest point.
//!
//! Flatness guarantee (proven): every emitted chord lies on a line from which every point of
//! its curve piece is at most `tol` away. Proof: the signed distance to the chord line is a
//! Bernstein combination of the two control distances, bounded by `0.75 · max(d1, d2) ≤ tol`.
//! This bounds distance to the chord's LINE; distance to the finite chord can exceed it by the
//! tangential overshoot near the endpoints, which is why the tests check the line bound.
//! Subdivision depth is capped at [`MAX_FLATTEN_DEPTH`]; requests finer than the cap are
//! honoured only to the cap (caller guidance: tolerance ≥ extent / 2^16).

use gip_coords::Vec2;

/// Subdivision depth cap for flattening (2^16 pieces per curve at most).
pub const MAX_FLATTEN_DEPTH: u32 = 16;

/// Bisection iterations in the final bracket refinement of [`Cubic::nearest`].
const NEAREST_ITERATIONS: usize = 64;
/// Branch-and-bound depth cap for [`Cubic::nearest`].
const MAX_NEAREST_DEPTH: u32 = 60;
/// Newton polish steps after the branch-and-bound search in [`Cubic::nearest`].
const NEAREST_POLISH_STEPS: usize = 4;
/// Leaf bracket size (world units) at which [`Cubic::nearest`] stops subdividing and refines.
/// Only bounds the search cost; the final parameter comes from bisection, not from this size.
const NEAREST_LEAF_SIZE: f64 = 1e-6;

/// A cubic Bézier segment in path-local coordinates.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Cubic {
    /// Start anchor.
    pub p0: Vec2,
    /// Start control point.
    pub p1: Vec2,
    /// End control point.
    pub p2: Vec2,
    /// End anchor.
    pub p3: Vec2,
}

impl Cubic {
    /// Creates a cubic from its four points.
    #[must_use]
    pub const fn new(p0: Vec2, p1: Vec2, p2: Vec2, p3: Vec2) -> Self {
        Self { p0, p1, p2, p3 }
    }

    /// Exact quadratic → cubic conversion (ADR-006 #1): `c1 = p0 + ⅔(q − p0)`, `c2 = p2 + ⅔(q − p2)`.
    #[must_use]
    pub fn from_quadratic(p0: Vec2, q: Vec2, p2: Vec2) -> Self {
        let c1 = p0 + (q - p0) * (2.0 / 3.0);
        let c2 = p2 + (q - p2) * (2.0 / 3.0);
        Self::new(p0, c1, c2, p2)
    }

    /// True when both controls coincide with their endpoints — the data definition of a
    /// straight segment (ADR-006 #6). Exact comparison by design.
    #[must_use]
    pub fn is_straight(&self) -> bool {
        self.p1 == self.p0 && self.p2 == self.p3
    }

    /// Point on the curve at parameter `t` (Bernstein form).
    #[must_use]
    pub fn at(&self, t: f64) -> Vec2 {
        let mt = 1.0 - t;
        let a = mt * mt * mt;
        let b = 3.0 * mt * mt * t;
        let c = 3.0 * mt * t * t;
        let d = t * t * t;
        Vec2::new(
            a * self.p0.x + b * self.p1.x + c * self.p2.x + d * self.p3.x,
            a * self.p0.y + b * self.p1.y + c * self.p2.y + d * self.p3.y,
        )
    }

    /// Splits at `t` by de Casteljau; the two halves reproduce the original curve exactly
    /// (up to floating-point rounding of the interpolation).
    #[must_use]
    pub fn split(&self, t: f64) -> (Cubic, Cubic) {
        let p01 = self.p0.lerp(self.p1, t);
        let p12 = self.p1.lerp(self.p2, t);
        let p23 = self.p2.lerp(self.p3, t);
        let p012 = p01.lerp(p12, t);
        let p123 = p12.lerp(p23, t);
        let s = p012.lerp(p123, t);
        (
            Cubic::new(self.p0, p01, p012, s),
            Cubic::new(s, p123, p23, self.p3),
        )
    }

    /// Tight axis-aligned bounds (min, max) from exact extrema, not control points (CAN-GEOM-4
    /// precision: control hulls overstate curves).
    #[must_use]
    pub fn tight_bounds(&self) -> (Vec2, Vec2) {
        let mut min = self.p0.min_components(self.p3);
        let mut max = self.p0.max_components(self.p3);
        for t in self.extrema_params() {
            let p = self.at(t);
            min = min.min_components(p);
            max = max.max_components(p);
        }
        (min, max)
    }

    /// Parameters in (0, 1) where the x or y derivative vanishes.
    fn extrema_params(&self) -> impl Iterator<Item = f64> {
        let xs = derivative_roots(self.p0.x, self.p1.x, self.p2.x, self.p3.x);
        let ys = derivative_roots(self.p0.y, self.p1.y, self.p2.y, self.p3.y);
        xs.into_iter().chain(ys).flatten()
    }

    /// Appends the flattened curve after the start point (the caller pushes `p0`).
    /// Output points are in order, end with `p3`, and deviate from the curve by ≤ `tol`.
    pub fn flatten_into(&self, tol: f64, out: &mut Vec<Vec2>) {
        self.flatten_rec(tol, 0, out);
    }

    fn flatten_rec(&self, tol: f64, depth: u32, out: &mut Vec<Vec2>) {
        if depth >= MAX_FLATTEN_DEPTH || self.is_flat(tol) {
            out.push(self.p3);
            return;
        }
        let (left, right) = self.split(0.5);
        left.flatten_rec(tol, depth + 1, out);
        right.flatten_rec(tol, depth + 1, out);
    }

    /// Flatness test: both controls within `tol` of the chord's line (or of the start point
    /// when the chord is degenerate). Implies the line-deviation bound stated in the module docs.
    fn is_flat(&self, tol: f64) -> bool {
        let d = self.p3 - self.p0;
        let len_sq = d.length_sq();
        if len_sq == 0.0 {
            return (self.p1 - self.p0).length_sq() <= tol * tol
                && (self.p2 - self.p0).length_sq() <= tol * tol;
        }
        let len = len_sq.sqrt();
        let d1 = d.cross(self.p1 - self.p0).abs() / len;
        let d2 = d.cross(self.p2 - self.p0).abs() / len;
        d1 <= tol && d2 <= tol
    }

    /// Nearest point on the curve to `p`: `(t, point, distance_sq)`.
    #[must_use]
    pub fn nearest(&self, p: Vec2) -> (f64, Vec2, f64) {
        self.nearest_within(p, f64::INFINITY)
            .expect("an unbounded search always finds a point")
    }

    /// Nearest point strictly closer than `bound_sq` (a squared distance). Returns `None` when
    /// the curve has nothing closer than the bound — this is how a path-level search shares one
    /// running best distance across segments and skips most of them cheaply.
    ///
    /// Method: branch-and-bound over sub-curves. A sub-curve is pruned when the distance from
    /// `p` to the bounding box of its control points (which contains the sub-curve) is at least
    /// the best found so far — so no branch that could hold the minimum is skipped. Nearer
    /// children are visited first to tighten the bound early. Surviving leaves (size ≤
    /// [`NEAREST_LEAF_SIZE`]) are refined by bisection of `g(t) = (B(t) − p) · B′(t)`, then a
    /// few guarded Newton steps polish the result.
    #[must_use]
    pub fn nearest_within(&self, p: Vec2, bound_sq: f64) -> Option<(f64, Vec2, f64)> {
        let mut best = (None::<f64>, bound_sq);
        for (t, end) in [(0.0, self.p0), (1.0, self.p3)] {
            let d = end.distance_sq(p);
            if d < best.1 {
                best = (Some(t), d);
            }
        }
        nearest_rec(self, *self, 0.0, 1.0, p, &mut best, 0);
        let mut t = best.0?;
        // Newton polish on the offset condition. Accepted only while it moves toward the root
        // (|g| shrinks) at a positive-curvature minimum; distance comparisons are avoided because
        // near the root the distance change is below rounding noise.
        for _ in 0..NEAREST_POLISH_STEPS {
            let off = self.at(t) - p;
            let g = off.dot(self.derivative(t));
            let dg = self.derivative(t).length_sq() + off.dot(self.second_derivative(t));
            if dg.partial_cmp(&0.0) != Some(std::cmp::Ordering::Greater) || !dg.is_finite() {
                break;
            }
            let candidate = (t - g / dg).clamp(0.0, 1.0);
            let g_next = (self.at(candidate) - p).dot(self.derivative(candidate));
            if g_next.abs() < g.abs() {
                t = candidate;
            } else {
                break;
            }
        }
        let point = self.at(t);
        Some((t, point, point.distance_sq(p)))
    }

    /// Second derivative `B″(t)`.
    #[must_use]
    pub fn second_derivative(&self, t: f64) -> Vec2 {
        let a = self.p2 - (self.p1 * 2.0) + self.p0;
        let b = self.p3 - (self.p2 * 2.0) + self.p1;
        (a * (6.0 * (1.0 - t))) + (b * (6.0 * t))
    }

    /// First derivative `B′(t)` (not normalized).
    #[must_use]
    pub fn derivative(&self, t: f64) -> Vec2 {
        let mt = 1.0 - t;
        let d0 = self.p1 - self.p0;
        let d1 = self.p2 - self.p1;
        let d2 = self.p3 - self.p2;
        (d0 * (3.0 * mt * mt)) + (d1 * (6.0 * mt * t)) + (d2 * (3.0 * t * t))
    }
}

/// Branch-and-bound step for [`Cubic::nearest`]. `sub` is the piece of `orig` on `[t0, t1]`.
fn nearest_rec(
    orig: &Cubic,
    sub: Cubic,
    t0: f64,
    t1: f64,
    p: Vec2,
    best: &mut (Option<f64>, f64),
    depth: u32,
) {
    let min = sub
        .p0
        .min_components(sub.p1)
        .min_components(sub.p2)
        .min_components(sub.p3);
    let max = sub
        .p0
        .max_components(sub.p1)
        .max_components(sub.p2)
        .max_components(sub.p3);
    let dx = (min.x - p.x).max(p.x - max.x).max(0.0);
    let dy = (min.y - p.y).max(p.y - max.y).max(0.0);
    if dx * dx + dy * dy >= best.1 {
        return;
    }
    let size = (max - min).x.max((max - min).y);
    if size <= NEAREST_LEAF_SIZE || depth >= MAX_NEAREST_DEPTH {
        let t = refine_on_bracket(orig, p, t0, t1);
        let d = orig.at(t).distance_sq(p);
        if d < best.1 {
            *best = (Some(t), d);
        }
        return;
    }
    // Nearer child first: establishes a tight bound early, which is what makes pruning effective.
    let tm = 0.5 * (t0 + t1);
    let (left, right) = sub.split(0.5);
    let box_dist = |c: &Cubic| {
        let lo =
            c.p0.min_components(c.p1)
                .min_components(c.p2)
                .min_components(c.p3);
        let hi =
            c.p0.max_components(c.p1)
                .max_components(c.p2)
                .max_components(c.p3);
        let dx = (lo.x - p.x).max(p.x - hi.x).max(0.0);
        let dy = (lo.y - p.y).max(p.y - hi.y).max(0.0);
        dx * dx + dy * dy
    };
    let (first, second) = if box_dist(&left) <= box_dist(&right) {
        ((left, t0, tm), (right, tm, t1))
    } else {
        ((right, tm, t1), (left, t0, tm))
    };
    nearest_rec(orig, first.0, first.1, first.2, p, best, depth + 1);
    nearest_rec(orig, second.0, second.1, second.2, p, best, depth + 1);
}

/// Parameter of the closest point within a tiny bracket: bisects the offset condition when it
/// changes sign, otherwise returns the better bracket end.
fn refine_on_bracket(orig: &Cubic, p: Vec2, t0: f64, t1: f64) -> f64 {
    let g = |t: f64| (orig.at(t) - p).dot(orig.derivative(t));
    let (ga, gb) = (g(t0), g(t1));
    if (ga <= 0.0) == (gb <= 0.0) {
        return if orig.at(t0).distance_sq(p) <= orig.at(t1).distance_sq(p) {
            t0
        } else {
            t1
        };
    }
    let (mut lo, mut hi) = (t0, t1);
    let lo_positive = ga > 0.0;
    for _ in 0..NEAREST_ITERATIONS {
        let mid = 0.5 * (lo + hi);
        if (g(mid) > 0.0) == lo_positive {
            lo = mid;
        } else {
            hi = mid;
        }
    }
    0.5 * (lo + hi)
}

/// Roots in (0, 1) of the derivative of one coordinate's cubic Bernstein form.
/// Derivative/3 = a·t² + b·t + c with a = d0 − 2d1 + d2, b = 2(d1 − d0), c = d0.
fn derivative_roots(p0: f64, p1: f64, p2: f64, p3: f64) -> [Option<f64>; 2] {
    let d0 = p1 - p0;
    let d1 = p2 - p1;
    let d2 = p3 - p2;
    let a = d0 - 2.0 * d1 + d2;
    let b = 2.0 * (d1 - d0);
    let c = d0;
    let in_open_unit = |t: f64| (t > 0.0 && t < 1.0).then_some(t);
    if a == 0.0 {
        if b == 0.0 {
            return [None, None];
        }
        return [in_open_unit(-c / b), None];
    }
    let disc = b * b - 4.0 * a * c;
    if disc < 0.0 {
        return [None, None];
    }
    let sq = disc.sqrt();
    // Numerically stable quadratic roots.
    let q = if b >= 0.0 {
        -0.5 * (b + sq)
    } else {
        -0.5 * (b - sq)
    };
    let r1 = q / a;
    let r2 = if q != 0.0 { c / q } else { r1 };
    [in_open_unit(r1), in_open_unit(r2)]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn v(x: f64, y: f64) -> Vec2 {
        Vec2::new(x, y)
    }

    #[test]
    fn endpoints_are_exact() {
        let c = Cubic::new(v(0.0, 0.0), v(1.0, 2.0), v(3.0, 2.0), v(4.0, 0.0));
        assert_eq!(c.at(0.0), c.p0);
        assert_eq!(c.at(1.0), c.p3);
    }

    #[test]
    fn split_reproduces_the_curve() {
        let c = Cubic::new(v(0.0, 0.0), v(10.0, 30.0), v(-5.0, 20.0), v(8.0, -4.0));
        let (l, r) = c.split(0.37);
        for i in 0..=20 {
            let t = i as f64 / 20.0;
            assert!(l.at(t).approx_eq(c.at(t * 0.37), 1e-12));
            assert!(r.at(t).approx_eq(c.at(0.37 + t * 0.63), 1e-12));
        }
    }

    #[test]
    fn tight_bounds_match_brute_force_sampling() {
        let c = Cubic::new(v(0.0, 0.0), v(10.0, 40.0), v(-20.0, 40.0), v(10.0, 0.0));
        let (min, max) = c.tight_bounds();
        let (mut smin, mut smax) = (
            v(f64::INFINITY, f64::INFINITY),
            v(f64::NEG_INFINITY, f64::NEG_INFINITY),
        );
        for i in 0..=100_000 {
            let p = c.at(i as f64 / 100_000.0);
            smin = smin.min_components(p);
            smax = smax.max_components(p);
        }
        assert!(min.approx_eq(smin, 1e-6), "min {min:?} vs sampled {smin:?}");
        assert!(max.approx_eq(smax, 1e-6), "max {max:?} vs sampled {smax:?}");
        // Control-point hull is strictly larger here — proves the extrema matter.
        assert!(max.y < 40.0 && min.x > -20.0);
    }

    #[test]
    fn straight_definition_is_exact() {
        let a = v(1.0, 1.0);
        let b = v(5.0, 9.0);
        assert!(Cubic::new(a, a, b, b).is_straight());
        assert!(!Cubic::new(a, v(1.0, 1.0 + 1e-15), b, b).is_straight());
    }

    #[test]
    fn flatten_stays_within_tolerance_and_ends_exactly() {
        let c = Cubic::new(v(0.0, 0.0), v(100.0, 300.0), v(-200.0, 300.0), v(50.0, 0.0));
        for tol in [1.0, 0.1, 0.01] {
            let mut pts = vec![c.p0];
            c.flatten_into(tol, &mut pts);
            assert_eq!(*pts.last().unwrap(), c.p3);
            // Every true curve sample lies within tol of the polyline.
            for i in 0..=2000 {
                let p = c.at(i as f64 / 2000.0);
                let d = pts
                    .windows(2)
                    .map(|w| seg_dist(p, w[0], w[1]))
                    .fold(f64::INFINITY, f64::min);
                assert!(d <= tol + 1e-9, "tol {tol}: sample off by {d}");
            }
        }
    }

    #[test]
    fn nearest_matches_brute_force() {
        let c = Cubic::new(v(0.0, 0.0), v(30.0, 80.0), v(70.0, -40.0), v(100.0, 10.0));
        for (px, py) in [(50.0, 50.0), (-20.0, 5.0), (120.0, -30.0), (33.3, 12.1)] {
            let p = v(px, py);
            let (_, _, d_sq) = c.nearest(p);
            let brute = (0..=200_000)
                .map(|i| c.at(i as f64 / 200_000.0).distance_sq(p))
                .fold(f64::INFINITY, f64::min);
            assert!(
                d_sq <= brute + 1e-9,
                "refined {d_sq} worse than brute {brute}"
            );
            assert!(
                (d_sq - brute).abs() <= 1e-6 * (1.0 + brute),
                "not converged at {p:?}"
            );
        }
    }

    #[test]
    fn nearest_point_satisfies_the_perpendicularity_condition() {
        // At the true minimum the offset is perpendicular to the tangent: g(t) = 0. A grid
        // search cannot resolve positions finer than its spacing, so the optimality condition
        // is the precise check; the grid only bounds distance from above.
        let c = Cubic::new(v(0.0, 0.0), v(30.0, 80.0), v(70.0, -40.0), v(100.0, 10.0));
        for (px, py) in [(50.0, 50.0), (-20.0, 5.0), (33.3, 12.1)] {
            let p = v(px, py);
            let (t, pt, d_sq) = c.nearest(p);
            assert!(pt.approx_eq(c.at(t), 1e-12));
            // Interior minima satisfy the condition; endpoint minima need not (they are clamps).
            if t > 0.0 && t < 1.0 {
                let residual = (c.at(t) - p).dot(c.derivative(t)).abs();
                let scale = (c.at(t) - p).length_sq().sqrt() * c.derivative(t).length_sq().sqrt();
                assert!(
                    residual <= 1e-9 * (1.0 + scale),
                    "not perpendicular: {residual}"
                );
            }
            let grid = (0..=400_000)
                .map(|i| c.at(i as f64 / 400_000.0).distance_sq(p))
                .fold(f64::INFINITY, f64::min);
            assert!(
                d_sq <= grid + 1e-9,
                "worse than grid search: {d_sq} vs {grid}"
            );
        }
    }

    #[test]
    fn derivative_matches_finite_difference() {
        let c = Cubic::new(v(0.0, 0.0), v(30.0, 80.0), v(70.0, -40.0), v(100.0, 10.0));
        let h = 1e-6;
        for i in 1..10 {
            let t = i as f64 / 10.0;
            let fd = (c.at(t + h) - c.at(t - h)) * (0.5 / h);
            assert!(c.derivative(t).approx_eq(fd, 1e-5));
        }
    }

    #[test]
    fn quadratic_conversion_is_exact() {
        let (p0, q, p2) = (v(0.0, 0.0), v(3.0, 6.0), v(6.0, 0.0));
        let c = Cubic::from_quadratic(p0, q, p2);
        for i in 0..=10 {
            let t = i as f64 / 10.0;
            let quad = p0 * ((1.0 - t) * (1.0 - t)) + q * (2.0 * (1.0 - t) * t) + p2 * (t * t);
            assert!(c.at(t).approx_eq(quad, 1e-12));
        }
    }

    pub(crate) fn seg_dist(p: Vec2, a: Vec2, b: Vec2) -> f64 {
        let ab = b - a;
        let len = ab.length_sq();
        let t = if len == 0.0 {
            0.0
        } else {
            ((p - a).dot(ab) / len).clamp(0.0, 1.0)
        };
        p.distance_sq(a + ab * t).sqrt()
    }
}
