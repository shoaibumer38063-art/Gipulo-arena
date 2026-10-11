//! 2D `f64` vector — the value type for every point, offset and delta in this crate.
//!
//! Boundary note: points cross to the shell as `f64` pairs only (engine-core.md §3);
//! this type never crosses as an object.

use std::ops::{Add, Mul, Sub};

/// A 2D point or delta in any of the canonical coordinate spaces (the space is carried by
/// context/type flow, never by the value — CAN-COORD-1: spaces are named at the API edge).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vec2 {
    /// X component.
    pub x: f64,
    /// Y component.
    pub y: f64,
}

impl Vec2 {
    /// The origin / zero delta.
    pub const ZERO: Self = Self::new(0.0, 0.0);

    /// Creates a vector from components.
    #[must_use]
    pub const fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }

    /// Multiplies both components by a scalar.
    #[must_use]
    pub const fn scale(self, factor: f64) -> Self {
        Self::new(self.x * factor, self.y * factor)
    }

    /// Dot product.
    #[must_use]
    pub fn dot(self, other: Self) -> f64 {
        self.x * other.x + self.y * other.y
    }

    /// 2D cross product (z component of the 3D cross): positive when `other` is counter-clockwise.
    #[must_use]
    pub fn cross(self, other: Self) -> f64 {
        self.x * other.y - self.y * other.x
    }

    /// Squared length.
    #[must_use]
    pub fn length_sq(self) -> f64 {
        self.dot(self)
    }

    /// Squared distance to another point.
    #[must_use]
    pub fn distance_sq(self, other: Self) -> f64 {
        (self - other).length_sq()
    }

    /// Component-wise minimum.
    #[must_use]
    pub fn min_components(self, other: Self) -> Self {
        Self::new(self.x.min(other.x), self.y.min(other.y))
    }

    /// Component-wise maximum.
    #[must_use]
    pub fn max_components(self, other: Self) -> Self {
        Self::new(self.x.max(other.x), self.y.max(other.y))
    }

    /// Linear interpolation: `self` at `t = 0`, `other` at `t = 1`.
    #[must_use]
    pub fn lerp(self, other: Self, t: f64) -> Self {
        Self::new(
            self.x + (other.x - self.x) * t,
            self.y + (other.y - self.y) * t,
        )
    }

    /// True when both components are finite (NaN/±Inf are invalid document values).
    #[must_use]
    pub fn is_finite(self) -> bool {
        self.x.is_finite() && self.y.is_finite()
    }

    /// Component-wise approximate equality within `eps` (world units).
    #[must_use]
    pub fn approx_eq(self, other: Self, eps: f64) -> bool {
        (self.x - other.x).abs() <= eps && (self.y - other.y).abs() <= eps
    }
}

impl Default for Vec2 {
    fn default() -> Self {
        Self::ZERO
    }
}

impl Add for Vec2 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Self::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl Sub for Vec2 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        Self::new(self.x - rhs.x, self.y - rhs.y)
    }
}

impl Mul<f64> for Vec2 {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self {
        self.scale(rhs)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn arithmetic_and_approx_equality() {
        let a = Vec2::new(1.5, -2.0);
        let b = Vec2::new(0.5, 0.25);
        assert_eq!(a + b, Vec2::new(2.0, -1.75));
        assert_eq!(a - b, Vec2::new(1.0, -2.25));
        assert_eq!(a * 2.0, Vec2::new(3.0, -4.0));
        assert!(a.approx_eq(Vec2::new(1.5 + 1e-12, -2.0), 1e-9));
        assert!(!a.approx_eq(Vec2::new(1.5 + 1e-6, -2.0), 1e-9));
    }

    #[test]
    fn non_finite_detection() {
        assert!(!Vec2::new(f64::NAN, 0.0).is_finite());
        assert!(!Vec2::new(0.0, f64::INFINITY).is_finite());
        assert!(Vec2::new(1e308, -1e308).is_finite());
    }
}
