//! Persisted transform fields (ADR-002 §1): the seven canonical values, mirrored field-for-field
//! from `gip_coords::Trs`. This crate cannot depend on gip-coords (layer-1 law); the bridge and
//! its exhaustive drift guard live in `gip-geometry::bridge`.
//!
//! Transform is `Copy` and sits on hot paths, so it carries NO `ext` map (ADR-003 #9 amendment).

use serde::{Deserialize, Serialize};

/// Canonical decomposed transform. Field order is the file order (ADR-003 #2).
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct Transform {
    /// Translation x in parent space.
    pub x: f64,
    /// Translation y in parent space.
    pub y: f64,
    /// Width (stored ≥ 0).
    pub w: f64,
    /// Height (stored ≥ 0).
    pub h: f64,
    /// Rotation in degrees, canonical range `(−180, 180]`.
    #[serde(rename = "rotation")]
    pub rotation_deg: f64,
    /// Horizontal mirror flag.
    pub flip_h: bool,
    /// Vertical mirror flag.
    pub flip_v: bool,
}

impl Transform {
    /// The identity placement at the parent origin with zero size.
    pub const ZERO: Self = Self {
        x: 0.0,
        y: 0.0,
        w: 0.0,
        h: 0.0,
        rotation_deg: 0.0,
        flip_h: false,
        flip_v: false,
    };

    /// Checks the stored-form laws (ADR-002 §1/§5): finite, sizes ≥ 0, rotation in `(−180, 180]`.
    ///
    /// # Errors
    /// Returns the first violated law as a reason string.
    pub fn validate(&self) -> Result<(), String> {
        let finite = [self.x, self.y, self.w, self.h, self.rotation_deg]
            .iter()
            .all(|v| v.is_finite());
        if !finite {
            return Err("transform contains a non-finite value".into());
        }
        if self.w < 0.0 || self.h < 0.0 {
            return Err("transform sizes must be ≥ 0 (mirroring uses flipH/flipV)".into());
        }
        if !(self.rotation_deg > -180.0 && self.rotation_deg <= 180.0) {
            return Err(format!(
                "rotation {} outside (−180, 180]",
                self.rotation_deg
            ));
        }
        Ok(())
    }
}
