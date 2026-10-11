//! # gip-layout — the layout engine (Auto Layout · constraints · RVS)
//!
//! Computes ALL automatic positioning from document data. Layout results are DERIVED state:
//! cached, invalidated, never hand-edited positions for laid-out children (L1).
//! Contract: `docs/architecture/layout-engine.md`.
//!
//! Includes the stretch-region semantics of **Responsive Vector Segments (RVS)** — the
//! signature feature (product law L3): only activated straight segments absorb resize deltas;
//! curves and cutouts are geometrically preserved (L3-1, bit-preservation invariant).
//! RVS redistribution semantics are ADR-gated (L3-4) and deliberately NOT implemented here yet.
//!
//! **Scaffold status:** the sizing-mode vocabulary only. Determinism + 1000× stability suites
//! land with the layout phase (L5).

#![forbid(unsafe_code)]

/// Per-axis sizing mode (L2). Explicit document data — never inferred.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SizingMode {
    /// The axis takes its stored size.
    Fixed,
    /// The axis shrinks to fit the node's content.
    Hug,
    /// The axis fills the resolved parent inner size.
    Fill,
}

impl SizingMode {
    /// All three modes, in canonical documentation order.
    pub const ALL: [SizingMode; 3] = [SizingMode::Fixed, SizingMode::Hug, SizingMode::Fill];

    /// Canonical lowercase name (matches documentation and UI vocabulary).
    #[must_use]
    pub const fn name(self) -> &'static str {
        match self {
            SizingMode::Fixed => "fixed",
            SizingMode::Hug => "hug",
            SizingMode::Fill => "fill",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sizing_modes_match_layout_law() {
        assert_eq!(
            SizingMode::ALL.len(),
            3,
            "L2 defines exactly three sizing modes"
        );
        assert_eq!(SizingMode::Hug.name(), "hug");
        assert_eq!(SizingMode::Fill.name(), "fill");
    }
}
