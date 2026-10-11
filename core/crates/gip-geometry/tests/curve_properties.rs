//! Randomized property checks for the cubic/path laws. Deterministic: splitmix64 with fixed
//! seeds (same policy as gip-coords — dev-dependencies stay out, see dependencies.md §5).

use gip_coords::Vec2;
use gip_document::path::{FillRule, Path, Subpath, Vertex};
use gip_geometry::ops::insert_vertex;
use gip_geometry::{Cubic, flatten_subpath, segment_cubic, segments};

struct Rng(u64);

impl Rng {
    fn next(&mut self) -> u64 {
        self.0 = self.0.wrapping_add(0x9E37_79B9_7F4A_7C15);
        let mut z = self.0;
        z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
        z ^ (z >> 31)
    }
    fn unit(&mut self) -> f64 {
        (self.next() >> 11) as f64 / (1u64 << 53) as f64
    }
    fn signed(&mut self, m: f64) -> f64 {
        (self.unit() * 2.0 - 1.0) * m
    }
    fn point(&mut self, m: f64) -> Vec2 {
        Vec2::new(self.signed(m), self.signed(m))
    }
}

#[test]
fn random_cubics_flatten_within_tolerance() {
    let mut rng = Rng(0xA11CE);
    for _ in 0..300 {
        let c = Cubic::new(
            rng.point(500.0),
            rng.point(500.0),
            rng.point(500.0),
            rng.point(500.0),
        );
        let tol = 0.05 + rng.unit();
        let mut pts = vec![c.p0];
        c.flatten_into(tol, &mut pts);
        for i in 0..=300 {
            let p = c.at(i as f64 / 300.0);
            // Module-doc guarantee: every curve point is within tol of SOME chord's line
            // (the chord whose piece contains it). Checked as: min over chords of line distance.
            let d = pts
                .windows(2)
                .map(|w| {
                    let ab = w[1] - w[0];
                    let len = ab.length_sq().sqrt();
                    if len == 0.0 {
                        p.distance_sq(w[0]).sqrt()
                    } else {
                        ab.cross(p - w[0]).abs() / len
                    }
                })
                .fold(f64::INFINITY, f64::min);
            assert!(d <= tol + 1e-7, "line deviation {d} > tol {tol}");
        }
    }
}

#[test]
fn random_insertions_never_change_the_curve() {
    let mut rng = Rng(0xBEEF);
    for _ in 0..200 {
        let mut a = Vertex::corner([0.0, 0.0]);
        a.handle_out = {
            let p = rng.point(200.0);
            [p.x, p.y]
        };
        let mut b = Vertex::corner({
            let p = rng.point(200.0);
            [p.x, p.y]
        });
        b.handle_in = {
            let p = rng.point(200.0);
            [p.x, p.y]
        };
        let path = Path {
            subpaths: vec![Subpath {
                vertices: vec![a, b],
                closed: false,
            }],
            fill_rule: FillRule::NonZero,
        };
        let original = segments(&path)[0].cubic;
        let t = rng.unit();
        let after = insert_vertex(&path, 0, 0, t).unwrap();
        let left = segment_cubic(&after.subpaths[0], 0);
        let right = segment_cubic(&after.subpaths[0], 1);
        for i in 0..=40 {
            let u = i as f64 / 40.0;
            assert!(left.at(u).approx_eq(original.at(t * u), 1e-8));
            assert!(right.at(u).approx_eq(original.at(t + (1.0 - t) * u), 1e-8));
        }
        // The flattened split outline stays on the original curve (within the tolerance).
        let halves = flatten_subpath(&after.subpaths[0], 0.01);
        for q in halves.iter().step_by(7) {
            let d = original.nearest(*q).2.sqrt();
            assert!(d <= 0.01 + 1e-6, "split outline left the curve by {d}");
        }
    }
}
