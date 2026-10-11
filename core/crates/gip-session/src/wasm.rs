//! wasm-bindgen boundary surface — the Rust source of truth for the generated bindings.
//!
//! Transport ONLY: nothing in this module may contain editor logic (Boundary Duplication Law,
//! engine-core.md §1). Generated output lives in `bindings/pkg/` and is never hand-edited.

#![allow(unreachable_pub)] // wasm-bindgen requires pub items inside this private module.

use wasm_bindgen::prelude::*;

use crate::PROTOCOL_VERSION;

/// Returns the boundary protocol version for the shell↔core handshake (engine-core.md §10).
#[wasm_bindgen]
#[must_use]
pub fn protocol_version() -> u32 {
    PROTOCOL_VERSION
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn wasm_surface_exposes_protocol_version() {
        assert_eq!(protocol_version(), PROTOCOL_VERSION);
    }
}
