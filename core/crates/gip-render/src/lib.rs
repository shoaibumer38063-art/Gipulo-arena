//! # gip-render — the renderer (wgpu)
//!
//! Turns (document + geometry + camera) into pixels. Per ADR-001 (ACCEPTED — Option B) the
//! renderer lives INSIDE the Rust/Wasm core and draws through **wgpu**: browser WebGPU
//! pass-through when available, WebGL2 via wgpu's `webgl` feature + Naga translation as the
//! fallback. This crate is the ONLY crate allowed to depend on wgpu
//! (`docs/architecture/dependencies.md`; enforced by `scripts/check-boundaries.py`).
//! Contract: `docs/architecture/rendering.md`.
//!
//! **Scaffold status:** the backend-priority law encoded as a constant only. Pipeline, scene
//! drawing, effects, invalidation, and export rendering land with the rendering phase; no
//! pixels are produced yet.

#![forbid(unsafe_code)]

/// Browser backend priority (ADR-001): WebGPU primary, WebGL2 fallback — matching the
/// publicly shipped Figma posture of dynamic fallback on incompatible devices.
pub const BROWSER_BACKEND_PRIORITY: [wgpu::Backend; 2] =
    [wgpu::Backend::BrowserWebGpu, wgpu::Backend::Gl];

/// The single shader source-of-truth language (WGSL — constrained by wgpu; Naga translates
/// for the WebGL2 fallback path).
pub const SHADER_LANGUAGE: &str = "wgsl";

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn webgpu_is_primary_and_webgl2_is_fallback() {
        assert_eq!(BROWSER_BACKEND_PRIORITY[0], wgpu::Backend::BrowserWebGpu);
        assert_eq!(BROWSER_BACKEND_PRIORITY[1], wgpu::Backend::Gl);
        assert_eq!(SHADER_LANGUAGE, "wgsl");
    }
}
