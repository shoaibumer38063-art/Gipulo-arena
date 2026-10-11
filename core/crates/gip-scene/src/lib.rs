//! # gip-scene — retained scene and invalidation
//!
//! Holds the retained scene derived from (document + geometry + camera): paint ops,
//! transforms, bounds, cache keys. Tracks dirty regions from transaction effects and camera
//! changes so the renderer redraws the minimum (R2). The scene consumes truth; it never
//! writes it (R1).
//! Contract: `docs/architecture/rendering.md` §2–3.
//!
//! **Scaffold status:** the render-layer vocabulary only. Scene graph, invalidation, culling,
//! and cache bookkeeping land with the rendering phase.

#![forbid(unsafe_code)]

/// The two render passes (rendering.md §2). Editor chrome (overlays) NEVER mixes into the
/// document layer — overlays are not exportable, not serializable, not selectable (CAN-OVL).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RenderLayer {
    /// Document content — the only layer that export renders.
    Document,
    /// Screen-space editor chrome (selection, guides, snap lines, handles).
    Overlay,
}

impl RenderLayer {
    /// Both layers, in draw order.
    pub const ALL: [RenderLayer; 2] = [RenderLayer::Document, RenderLayer::Overlay];

    /// Canonical lowercase name (matches documentation and diagnostics).
    #[must_use]
    pub const fn name(self) -> &'static str {
        match self {
            RenderLayer::Document => "document",
            RenderLayer::Overlay => "overlay",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn layers_are_ordered_document_then_overlay() {
        assert_eq!(RenderLayer::ALL[0], RenderLayer::Document);
        assert_eq!(RenderLayer::ALL[1], RenderLayer::Overlay);
        assert_ne!(RenderLayer::Document, RenderLayer::Overlay);
    }
}
