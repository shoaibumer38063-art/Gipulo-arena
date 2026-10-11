//! Central numeric tolerances (CAN-NUM-5) — the ONE place epsilons may be defined.
//!
//! ADR-002 §10 decided this module: no other module or crate may introduce a tolerance
//! constant; tests cite these values instead of literals. Display rounding is a shell
//! formatting concern and never touches stored values (CAN-NUM-2).
//!
//! Cross-platform note: `sin`/`cos`/`atan2` may differ by an ulp between platforms and
//! engines. Every comparison in this crate is epsilon-based so such differences are absorbed;
//! byte-level determinism (CAN-SER-1) applies to serialized data, not to transcendental
//! last bits. Platform parity is measured by the budget/parity suites, not assumed.

/// Round-trip identity tolerance, in world units. CAN-COORD invariant: `A→B(B→A(p)) ≈ p`
/// within this value, for all space pairs, at zoom 0.05–32, any rotation/flip/nesting.
pub const EPS_ROUNDTRIP: f64 = 1e-9;

/// Angle equality tolerance, in degrees. Exactly-snapped commits (0/±90/180) must compare
/// equal bit-for-bit (CAN-TR-4, ADR-002 §5); this tolerance covers the general case.
pub const EPS_ANGLE: f64 = 1e-9;

/// Maximum absolute entry difference for two matrices to compare equal (ADR-002 §10).
pub const EPS_MATRIX: f64 = 1e-9;

/// Relative (unitless) tolerance for orthogonality/degeneracy classification, where an
/// absolute tolerance would be scale-dependent — e.g. skew detection in
/// [`crate::Mat2::decompose`].
pub const EPS_REL: f64 = 1e-12;
