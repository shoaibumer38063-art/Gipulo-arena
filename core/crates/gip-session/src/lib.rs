//! # gip-session — the engine facade and wasm-bindgen boundary
//!
//! The ONE crate the shell talks to. Shell intents go DOWN (typed commands); discrete events
//! and per-frame snapshot reads come UP (engine-core.md §3). The generated bindings package
//! (`bindings/pkg`, built by `scripts/build-wasm.sh`) is never hand-edited.
//!
//! Boundary laws enforced by construction here: the shell has NO document write API (SM3 —
//! mutations are impossible except as core transactions); no editor logic may exist on both
//! sides of the boundary (Boundary Duplication Law, engine-core.md §1).
//!
//! **Scaffold status:** protocol-version handshake only. Event feeding, intents, snapshots,
//! and rendering surfaces land with the boundary phase.

#![forbid(unsafe_code)]

/// Boundary protocol version — the shell↔core handshake (engine-core.md §10). A mismatch is a
/// clear startup error, never silent misbehavior. Bump on every breaking boundary change.
pub const PROTOCOL_VERSION: u32 = 4;

/// Core crates re-exported as the native facade surface (tests, tools, CI consumers).
pub use gip_coords as coords;
pub use gip_document as document;
pub use gip_geometry as geometry;
pub use gip_history as history;
pub use gip_layout as layout;
pub use gip_render as render;
pub use gip_scene as scene;
pub use gip_text as text;

pub mod fixture;
pub mod session;
pub use session::{RectOp, Session, ViewSnapshot};

#[cfg(target_arch = "wasm32")]
mod wasm;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn facade_reexports_core_crates() {
        // The facade must expose every core crate (native consumers depend on this).
        assert_eq!(gip_coords::Space::ALL.len(), 8);
        assert_eq!(gip_document::NodeId::from_raw(1).to_raw(), 1);
        assert_eq!(gip_render::SHADER_LANGUAGE, "wgsl");
    }

    #[test]
    fn protocol_version_is_four_after_the_resize_boundary_was_added() {
        assert_eq!(PROTOCOL_VERSION, 4);
    }
}
