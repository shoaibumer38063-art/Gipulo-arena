//! Path-level segment index (ADR-007): a static bounding-volume hierarchy over one path's
//! segments. Built from tight segment bounds; queried by branch-and-bound nearest search.
//! Brute force (`hit::distance_to_path`) is the reference oracle; tests pin the equivalence.

use gip_coords::Vec2;
use gip_document::path::Path;

use crate::cubic::Cubic;
use crate::hit::Nearest;
use crate::segments::{segment_count, segment_cubic};

/// Maximum segments per leaf (ADR-007 #1). Measured: 2 gives 0.60 ms/query vs 1.08 ms at 8 (dense 1000-seg fixture).
const LEAF_SIZE: usize = 2;

#[derive(Debug, Clone, Copy)]
struct Item {
    subpath: usize,
    segment: usize,
    cubic: Cubic,
    min: Vec2,
    max: Vec2,
}

#[derive(Debug, Clone, Copy)]
enum Kind {
    /// Leaf: a range of `items`.
    Leaf { start: usize, len: usize },
    /// Inner node: indices of the two children in `nodes`.
    Inner { left: usize, right: usize },
}

#[derive(Debug, Clone, Copy)]
struct BvhNode {
    min: Vec2,
    max: Vec2,
    kind: Kind,
}

/// Static segment index for one path. Rebuild it when the path changes; never per frame.
#[derive(Debug, Clone, Default)]
pub struct SegmentIndex {
    items: Vec<Item>,
    nodes: Vec<BvhNode>,
    root: Option<usize>,
}

impl SegmentIndex {
    /// Builds the index for `path`. O(n log n) in the segment count.
    #[must_use]
    pub fn build(path: &Path) -> Self {
        let mut items = Vec::new();
        for (subpath, sub) in path.subpaths.iter().enumerate() {
            for segment in 0..segment_count(sub) {
                let cubic = segment_cubic(sub, segment);
                let (min, max) = cubic.tight_bounds();
                items.push(Item {
                    subpath,
                    segment,
                    cubic,
                    min,
                    max,
                });
            }
        }
        if items.is_empty() {
            return Self::default();
        }
        let mut order: Vec<usize> = (0..items.len()).collect();
        let mut nodes = Vec::new();
        let root = build_rec(&items, &mut order, 0, &mut nodes);
        let sorted = order.iter().map(|&i| items[i]).collect();
        Self {
            items: sorted,
            nodes,
            root: Some(root),
        }
    }

    /// Number of indexed segments.
    #[must_use]
    pub fn len(&self) -> usize {
        self.items.len()
    }

    /// Whether the index holds no segments.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    /// Nearest point on any indexed segment; `None` when there are no segments. Result
    /// semantics are identical to `hit::distance_to_path` (distance exactly, segment on ties).
    #[must_use]
    pub fn nearest(&self, p: Vec2) -> Option<Nearest> {
        let mut best = None;
        if let Some(root) = self.root {
            self.visit(root, p, &mut best);
        }
        best
    }

    fn visit(&self, node: usize, p: Vec2, best: &mut Option<Nearest>) {
        let n = self.nodes[node];
        if box_dist_sq(n.min, n.max, p) >= bound_sq(*best) {
            return;
        }
        match n.kind {
            Kind::Leaf { start, len } => {
                for item in &self.items[start..start + len] {
                    let bound = bound_sq(*best);
                    if box_dist_sq(item.min, item.max, p) >= bound {
                        continue;
                    }
                    if let Some((t, point, dist_sq)) = item.cubic.nearest_within(p, bound) {
                        *best = Some(Nearest {
                            subpath: item.subpath,
                            segment: item.segment,
                            t,
                            point,
                            distance: dist_sq.sqrt(),
                        });
                    }
                }
            }
            Kind::Inner { left, right } => {
                let dl = box_dist_sq(self.nodes[left].min, self.nodes[left].max, p);
                let dr = box_dist_sq(self.nodes[right].min, self.nodes[right].max, p);
                let (near, far) = if dl <= dr {
                    (left, right)
                } else {
                    (right, left)
                };
                self.visit(near, p, best);
                self.visit(far, p, best);
            }
        }
    }
}

fn bound_sq(best: Option<Nearest>) -> f64 {
    best.map_or(f64::INFINITY, |b| b.distance * b.distance)
}

/// Squared distance from `p` to the box `[min, max]` (zero inside).
fn box_dist_sq(min: Vec2, max: Vec2, p: Vec2) -> f64 {
    let dx = (min.x - p.x).max(0.0).max(p.x - max.x);
    let dy = (min.y - p.y).max(0.0).max(p.y - max.y);
    dx * dx + dy * dy
}

/// Builds the subtree over `order` (indices into `items`); `offset` is the position of
/// `order[0]` in the final sorted item list. Returns the node index.
fn build_rec(
    items: &[Item],
    order: &mut [usize],
    offset: usize,
    nodes: &mut Vec<BvhNode>,
) -> usize {
    let mut min = Vec2::new(f64::INFINITY, f64::INFINITY);
    let mut max = Vec2::new(f64::NEG_INFINITY, f64::NEG_INFINITY);
    let mut cmin = min;
    let mut cmax = max;
    for &i in order.iter() {
        let item = &items[i];
        min = Vec2::new(min.x.min(item.min.x), min.y.min(item.min.y));
        max = Vec2::new(max.x.max(item.max.x), max.y.max(item.max.y));
        let c = Vec2::new(
            (item.min.x + item.max.x) * 0.5,
            (item.min.y + item.max.y) * 0.5,
        );
        cmin = Vec2::new(cmin.x.min(c.x), cmin.y.min(c.y));
        cmax = Vec2::new(cmax.x.max(c.x), cmax.y.max(c.y));
    }
    if order.len() <= LEAF_SIZE {
        nodes.push(BvhNode {
            min,
            max,
            kind: Kind::Leaf {
                start: offset,
                len: order.len(),
            },
        });
        return nodes.len() - 1;
    }
    let use_x = cmax.x - cmin.x >= cmax.y - cmin.y;
    let key = |i: &usize| {
        let item = &items[*i];
        if use_x {
            (item.min.x + item.max.x) * 0.5
        } else {
            (item.min.y + item.max.y) * 0.5
        }
    };
    let mid = order.len() / 2;
    order.select_nth_unstable_by(mid, |a, b| key(a).total_cmp(&key(b)));
    let (lo, hi) = order.split_at_mut(mid);
    let left = build_rec(items, lo, offset, nodes);
    let right = build_rec(items, hi, offset + mid, nodes);
    nodes.push(BvhNode {
        min,
        max,
        kind: Kind::Inner { left, right },
    });
    nodes.len() - 1
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::hit::distance_to_path;
    use crate::segments::pt;
    use gip_document::path::{Subpath, Vertex};

    fn wave(n: usize, phase: f64) -> Subpath {
        let vertices = (0..n)
            .map(|i| {
                let x = i as f64 * 10.0;
                let y = ((i as f64 + phase) * 0.9).sin() * 40.0;
                Vertex::corner([x, y])
            })
            .collect();
        Subpath {
            vertices,
            closed: false,
        }
    }

    #[test]
    fn empty_and_anchor_only_paths_have_no_candidates() {
        assert!(
            SegmentIndex::build(&Path::default())
                .nearest(pt([0.0, 0.0]))
                .is_none()
        );
        let single = Path {
            subpaths: vec![Subpath {
                vertices: vec![Vertex::corner([1.0, 1.0])],
                closed: false,
            }],
            ..Path::default()
        };
        let index = SegmentIndex::build(&single);
        assert!(index.is_empty());
        assert!(index.nearest(pt([0.0, 0.0])).is_none());
    }

    #[test]
    fn matches_brute_force_on_a_deterministic_sweep() {
        // Deterministic queries over several shapes: exact distance agreement with the oracle.
        let shapes = [
            Path {
                subpaths: vec![wave(3, 0.0)],
                ..Path::default()
            },
            Path {
                subpaths: vec![wave(40, 0.3), wave(25, 1.1)],
                ..Path::default()
            },
            Path {
                subpaths: vec![wave(200, 2.0)],
                ..Path::default()
            },
        ];
        for path in &shapes {
            let index = SegmentIndex::build(path);
            for qx in (-20..=220).step_by(13) {
                for qy in [-80.0, -10.0, 17.5, 90.0] {
                    let q = pt([f64::from(qx) * 1.3, qy]);
                    let fast = index.nearest(q).map(|n| n.distance);
                    let slow = distance_to_path(path, q).map(|n| n.distance);
                    match (fast, slow) {
                        (Some(a), Some(b)) => {
                            assert!((a - b).abs() <= 1e-9, "query {q:?}: index {a} vs brute {b}")
                        }
                        (None, None) => {}
                        other => panic!("query {q:?}: presence mismatch {other:?}"),
                    }
                }
            }
        }
    }

    #[test]
    fn reports_the_segment_that_owns_the_distance() {
        let path = Path {
            subpaths: vec![wave(30, 0.0)],
            ..Path::default()
        };
        let index = SegmentIndex::build(&path);
        let q = pt([105.0, -30.0]);
        let hit = index.nearest(q).expect("segments exist");
        let oracle = distance_to_path(&path, q).expect("segments exist");
        assert!((hit.distance - oracle.distance).abs() <= 1e-9);
        assert_eq!(hit.subpath, 0);
        assert!(hit.segment < 29);
    }
}
