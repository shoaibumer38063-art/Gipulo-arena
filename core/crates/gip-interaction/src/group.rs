//! Multi-object transforms (task 024). Pure math over the selected objects' transforms.
//!
//! - The **group box** is the axis-aligned bounds of the members' world boxes.
//! - **Rotate** turns every member about the group center (the pivot) by the pointer's angular
//!   change. Each member's own rotation changes by the same angle, and its center moves around the
//!   pivot. Shift snaps the change to 15° steps. Works for any members.
//! - **Resize** scales members inside the group box. It needs axis-aligned members (rotation 0,
//!   unflipped), so the scale is exact and no member is sheared.

use gip_coords::{Vec2, normalize_deg};
use gip_document::Transform;

use crate::machine::{Handle, ROTATE_STEP_DEG, center_of, keep_center, resize_box};

/// Tolerance for "rotation is zero" (a rotate-then-back gesture leaves float dust).
const AXIS_EPS_DEG: f64 = 1e-9;

/// Axis-aligned world bounds `(x, y, w, h)` of a set of transforms.
#[must_use]
pub fn group_box(ts: &[Transform]) -> (f64, f64, f64, f64) {
    let mut min = Vec2::new(f64::INFINITY, f64::INFINITY);
    let mut max = Vec2::new(f64::NEG_INFINITY, f64::NEG_INFINITY);
    for t in ts {
        let m = crate::machine::unit_mapping(t);
        for p in [(0.0, 0.0), (1.0, 0.0), (1.0, 1.0), (0.0, 1.0)] {
            let q = m.apply(Vec2::new(p.0, p.1));
            min = Vec2::new(min.x.min(q.x), min.y.min(q.y));
            max = Vec2::new(max.x.max(q.x), max.y.max(q.y));
        }
    }
    if ts.is_empty() {
        return (0.0, 0.0, 0.0, 0.0);
    }
    (min.x, min.y, max.x - min.x, max.y - min.y)
}

/// The group box as an unrotated, unflipped transform (the frame that handles are drawn on).
#[must_use]
pub fn group_frame(bounds: (f64, f64, f64, f64)) -> Transform {
    let (x, y, w, h) = bounds;
    Transform {
        x,
        y,
        w,
        h,
        rotation_deg: 0.0,
        flip_h: false,
        flip_v: false,
    }
}

/// A member can take part in a group resize: unrotated and unflipped.
#[must_use]
pub fn axis_aligned(t: &Transform) -> bool {
    t.rotation_deg.abs() < AXIS_EPS_DEG && !t.flip_h && !t.flip_v
}

/// Rotates every member about `pivot` by the pointer's angular change from `start_angle`
/// (radians). Shift snaps the change to 15° steps. Each result is normalized to −180…180°.
#[must_use]
pub fn rotate_group(
    starts: &[Transform],
    pivot: Vec2,
    start_angle: f64,
    pointer: Vec2,
    shift: bool,
) -> Vec<Transform> {
    let pointer_angle = (pointer.y - pivot.y).atan2(pointer.x - pivot.x);
    let mut delta = normalize_deg((pointer_angle - start_angle).to_degrees());
    if shift {
        delta = (delta / ROTATE_STEP_DEG).round() * ROTATE_STEP_DEG;
    }
    let (sin, cos) = delta.to_radians().sin_cos();
    starts
        .iter()
        .map(|start| {
            let c = center_of(start);
            let rel = c - pivot;
            let moved = pivot + Vec2::new(rel.x * cos - rel.y * sin, rel.x * sin + rel.y * cos);
            let mut t = *start;
            t.rotation_deg = normalize_deg(start.rotation_deg + delta);
            keep_center(t, moved)
        })
        .collect()
}

/// Resizes the group box by a handle drag (Slice 1 box math, including Shift and Alt) and scales
/// every member to the new box. Members keep their relative positions inside the box.
#[must_use]
pub fn resize_group(
    starts: &[Transform],
    start_box: (f64, f64, f64, f64),
    handle: Handle,
    delta: Vec2,
    mods: crate::machine::Modifiers,
) -> Vec<Transform> {
    let (bx, by, bw, bh) = start_box;
    let (gx, gy, gw, gh) = resize_box(start_box, handle, delta, mods);
    // A zero-size group has no scale to derive; members then keep their size.
    let sx = if bw > 0.0 { gw / bw } else { 1.0 };
    let sy = if bh > 0.0 { gh / bh } else { 1.0 };
    starts
        .iter()
        .map(|start| {
            let c = center_of(start);
            let new_center = Vec2::new(gx + (c.x - bx) * sx, gy + (c.y - by) * sy);
            let mut t = *start;
            t.w = start.w * sx;
            t.h = start.h * sy;
            keep_center(t, new_center)
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::machine::Modifiers;

    fn t(x: f64, y: f64, w: f64, h: f64) -> Transform {
        Transform {
            x,
            y,
            w,
            h,
            rotation_deg: 0.0,
            flip_h: false,
            flip_v: false,
        }
    }

    fn close(a: f64, b: f64) -> bool {
        (a - b).abs() < 1e-9
    }

    #[test]
    fn group_box_is_the_bounds_of_all_members() {
        let g = group_box(&[t(0.0, 0.0, 10.0, 10.0), t(20.0, 5.0, 10.0, 10.0)]);
        assert_eq!(g, (0.0, 0.0, 30.0, 15.0));
    }

    #[test]
    fn rotating_a_group_keeps_its_center_and_each_members_distance_to_it() {
        let starts = [t(0.0, 0.0, 10.0, 10.0), t(40.0, 0.0, 20.0, 10.0)];
        let (gx, gy, gw, gh) = group_box(&starts);
        let pivot = center_of(&group_frame((gx, gy, gw, gh)));
        // Pointer at 90° about the pivot from the start direction (0°), no snap.
        let start_angle = 0.0;
        let pointer = pivot + Vec2::new(0.0, 50.0);
        let out = rotate_group(&starts, pivot, start_angle, pointer, false);
        for (s, o) in starts.iter().zip(&out) {
            let d0 = (center_of(s) - pivot).length_sq().sqrt();
            let d1 = (center_of(o) - pivot).length_sq().sqrt();
            assert!(close(d0, d1), "distance to pivot is preserved");
            assert!(
                close(o.rotation_deg, 90.0),
                "each member turns by the same angle"
            );
        }
        let after = group_box(&out);
        assert!(close(after.0 + after.2 / 2.0, pivot.x));
        assert!(close(after.1 + after.3 / 2.0, pivot.y));
    }

    #[test]
    fn group_resize_scales_member_positions_and_sizes_inside_the_box() {
        let starts = [t(0.0, 0.0, 10.0, 10.0), t(20.0, 0.0, 10.0, 10.0)];
        let start_box = group_box(&starts); // (0, 0, 30, 10)
        let out = resize_group(
            &starts,
            start_box,
            Handle::E,
            Vec2::new(30.0, 0.0),
            Modifiers::default(),
        );
        // The box grows to 60 wide, so x scales by 2 about the left edge.
        assert!(close(out[0].w, 20.0));
        assert!(close(center_of(&out[1]).x, 50.0), "second center: 25 → 50");
        assert_eq!(group_box(&out), (0.0, 0.0, 60.0, 10.0));
    }

    #[test]
    fn axis_aligned_excludes_rotated_and_flipped_members() {
        assert!(axis_aligned(&t(0.0, 0.0, 1.0, 1.0)));
        let mut r = t(0.0, 0.0, 1.0, 1.0);
        r.rotation_deg = 5.0;
        assert!(!axis_aligned(&r));
        let mut f = t(0.0, 0.0, 1.0, 1.0);
        f.flip_v = true;
        assert!(!axis_aligned(&f));
    }
}
