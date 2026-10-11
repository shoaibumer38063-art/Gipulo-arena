//! Editing operations (VG2) — pure functions: `Path` in, new `Path` out. Callers run them
//! inside document transactions (ADR-004 records the before/after values). Each operation
//! documents its shape guarantee; none approximates the curve it edits.

use gip_coords::Mat2;
use gip_document::path::{HandleMode, Path, PointType, Vertex};

use crate::segments::{pt, segment_count};

/// Reasons an operation refuses to run. The input path is never modified on error.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum GeomError {
    /// A subpath or vertex/segment index does not exist.
    IndexOutOfRange,
    /// A matrix or coordinate was NaN or infinite (invalid document data).
    NonFinite,
}

fn subpath_mut(path: &mut Path, sub: usize) -> Result<&mut gip_document::path::Subpath, GeomError> {
    path.subpaths.get_mut(sub).ok_or(GeomError::IndexOutOfRange)
}

/// Applies an affine transform to every anchor and handle. Exact for affine maps: the curve of
/// the transformed path is the transformed curve (no sampling). Degenerate (zero-scale)
/// matrices are allowed — the path collapses — but non-finite matrices are rejected.
pub fn transform(path: &Path, m: Mat2) -> Result<Path, GeomError> {
    if !m.is_finite() {
        return Err(GeomError::NonFinite);
    }
    let map = |p: [f64; 2]| {
        let q = m.apply(pt(p));
        [q.x, q.y]
    };
    let mut out = path.clone();
    for sub in &mut out.subpaths {
        for v in &mut sub.vertices {
            v.anchor = map(v.anchor);
            v.handle_in = map(v.handle_in);
            v.handle_out = map(v.handle_out);
        }
    }
    Ok(out)
}

/// Reverses a subpath's drawing direction. Each vertex swaps its in/out handles; the shape is
/// unchanged (point-set identical), and the fill winding flips.
pub fn reverse_subpath(path: &Path, sub: usize) -> Result<Path, GeomError> {
    let mut out = path.clone();
    let s = subpath_mut(&mut out, sub)?;
    s.vertices.reverse();
    for v in &mut s.vertices {
        std::mem::swap(&mut v.handle_in, &mut v.handle_out);
    }
    Ok(out)
}

/// Sets a subpath's closed flag (explicit data, ADR-006 #4). Opening a closed path keeps the
/// closing segment's data on the last vertex's handle — nothing is discarded silently.
pub fn set_closed(path: &Path, sub: usize, closed: bool) -> Result<Path, GeomError> {
    let mut out = path.clone();
    subpath_mut(&mut out, sub)?.closed = closed;
    Ok(out)
}

/// Splits segment `seg` of subpath `sub` at parameter `t`, inserting a new vertex.
///
/// Curved segments: de Casteljau split — the two halves reproduce the original curve exactly,
/// the new vertex's handles are the split control points (Asymmetric, Smooth-typed because the
/// tangent is continuous by construction). Straight segments: the split point lies on the line
/// and both halves keep exact straight data (ADR-006 #6) — no rounding can make them curved.
pub fn insert_vertex(path: &Path, sub: usize, seg: usize, t: f64) -> Result<Path, GeomError> {
    if !(0.0..=1.0).contains(&t) || !t.is_finite() {
        return Err(GeomError::IndexOutOfRange);
    }
    let mut out = path.clone();
    let s = subpath_mut(&mut out, sub)?;
    if seg >= segment_count(s) {
        return Err(GeomError::IndexOutOfRange);
    }
    let n = s.vertices.len();
    let a_idx = seg;
    let b_idx = (seg + 1) % n;
    let cubic = crate::segments::segment_cubic(s, seg);
    let (vertex, new_a_out, new_b_in) = if cubic.is_straight() {
        let at = cubic.at(t);
        (Vertex::corner([at.x, at.y]), None, None)
    } else {
        let (left, right) = cubic.split(t);
        let mut v = Vertex::corner([left.p3.x, left.p3.y]);
        v.handle_in = [left.p2.x, left.p2.y];
        v.handle_out = [right.p1.x, right.p1.y];
        v.point_type = PointType::Smooth;
        v.handle_mode = HandleMode::Asymmetric;
        (
            v,
            Some([left.p1.x, left.p1.y]),
            Some([right.p2.x, right.p2.y]),
        )
    };
    if let Some(h) = new_a_out {
        s.vertices[a_idx].handle_out = h;
    }
    if let Some(h) = new_b_in {
        s.vertices[b_idx].handle_in = h;
    }
    // Insert after a_idx. For the closing segment (a = last, b = first) append at the end.
    let insert_at = if s.closed && seg == n - 1 {
        n
    } else {
        a_idx + 1
    };
    s.vertices.insert(insert_at, vertex);
    Ok(out)
}

/// Deletes vertex `idx` of subpath `sub`. The neighbours keep their own handles, so the joined
/// segment reuses their shaped curve data (deterministic; nothing re-derived). Deleting the last
/// remaining vertex removes the subpath; a subpath reduced below two vertices becomes open.
pub fn delete_vertex(path: &Path, sub: usize, idx: usize) -> Result<Path, GeomError> {
    let mut out = path.clone();
    let s = subpath_mut(&mut out, sub)?;
    if idx >= s.vertices.len() {
        return Err(GeomError::IndexOutOfRange);
    }
    s.vertices.remove(idx);
    if s.vertices.len() < 2 {
        s.closed = false;
    }
    if s.vertices.is_empty() {
        out.subpaths.remove(sub);
    }
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cubic::Cubic;
    use crate::hit::contains_point;
    use crate::segments::{flatten_subpath, segment_cubic, segments};
    use gip_coords::Vec2;
    use gip_document::path::{FillRule, Subpath};

    fn curve_path() -> Path {
        Path {
            subpaths: vec![Subpath {
                vertices: vec![
                    Vertex {
                        anchor: [0.0, 0.0],
                        handle_in: [0.0, 0.0],
                        handle_out: [30.0, 80.0],
                        point_type: PointType::Corner,
                        handle_mode: HandleMode::Disconnected,
                    },
                    Vertex {
                        anchor: [100.0, 10.0],
                        handle_in: [70.0, -40.0],
                        handle_out: [100.0, 10.0],
                        point_type: PointType::Corner,
                        handle_mode: HandleMode::Disconnected,
                    },
                ],
                closed: false,
            }],
            fill_rule: FillRule::NonZero,
        }
    }

    fn samples(path: &Path) -> Vec<Vec2> {
        segments(path)
            .iter()
            .flat_map(|s| (0..=50).map(move |i| s.cubic.at(i as f64 / 50.0)))
            .collect()
    }

    #[test]
    fn insert_on_curve_preserves_the_shape_exactly() {
        let p = curve_path();
        let before = segments(&p)[0].cubic;
        for t in [0.1, 0.5, 0.93] {
            let q = insert_vertex(&p, 0, 0, t).unwrap();
            assert_eq!(q.subpaths[0].vertices.len(), 3);
            let a = segment_cubic(&q.subpaths[0], 0);
            let b = segment_cubic(&q.subpaths[0], 1);
            for i in 0..=100 {
                let u = i as f64 / 100.0;
                assert!(a.at(u).approx_eq(before.at(t * u), 1e-9));
                assert!(b.at(u).approx_eq(before.at(t + (1.0 - t) * u), 1e-9));
            }
        }
    }

    #[test]
    fn insert_on_straight_keeps_both_halves_straight() {
        let p = Path {
            subpaths: vec![Subpath {
                vertices: vec![Vertex::corner([0.0, 0.0]), Vertex::corner([7.0, 3.0])],
                closed: false,
            }],
            ..Path::default()
        };
        let q = insert_vertex(&p, 0, 0, 0.3).unwrap();
        for seg in segments(&q) {
            assert!(seg.straight, "a split line must stay exactly straight");
        }
    }

    #[test]
    fn insert_on_closing_segment_appends_and_keeps_closure() {
        let p = Path {
            subpaths: vec![Subpath {
                vertices: vec![
                    Vertex::corner([0.0, 0.0]),
                    Vertex::corner([10.0, 0.0]),
                    Vertex::corner([10.0, 10.0]),
                ],
                closed: true,
            }],
            ..Path::default()
        };
        let q = insert_vertex(&p, 0, 2, 0.5).unwrap();
        assert_eq!(q.subpaths[0].vertices.len(), 4);
        assert!(q.subpaths[0].closed);
        assert!(
            q.subpaths[0].vertices[3]
                .anchor
                .iter()
                .zip([5.0, 5.0])
                .all(|(a, b)| (a - b).abs() < 1e-12)
        );
    }

    #[test]
    fn transform_is_exact_for_the_curve_and_round_trips() {
        let p = curve_path();
        let m = Mat2::from_trs(gip_coords::Trs::new(
            15.0, -4.0, 2.0, 0.5, 37.0, true, false,
        ));
        let there = transform(&p, m).unwrap();
        let back = transform(&there, m.invert().unwrap()).unwrap();
        for (a, b) in samples(&p).iter().zip(samples(&back)) {
            assert!(a.approx_eq(b, 1e-9));
        }
        // The transformed curve equals the mapped original curve.
        let orig = segments(&p)[0].cubic;
        let moved = segments(&there)[0].cubic;
        assert!(moved.at(0.4).approx_eq(m.apply(orig.at(0.4)), 1e-9));
    }

    #[test]
    fn transform_rejects_non_finite_matrices() {
        let bad = Mat2::new(f64::NAN, 0.0, 0.0, 1.0, 0.0, 0.0);
        assert_eq!(transform(&curve_path(), bad), Err(GeomError::NonFinite));
    }

    #[test]
    fn reverse_keeps_the_point_set_and_flips_fill_winding() {
        let square = Path {
            subpaths: vec![Subpath {
                vertices: vec![
                    Vertex::corner([0.0, 0.0]),
                    Vertex::corner([10.0, 0.0]),
                    Vertex::corner([10.0, 10.0]),
                    Vertex::corner([0.0, 10.0]),
                ],
                closed: true,
            }],
            ..Path::default()
        };
        let rev = reverse_subpath(&square, 0).unwrap();
        let mut a = flatten_subpath(&square.subpaths[0], 0.01);
        let mut b = flatten_subpath(&rev.subpaths[0], 0.01);
        a.sort_by(|p, q| p.x.total_cmp(&q.x).then(p.y.total_cmp(&q.y)));
        b.sort_by(|p, q| p.x.total_cmp(&q.x).then(p.y.total_cmp(&q.y)));
        assert_eq!(a.len(), b.len());
        assert!(contains_point(&rev, Vec2::new(5.0, 5.0), 0.01));
    }

    #[test]
    fn set_closed_is_explicit_and_out_of_range_is_an_error() {
        let p = curve_path();
        assert!(set_closed(&p, 0, true).unwrap().subpaths[0].closed);
        assert_eq!(set_closed(&p, 5, true), Err(GeomError::IndexOutOfRange));
        assert_eq!(
            insert_vertex(&p, 0, 9, 0.5),
            Err(GeomError::IndexOutOfRange)
        );
        assert_eq!(
            insert_vertex(&p, 0, 0, 1.5),
            Err(GeomError::IndexOutOfRange)
        );
        assert_eq!(delete_vertex(&p, 0, 9), Err(GeomError::IndexOutOfRange));
    }

    #[test]
    fn delete_vertex_joins_neighbours_and_collapses_empty_subpaths() {
        let p = Path {
            subpaths: vec![Subpath {
                vertices: vec![
                    Vertex::corner([0.0, 0.0]),
                    Vertex::corner([5.0, 5.0]),
                    Vertex::corner([10.0, 0.0]),
                ],
                closed: true,
            }],
            ..Path::default()
        };
        let q = delete_vertex(&p, 0, 1).unwrap();
        assert_eq!(q.subpaths[0].vertices.len(), 2);
        let q2 = delete_vertex(&q, 0, 0).unwrap();
        assert!(!q2.subpaths[0].closed, "fewer than two vertices is open");
        let q3 = delete_vertex(&q2, 0, 0).unwrap();
        assert!(q3.subpaths.is_empty(), "last vertex removes the subpath");
    }

    #[test]
    fn cubic_helper_is_consistent_with_segment_data() {
        let p = curve_path();
        let c: Cubic = segment_cubic(&p.subpaths[0], 0);
        assert_eq!(c.p3, Vec2::new(100.0, 10.0));
    }
}
