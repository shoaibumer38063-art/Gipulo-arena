//! Path-level views over the document path data (ADR-006): segment enumeration, tight
//! bounds, and flattening into polylines. Pure functions; nothing is stored.

use gip_coords::Vec2;
use gip_document::path::{Path, Subpath};

use crate::BoundsRect;
use crate::cubic::Cubic;

/// One implied segment of a path, addressed by `(subpath, index)`.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Segment {
    /// Index of the owning subpath.
    pub subpath: usize,
    /// Index of the segment within the subpath (0-based, drawing order).
    pub index: usize,
    /// The cubic geometry of the segment.
    pub cubic: Cubic,
    /// True when the segment is straight by the data definition (ADR-006 #6).
    pub straight: bool,
}

/// Number of segments a subpath implies: `n − 1` open, `n` closed, zero below two vertices.
#[must_use]
pub fn segment_count(sub: &Subpath) -> usize {
    let n = sub.vertices.len();
    match (n, sub.closed) {
        (0 | 1, _) => 0,
        (n, true) => n,
        (n, false) => n - 1,
    }
}

/// The cubic of segment `index` in a subpath (`index < segment_count`).
#[must_use]
pub fn segment_cubic(sub: &Subpath, index: usize) -> Cubic {
    let n = sub.vertices.len();
    let a = &sub.vertices[index];
    let b = &sub.vertices[(index + 1) % n];
    Cubic::new(
        pt(a.anchor),
        pt(a.handle_out),
        pt(b.handle_in),
        pt(b.anchor),
    )
}

/// Every segment of a path, in subpath then segment order.
#[must_use]
pub fn segments(path: &Path) -> Vec<Segment> {
    let mut out = Vec::new();
    for (subpath, sub) in path.subpaths.iter().enumerate() {
        for index in 0..segment_count(sub) {
            let cubic = segment_cubic(sub, index);
            out.push(Segment {
                subpath,
                index,
                cubic,
                straight: cubic.is_straight(),
            });
        }
    }
    out
}

/// Tight bounds of the geometry (anchors + exact curve extrema; handles do not count).
/// `None` when the path has no vertices.
#[must_use]
pub fn tight_bounds(path: &Path) -> Option<BoundsRect> {
    let mut acc: Option<BoundsRect> = None;
    for sub in &path.subpaths {
        for v in &sub.vertices {
            acc = Some(extend(acc, pt(v.anchor)));
        }
    }
    for seg in segments(path) {
        let (min, max) = seg.cubic.tight_bounds();
        acc = Some(match acc {
            Some(b) => b.union(BoundsRect::new(min, max)),
            None => BoundsRect::new(min, max),
        });
    }
    acc
}

fn extend(acc: Option<BoundsRect>, p: Vec2) -> BoundsRect {
    match acc {
        Some(b) => b.union(BoundsRect::new(p, p)),
        None => BoundsRect::new(p, p),
    }
}

/// Flattened outline of one subpath at `tol` (world units). Starts at the first anchor and
/// ends at the last segment's end anchor (for closed subpaths, the closing segment's end is the
/// first anchor again). Subpaths with no segments yield their anchors as a point list.
#[must_use]
pub fn flatten_subpath(sub: &Subpath, tol: f64) -> Vec<Vec2> {
    let mut out = Vec::new();
    let Some(first) = sub.vertices.first() else {
        return out;
    };
    out.push(pt(first.anchor));
    let count = segment_count(sub);
    if count == 0 {
        for v in sub.vertices.iter().skip(1) {
            out.push(pt(v.anchor));
        }
        return out;
    }
    for index in 0..count {
        segment_cubic(sub, index).flatten_into(tol, &mut out);
    }
    out
}

/// Converts document `[f64; 2]` points to geometry vectors (the one boundary conversion).
#[must_use]
pub fn pt(p: [f64; 2]) -> Vec2 {
    Vec2::new(p[0], p[1])
}

#[cfg(test)]
mod tests {
    use super::*;
    use gip_document::path::Vertex;

    fn sub(points: &[[f64; 2]], closed: bool) -> Subpath {
        Subpath {
            vertices: points.iter().map(|p| Vertex::corner(*p)).collect(),
            closed,
        }
    }

    #[test]
    fn segment_counts_follow_the_adr_rule() {
        assert_eq!(segment_count(&sub(&[], false)), 0);
        assert_eq!(segment_count(&sub(&[[0.0, 0.0]], true)), 0);
        assert_eq!(segment_count(&sub(&[[0.0, 0.0], [1.0, 0.0]], false)), 1);
        assert_eq!(segment_count(&sub(&[[0.0, 0.0], [1.0, 0.0]], true)), 2);
    }

    #[test]
    fn closing_segment_connects_last_back_to_first() {
        let s = sub(&[[0.0, 0.0], [10.0, 0.0], [10.0, 10.0]], true);
        let closing = segment_cubic(&s, 2);
        assert_eq!(closing.p0, Vec2::new(10.0, 10.0));
        assert_eq!(closing.p3, Vec2::new(0.0, 0.0));
    }

    #[test]
    fn straight_flag_follows_handle_data() {
        let mut s = sub(&[[0.0, 0.0], [10.0, 0.0]], false);
        assert!(
            segments(&Path {
                subpaths: vec![s.clone()],
                ..Path::default()
            })[0]
                .straight
        );
        s.vertices[0].handle_out = [3.0, 4.0];
        assert!(
            !segments(&Path {
                subpaths: vec![s],
                ..Path::default()
            })[0]
                .straight
        );
    }

    #[test]
    fn tight_bounds_of_empty_and_anchor_only_paths() {
        assert!(tight_bounds(&Path::default()).is_none());
        let p = Path {
            subpaths: vec![sub(&[[2.0, 3.0], [-1.0, 7.0]], false)],
            ..Path::default()
        };
        let b = tight_bounds(&p).unwrap();
        assert_eq!((b.min, b.max), (Vec2::new(-1.0, 3.0), Vec2::new(2.0, 7.0)));
    }
}
