//! # gip-text — the text engine
//!
//! Shaping, line breaking, and run geometry for text nodes. ONE layout result feeds render,
//! edit, select, and export (CAN-TEXT-1) — text geometry is derived, never stored in the
//! document. IME/composition law (CAN-TEXT-3) binds whichever editing path is chosen.
//! Contract: `docs/architecture/document-tree.md` §5 (+ CAN-TEXT in `rules/canvas.md`).
//!
//! **Scaffold status:** the box-mode vocabulary only. The shaping strategy (HarfBuzz-class
//! library vs platform shaping vs custom) is ADR-gated and deliberately NOT chosen here; no
//! text dependency is allowed until that ADR exists (CQ1.4).

#![forbid(unsafe_code)]

/// Text box sizing mode (CAN-TEXT-2). Explicit document data.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BoxMode {
    /// Width derives from content; the box grows horizontally.
    AutoWidth,
    /// Height derives from wrapping; the box grows vertically.
    AutoHeight,
    /// Both axes are fixed; content wraps/clips per text rules.
    Fixed,
}

impl BoxMode {
    /// All three modes, in canonical documentation order.
    pub const ALL: [BoxMode; 3] = [BoxMode::AutoWidth, BoxMode::AutoHeight, BoxMode::Fixed];

    /// Canonical lowercase name (matches documentation and UI vocabulary).
    #[must_use]
    pub const fn name(self) -> &'static str {
        match self {
            BoxMode::AutoWidth => "auto-width",
            BoxMode::AutoHeight => "auto-height",
            BoxMode::Fixed => "fixed",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn box_modes_match_text_law() {
        assert_eq!(
            BoxMode::ALL.len(),
            3,
            "CAN-TEXT-2 defines exactly three box modes"
        );
        assert_eq!(BoxMode::AutoHeight.name(), "auto-height");
    }
}
