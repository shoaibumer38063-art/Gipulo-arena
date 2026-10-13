//! Snapping (task 025, CAN-SNAP; activation rules from `docs/architecture/input-system.md`).
//!
//! A moving edge or center snaps to the edges and centers of other objects when it is within
//! [`SNAP_ACTIVATE_PX`] screen pixels. Once snapped, it stays snapped until it moves more than
//! [`SNAP_RELEASE_PX`] (hysteresis, so a snap does not flicker at the boundary). The core returns the
//! correction and guide descriptors; the shell only draws them.

use gip_document::Transform;

use crate::machine::Modifiers;

/// Screen distance at which a moving line snaps to a target (input-system §snapping).
pub const SNAP_ACTIVATE_PX: f64 = 4.0;
/// Screen distance at which a held snap is released (hysteresis, proposed in input-system).
pub const SNAP_RELEASE_PX: f64 = 6.0;

/// One snap guide, in world units. `vertical` guides are at `x = pos` from `y = from` to `to`.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Guide {
    /// True for a vertical line (an x target), false for a horizontal one.
    pub vertical: bool,
    /// The coordinate of the line on its normal axis.
    pub pos: f64,
    /// Start of the span along the line.
    pub from: f64,
    /// End of the span along the line.
    pub to: f64,
}

/// The target each axis is currently locked to, if any.
#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub struct SnapHold {
    /// Locked vertical target (an x coordinate).
    pub x: Option<f64>,
    /// Locked horizontal target (a y coordinate).
    pub y: Option<f64>,
}

/// The three snap lines of a box along one axis: start, center, end.
#[must_use]
pub fn lines(min: f64, size: f64) -> [f64; 3] {
    [min, min + size / 2.0, min + size]
}

/// Finds the best correction for moving `values` (lines on one axis) against `targets`.
/// Returns `(diff, target)`: add `diff` to move the nearest line onto `target`.
///
/// A held target keeps its snap while any moving line stays within `release`. Otherwise the
/// nearest target within `activate` wins. Distances are in world units.
#[must_use]
pub fn snap_value(
    values: &[f64],
    targets: &[f64],
    activate: f64,
    release: f64,
    hold: Option<f64>,
) -> Option<(f64, f64)> {
    if let Some(t) = hold {
        for v in values {
            let d = t - v;
            if d.abs() <= release {
                return Some((d, t));
            }
        }
    }
    let mut best: Option<(f64, f64)> = None;
    for v in values {
        for t in targets {
            let d = t - v;
            if d.abs() <= activate && best.is_none_or(|(b, _)| d.abs() < b.abs()) {
                best = Some((d, *t));
            }
        }
    }
    best
}

/// Whether snapping is switched off for this gesture (⌘ / Ctrl, input-system §modifiers).
#[must_use]
pub fn disabled(mods: Modifiers) -> bool {
    mods.snap_off
}

/// World boxes of the objects snapping may align to: visible, unlocked, not in `exclude`.
#[must_use]
pub fn candidate_boxes(
    doc: &gip_document::Document,
    exclude: &[gip_document::NodeId],
) -> Vec<(f64, f64, f64, f64)> {
    let Some(page) = doc.pages().first() else {
        return Vec::new();
    };
    page.children
        .iter()
        .filter(|id| !exclude.contains(id))
        .filter_map(|id| {
            let node = doc.nodes().get(id)?;
            (node.visible && !node.locked).then(|| transform_box(&node.transform))
        })
        .collect()
}

/// Axis-aligned world box of one transform (rotation and flips included).
#[must_use]
pub fn transform_box(t: &Transform) -> (f64, f64, f64, f64) {
    crate::group::group_box(std::slice::from_ref(t))
}

/// The guide for a snapped line at `pos`: spans the moving box and every candidate that has a line
/// at `pos`.
#[must_use]
pub fn guide_for(
    vertical: bool,
    pos: f64,
    moving: (f64, f64, f64, f64),
    boxes: &[(f64, f64, f64, f64)],
) -> Guide {
    let (mut from, mut to) = if vertical {
        (moving.1, moving.1 + moving.3)
    } else {
        (moving.0, moving.0 + moving.2)
    };
    for b in boxes {
        let (lo, size) = if vertical { (b.0, b.2) } else { (b.1, b.3) };
        let has_line = lines(lo, size).iter().any(|l| (l - pos).abs() < 1e-6);
        if has_line {
            let (a, s) = if vertical { (b.1, b.3) } else { (b.0, b.2) };
            from = from.min(a);
            to = to.max(a + s);
        }
    }
    Guide {
        vertical,
        pos,
        from,
        to,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_line_within_activation_snaps_to_the_nearest_target() {
        let r = snap_value(&[101.0], &[100.0, 110.0], 4.0, 6.0, None);
        assert_eq!(r, Some((-1.0, 100.0)));
    }

    #[test]
    fn a_line_outside_activation_does_not_snap() {
        assert_eq!(snap_value(&[105.0], &[100.0], 4.0, 6.0, None), None);
    }

    #[test]
    fn a_held_snap_is_released_only_past_the_release_distance() {
        // Held at 100: a line at 105 (5 px away) stays snapped, though it would not activate.
        assert_eq!(
            snap_value(&[105.0], &[100.0], 4.0, 6.0, Some(100.0)),
            Some((-5.0, 100.0))
        );
        // Past the release distance the hold lets go.
        assert_eq!(snap_value(&[107.0], &[100.0], 4.0, 6.0, Some(100.0)), None);
    }

    #[test]
    fn lines_are_start_center_end() {
        assert_eq!(lines(10.0, 20.0), [10.0, 20.0, 30.0]);
    }
}
