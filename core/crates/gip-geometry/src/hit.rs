//! Hit geometry (VG5, CAN-GEOM-4): point-in-fill and distance-to-path. Hit results are
//! computed from the path itself at a caller-supplied FIXED tolerance — never from display
//! LOD — so a coarsened render can never change what is clickable (VG3).

use gip_coords::Vec2;
use gip_document::path::{FillRule, Path};

use crate::cubic::Cubic;
use crate::segments::{flatten_subpath, segment_count, segment_cubic};

/// Nearest point on a path, with its addressing (subpath, segment, parameter t).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Nearest {
    /// Owning subpath index.
    pub subpath: usize,
    /// Segment index within the subpath.
    pub segment: usize,
    /// Parameter on that segment's cubic, in `[0, 1]`.
    pub t: f64,
    /// The nearest point.
    pub point: Vec2,
    /// Distance from the query point to `point`.
    pub distance: f64,
}

/// Point-in-fill. Open subpaths fill as if closed (VG5). `tol` controls the flattening
/// precision of the boundary (a point within `tol` of the boundary may classify either way;
/// use [`distance_to_path`] for boundary-tolerant picking).
#[must_use]
pub fn contains_point(path: &Path, p: Vec2, tol: f64) -> bool {
    let mut winding = 0i32;
    let mut crossings = 0u32;
    for sub in &path.subpaths {
        let ring = flatten_subpath(sub, tol);
        if ring.len() < 3 {
            continue;
        }
        for i in 0..ring.len() {
            let a = ring[i];
            let b = ring[(i + 1) % ring.len()];
            if a.y <= p.y {
                if b.y > p.y && (b - a).cross(p - a) > 0.0 {
                    winding += 1;
                    crossings += 1;
                }
            } else if b.y <= p.y && (b - a).cross(p - a) < 0.0 {
                winding -= 1;
                crossings += 1;
            }
        }
    }
    match path.fill_rule {
        FillRule::NonZero => winding != 0,
        FillRule::EvenOdd => crossings % 2 == 1,
    }
}

/// Nearest point on any segment of the path. `None` when the path has no segments
/// (anchor-only paths have no curve to be near; their anchors are the points).
///
/// One running best distance is shared across segments: each segment is first rejected by its
/// control-hull box distance, then searched only if it can beat the current best.
#[must_use]
pub fn distance_to_path(path: &Path, p: Vec2) -> Option<Nearest> {
    let mut best: Option<Nearest> = None;
    for (subpath, sub) in path.subpaths.iter().enumerate() {
        for segment in 0..segment_count(sub) {
            let cubic: Cubic = segment_cubic(sub, segment);
            let bound = best.map_or(f64::INFINITY, |b| b.distance * b.distance);
            if let Some((t, point, dist_sq)) = cubic.nearest_within(p, bound) {
                let distance = dist_sq.sqrt();
                if best.is_none_or(|b| distance < b.distance) {
                    best = Some(Nearest {
                        subpath,
                        segment,
                        t,
                        point,
                        distance,
                    });
                }
            }
        }
    }
    best
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::segments::pt;
    use gip_document::path::{Subpath, Vertex};

    fn polygon(points: &[[f64; 2]], closed: bool) -> Subpath {
        Subpath {
            vertices: points.iter().map(|p| Vertex::corner(*p)).collect(),
            closed,
        }
    }

    fn square(x: f64, y: f64, s: f64) -> Subpath {
        polygon(&[[x, y], [x + s, y], [x + s, y + s], [x, y + s]], true)
    }

    #[test]
    fn nonzero_and_evenodd_differ_for_nested_same_direction_rings() {
        // Two concentric squares, both wound the same way: nonzero fills the hole, evenodd not.
        let mut path = Path {
            subpaths: vec![square(0.0, 0.0, 100.0), square(25.0, 25.0, 50.0)],
            fill_rule: FillRule::NonZero,
        };
        assert!(contains_point(&path, pt([10.0, 10.0]), 0.01));
        assert!(
            contains_point(&path, pt([50.0, 50.0]), 0.01),
            "nonzero fills the inner ring"
        );
        path.fill_rule = FillRule::EvenOdd;
        assert!(contains_point(&path, pt([10.0, 10.0]), 0.01));
        assert!(
            !contains_point(&path, pt([50.0, 50.0]), 0.01),
            "evenodd cuts the hole"
        );
    }

    #[test]
    fn reversed_inner_ring_cuts_a_hole_under_nonzero() {
        let mut inner = square(25.0, 25.0, 50.0);
        inner.vertices.reverse();
        let path = Path {
            subpaths: vec![square(0.0, 0.0, 100.0), inner],
            fill_rule: FillRule::NonZero,
        };
        assert!(!contains_point(&path, pt([50.0, 50.0]), 0.01));
        assert!(contains_point(&path, pt([5.0, 5.0]), 0.01));
    }

    #[test]
    fn open_subpaths_fill_as_if_closed() {
        let open_triangle = polygon(&[[0.0, 0.0], [10.0, 0.0], [0.0, 10.0]], false);
        let path = Path {
            subpaths: vec![open_triangle],
            ..Path::default()
        };
        assert!(contains_point(&path, pt([2.0, 2.0]), 0.01));
        assert!(!contains_point(&path, pt([8.0, 8.0]), 0.01));
    }

    #[test]
    fn distance_is_exact_for_a_straight_edge_and_addresses_the_segment() {
        let path = Path {
            subpaths: vec![square(0.0, 0.0, 10.0)],
            ..Path::default()
        };
        let n = distance_to_path(&path, pt([5.0, -3.0])).unwrap();
        assert_eq!((n.subpath, n.segment), (0, 0));
        assert!((n.distance - 3.0).abs() <= 1e-12);
        assert!(n.point.approx_eq(Vec2::new(5.0, 0.0), 1e-12));
    }

    #[test]
    fn anchor_only_paths_have_no_nearest_segment() {
        let path = Path {
            subpaths: vec![polygon(&[[0.0, 0.0]], false)],
            ..Path::default()
        };
        assert!(distance_to_path(&path, pt([1.0, 1.0])).is_none());
    }
}
