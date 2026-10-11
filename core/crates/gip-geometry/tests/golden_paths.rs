//! VG7 golden path fixtures: every pathology the vector rules name, each run through the
//! measurement, flattening, hit, and editing operations. Fixtures are built in code (no
//! hand-edited files) so they stay readable and reviewable.
//!
//! Fixture set (vector-geometry.md VG7): straight · cubic · mixed open/closed ·
//! self-intersecting · coincident points · tiny (<1e-3) · huge (1e5) · negative coordinates ·
//! many subpaths · degenerate (zero-length segment).

use gip_coords::{Mat2, Vec2};
use gip_document::path::{FillRule, HandleMode, Path, PointType, Subpath, Vertex};
use gip_geometry::ops::{delete_vertex, insert_vertex, reverse_subpath, set_closed, transform};
use gip_geometry::{contains_point, distance_to_path, flatten_subpath, segments, tight_bounds};

const TOL: f64 = 0.01;

fn corners(points: &[[f64; 2]], closed: bool) -> Subpath {
    Subpath {
        vertices: points.iter().map(|p| Vertex::corner(*p)).collect(),
        closed,
    }
}

fn path_of(subpaths: Vec<Subpath>) -> Path {
    Path {
        subpaths,
        fill_rule: FillRule::NonZero,
    }
}

fn curved_arc() -> Subpath {
    let mut v = [Vertex::corner([0.0, 0.0]), Vertex::corner([100.0, 10.0])];
    v[0].handle_out = [30.0, 80.0];
    v[1].handle_in = [70.0, -40.0];
    v[0].point_type = PointType::Smooth;
    v[0].handle_mode = HandleMode::Mirrored;
    Subpath {
        vertices: v.to_vec(),
        closed: false,
    }
}

/// `(name, path)` for each fixture, in VG7 order.
fn fixtures() -> Vec<(&'static str, Path)> {
    vec![
        (
            "straight",
            path_of(vec![corners(
                &[[0.0, 0.0], [50.0, 0.0], [50.0, 30.0]],
                false,
            )]),
        ),
        ("cubic", path_of(vec![curved_arc()])),
        (
            "mixed open+closed",
            path_of(vec![
                corners(&[[0.0, 0.0], [40.0, 0.0], [40.0, 40.0]], false),
                corners(
                    &[[60.0, 0.0], [100.0, 0.0], [100.0, 40.0], [60.0, 40.0]],
                    true,
                ),
            ]),
        ),
        (
            "self-intersecting bowtie",
            path_of(vec![corners(
                &[[0.0, 0.0], [60.0, 60.0], [60.0, 0.0], [0.0, 60.0]],
                true,
            )]),
        ),
        (
            "coincident points",
            path_of(vec![corners(
                &[[10.0, 10.0], [10.0, 10.0], [30.0, 10.0], [30.0, 10.0]],
                true,
            )]),
        ),
        (
            "tiny (<1e-3)",
            path_of(vec![corners(
                &[[0.0, 0.0], [5e-4, 0.0], [5e-4, 4e-4]],
                true,
            )]),
        ),
        (
            "huge (1e5)",
            path_of(vec![corners(
                &[[0.0, 0.0], [1e5, 0.0], [1e5, 1e5], [0.0, 1e5]],
                true,
            )]),
        ),
        (
            "negative coordinates",
            path_of(vec![corners(
                &[[-120.0, -80.0], [-20.0, -80.0], [-20.0, -10.0]],
                true,
            )]),
        ),
        (
            "many subpaths",
            path_of(
                (0..25)
                    .map(|i| {
                        let x = (i % 5) as f64 * 20.0;
                        let y = (i / 5) as f64 * 20.0;
                        corners(&[[x, y], [x + 10.0, y], [x + 10.0, y + 10.0]], true)
                    })
                    .collect(),
            ),
        ),
        (
            "degenerate zero-length segment",
            path_of(vec![corners(&[[0.0, 0.0], [0.0, 0.0], [20.0, 0.0]], false)]),
        ),
    ]
}

/// Sampled bounds of the true curves (the oracle the tight bounds must agree with).
fn sampled_bounds(path: &Path) -> (Vec2, Vec2) {
    let mut min = Vec2::new(f64::INFINITY, f64::INFINITY);
    let mut max = Vec2::new(f64::NEG_INFINITY, f64::NEG_INFINITY);
    for s in segments(path) {
        for i in 0..=20_000 {
            let p = s.cubic.at(i as f64 / 20_000.0);
            min = min.min_components(p);
            max = max.max_components(p);
        }
    }
    for sub in &path.subpaths {
        for v in &sub.vertices {
            let p = Vec2::new(v.anchor[0], v.anchor[1]);
            min = min.min_components(p);
            max = max.max_components(p);
        }
    }
    (min, max)
}

#[test]
fn tight_bounds_agree_with_sampling_on_every_fixture() {
    for (name, path) in fixtures() {
        let b = tight_bounds(&path).expect(name);
        let (smin, smax) = sampled_bounds(&path);
        let scale = 1.0
            + smax
                .x
                .abs()
                .max(smax.y.abs())
                .max(smin.x.abs())
                .max(smin.y.abs());
        assert!(
            b.min.approx_eq(smin, 1e-7 * scale),
            "{name}: min {:?} vs {smin:?}",
            b.min
        );
        assert!(
            b.max.approx_eq(smax, 1e-7 * scale),
            "{name}: max {:?} vs {smax:?}",
            b.max
        );
    }
}

#[test]
fn flattening_stays_within_tolerance_on_every_fixture() {
    for (name, path) in fixtures() {
        for sub in &path.subpaths {
            let poly = flatten_subpath(sub, TOL);
            assert!(!poly.is_empty(), "{name}");
        }
        for s in segments(&path) {
            let mut pts = vec![s.cubic.p0];
            s.cubic.flatten_into(TOL, &mut pts);
            for i in 0..=400 {
                let p = s.cubic.at(i as f64 / 400.0);
                let d = pts
                    .windows(2)
                    .map(|w| seg_dist(p, w[0], w[1]))
                    .fold(f64::INFINITY, f64::min);
                assert!(d <= TOL + 1e-9, "{name}: flatten deviates {d}");
            }
        }
    }
}

#[test]
fn flattening_and_queries_are_deterministic() {
    for (name, path) in fixtures() {
        let a: Vec<Vec<Vec2>> = path
            .subpaths
            .iter()
            .map(|s| flatten_subpath(s, TOL))
            .collect();
        let b: Vec<Vec<Vec2>> = path
            .subpaths
            .iter()
            .map(|s| flatten_subpath(s, TOL))
            .collect();
        assert_eq!(a, b, "{name}: flatten must be bit-identical across runs");
        let q = Vec2::new(12.5, 7.25);
        assert_eq!(
            distance_to_path(&path, q),
            distance_to_path(&path, q),
            "{name}"
        );
    }
}

#[test]
fn fill_is_consistent_and_boundary_points_are_stable_at_tolerance() {
    // A point deep inside the closed square is inside; far outside is outside.
    let square = path_of(vec![corners(
        &[[0.0, 0.0], [100.0, 0.0], [100.0, 100.0], [0.0, 100.0]],
        true,
    )]);
    assert!(contains_point(&square, Vec2::new(50.0, 50.0), TOL));
    assert!(!contains_point(&square, Vec2::new(150.0, 50.0), TOL));
    // Display-LOD independence (VG3): hit results at a fixed tolerance do not change when the
    // query tolerance is refined, for points clearly away from the boundary.
    for (name, path) in fixtures() {
        for p in [
            Vec2::new(3.0, 2.0),
            Vec2::new(-50.0, 40.0),
            Vec2::new(500.0, -500.0),
        ] {
            let coarse = contains_point(&path, p, 1.0);
            let fine = contains_point(&path, p, 1e-4);
            let d = distance_to_path(&path, p).map_or(f64::INFINITY, |n| n.distance);
            if d > 1.0 {
                assert_eq!(
                    coarse, fine,
                    "{name}: tolerance changed a far-from-boundary result"
                );
            }
        }
    }
}

#[test]
fn every_editing_operation_keeps_the_path_valid_and_finite() {
    for (name, path) in fixtures() {
        let mut current = path.clone();
        if let Ok(p) = reverse_subpath(&current, 0) {
            current = p;
        }
        if let Ok(p) = set_closed(&current, 0, true) {
            current = p;
        }
        if let Ok(p) = insert_vertex(&current, 0, 0, 0.37) {
            current = p;
        }
        if let Ok(p) = delete_vertex(&current, 0, 0) {
            current = p;
        }
        let (min, max) = sampled_bounds(&current);
        assert!(
            min.is_finite() && max.is_finite() || current.subpaths.is_empty(),
            "{name}"
        );
        for seg in segments(&current) {
            assert!(
                seg.cubic.p0.is_finite() && seg.cubic.p3.is_finite(),
                "{name}"
            );
        }
    }
}

#[test]
fn thousand_cycle_transform_stability_on_every_fixture() {
    let m = Mat2::from_trs(gip_coords::Trs::new(
        3.0, -7.0, 1.25, 0.8, 41.0, false, true,
    ));
    let inv = m.invert().unwrap();
    for (name, path) in fixtures() {
        let mut current = path.clone();
        for _ in 0..1000 {
            current = transform(&current, m).unwrap();
            current = transform(&current, inv).unwrap();
        }
        for (a, b) in path.subpaths.iter().zip(&current.subpaths) {
            for (va, vb) in a.vertices.iter().zip(&b.vertices) {
                for (x, y) in [
                    (va.anchor, vb.anchor),
                    (va.handle_in, vb.handle_in),
                    (va.handle_out, vb.handle_out),
                ] {
                    let d = Vec2::new(x[0] - y[0], x[1] - y[1]).length_sq().sqrt();
                    assert!(
                        d <= 1e-6 * (1.0 + x[0].abs().max(x[1].abs())),
                        "{name}: drift {d}"
                    );
                }
            }
        }
    }
}

#[test]
fn reversing_twice_is_the_identity() {
    for (name, path) in fixtures() {
        let twice = reverse_subpath(&reverse_subpath(&path, 0).unwrap(), 0).unwrap();
        assert_eq!(twice, path, "{name}");
    }
}

fn seg_dist(p: Vec2, a: Vec2, b: Vec2) -> f64 {
    let ab = b - a;
    let len = ab.length_sq();
    let t = if len == 0.0 {
        0.0
    } else {
        ((p - a).dot(ab) / len).clamp(0.0, 1.0)
    };
    p.distance_sq(a + ab * t).sqrt()
}

#[test]
fn shared_bound_search_matches_brute_force_on_every_fixture() {
    // distance_to_path shares one running bound across segments; its answer must equal the
    // minimum over independent per-segment sampling on every golden fixture.
    for (name, path) in fixtures() {
        for p in [
            Vec2::new(7.0, -3.0),
            Vec2::new(33.0, 21.5),
            Vec2::new(-90.0, 60.0),
            Vec2::new(120.0, 120.0),
        ] {
            let got = distance_to_path(&path, p).map(|n| n.distance);
            let mut brute = f64::INFINITY;
            for s in segments(&path) {
                for i in 0..=20_000 {
                    brute = brute.min(s.cubic.at(i as f64 / 20_000.0).distance_sq(p).sqrt());
                }
            }
            if let Some(d) = got {
                assert!(
                    d <= brute + 1e-9,
                    "{name}: search {d} worse than brute {brute}"
                );
                assert!(
                    brute - d <= 1e-3 * (1.0 + brute),
                    "{name}: search {d} missed brute {brute}"
                );
            } else {
                assert!(brute.is_infinite(), "{name}: no nearest but segments exist");
            }
        }
    }
}
