//! The single adapter between the persisted transform (`gip_document::Transform`, ADR-002 §1)
//! and the engine's math type (`gip_coords::Trs`). The two structs mirror each other
//! field-for-field; both directions use EXHAUSTIVE destructuring so that adding, renaming, or
//! retyping a field on either side fails to compile here instead of silently drifting.

use gip_coords::Trs;
use gip_document::Transform;

/// Persisted transform → engine transform (values copied unchanged; no normalization).
#[must_use]
pub fn trs_of(t: &Transform) -> Trs {
    let Transform {
        x,
        y,
        w,
        h,
        rotation_deg,
        flip_h,
        flip_v,
    } = *t;
    Trs {
        x,
        y,
        w,
        h,
        rotation_deg,
        flip_h,
        flip_v,
    }
}

/// Engine transform → persisted transform (values copied unchanged; no normalization).
#[must_use]
pub fn transform_of(t: &Trs) -> Transform {
    let Trs {
        x,
        y,
        w,
        h,
        rotation_deg,
        flip_h,
        flip_v,
    } = *t;
    Transform {
        x,
        y,
        w,
        h,
        rotation_deg,
        flip_h,
        flip_v,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn both_directions_are_lossless_for_every_field() {
        let t = Transform {
            x: 1.5,
            y: -2.25,
            w: 3.0,
            h: 4.0,
            rotation_deg: -90.0,
            flip_h: true,
            flip_v: false,
        };
        let back = transform_of(&trs_of(&t));
        assert_eq!(back, t);
        let trs = Trs {
            x: 0.1,
            y: 0.2,
            w: 0.3,
            h: 0.4,
            rotation_deg: 180.0,
            flip_h: false,
            flip_v: true,
        };
        assert_eq!(trs_of(&transform_of(&trs)), trs);
    }
}
