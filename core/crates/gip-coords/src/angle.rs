//! Canonical angle convention (ADR-002 §5): degrees, `f64`, normalized to `(−180, 180]`.
//!
//! ONE convention in storage, UI, tests and files; radians exist only inside trig calls in
//! this crate (ADR-002 §5, CAN-TR-4).

/// Normalizes an angle in degrees into the canonical range `(−180, 180]`.
///
/// - `−180.0` maps to `+180.0` (the range is half-open on the left).
/// - `−0.0` maps to `+0.0` so serialization is byte-stable (ADR-003 determinism).
/// - NaN / ±Inf are returned unchanged: they are invalid *document* values (ADR-003 schema)
///   and are rejected at the schema edge; laundering them here would hide bugs.
///
/// ```
/// use gip_coords::angle::normalize_deg;
/// assert_eq!(normalize_deg(270.0), -90.0);
/// assert_eq!(normalize_deg(180.0), 180.0);
/// assert_eq!(normalize_deg(-180.0), 180.0);
/// assert_eq!(normalize_deg(360.0), 0.0);
/// ```
#[must_use]
pub fn normalize_deg(deg: f64) -> f64 {
    if !deg.is_finite() {
        return deg;
    }
    let mut d = deg % 360.0;
    if d <= -180.0 {
        d += 360.0;
    } else if d > 180.0 {
        d -= 360.0;
    }
    // Collapse −0.0 to +0.0 (bit-stable serialization; ADR-003 "no locale/format variance").
    if d == 0.0 { 0.0 } else { d }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Exactly-snapped values must survive normalization bit-for-bit (ADR-002 §5:
    /// after commit cleanup, 0/±90/180 compare equal exactly).
    #[test]
    fn snapped_values_are_bit_exact() {
        let cases: [(f64, f64); 9] = [
            (0.0, 0.0),
            (90.0, 90.0),
            (-90.0, -90.0),
            (180.0, 180.0),
            (-180.0, 180.0),
            (360.0, 0.0),
            (270.0, -90.0),
            (-270.0, 90.0),
            (450.0, 90.0),
        ];
        for (input, expected) in cases {
            assert_eq!(
                normalize_deg(input).to_bits(),
                expected.to_bits(),
                "normalize_deg({input})"
            );
        }
    }

    #[test]
    fn negative_zero_collapses_to_positive_zero() {
        assert_eq!(normalize_deg(-0.0).to_bits(), 0.0f64.to_bits());
        assert_eq!(normalize_deg(-360.0).to_bits(), 0.0f64.to_bits());
    }

    #[test]
    fn range_is_half_open_and_idempotent() {
        let mut d = -4_000.0;
        while d < 4_000.0 {
            let n = normalize_deg(d);
            assert!(n > -180.0 && n <= 180.0, "range violated at {d} → {n}");
            assert_eq!(
                normalize_deg(n).to_bits(),
                n.to_bits(),
                "not idempotent at {d}"
            );
            d += 0.5;
        }
    }

    #[test]
    fn non_finite_passes_through_unlaundered() {
        assert!(normalize_deg(f64::NAN).is_nan());
        assert_eq!(normalize_deg(f64::INFINITY), f64::INFINITY);
        assert_eq!(normalize_deg(f64::NEG_INFINITY), f64::NEG_INFINITY);
    }
}
