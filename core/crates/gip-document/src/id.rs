//! Identifiers (ADR-003 #4/#5): 64-bit values = `salt(24) << 40 | counter(40)`.
//!
//! Wire encoding (ADR-003 amendment, sub-decision 14): every identifier and every u64 metadata
//! value is a 16-digit lowercase hexadecimal JSON string. JSON numbers cannot carry 64-bit
//! identifiers losslessly (JavaScript safe-integer limit 2^53), so numbers are never used for IDs.

use serde::de::{self, Deserializer};
use serde::{Deserialize, Serialize, Serializer};

/// Bits of the per-document salt (high bits of every identifier).
pub const SALT_BITS: u32 = 24;
/// Bits of the monotonic counter (low bits of every identifier).
pub const COUNTER_BITS: u32 = 40;
const COUNTER_MASK: u64 = (1 << COUNTER_BITS) - 1;

/// Decodes the strict wire form: exactly 16 lowercase hex digits.
///
/// # Errors
/// Returns a reason string when the text is not exactly 16 lowercase hex digits.
pub fn decode_hex64(text: &str) -> Result<u64, String> {
    let valid = text.len() == 16
        && text
            .bytes()
            .all(|b| b.is_ascii_digit() || (b'a'..=b'f').contains(&b));
    if !valid {
        return Err(format!("expected 16 lowercase hex digits, got {text:?}"));
    }
    u64::from_str_radix(text, 16).map_err(|e| e.to_string())
}

/// Serde adapter for plain `u64` fields (`#[serde(with = "hex64")]`).
pub mod hex64 {
    use super::{Deserializer, Serializer, de, decode_hex64};
    use serde::Deserialize;

    /// Writes `value` as 16 lowercase hex digits.
    pub fn serialize<S: Serializer>(value: &u64, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_str(&format!("{value:016x}"))
    }

    /// Reads the strict 16-digit form.
    pub fn deserialize<'de, D: Deserializer<'de>>(d: D) -> Result<u64, D::Error> {
        let text = String::deserialize(d)?;
        decode_hex64(&text).map_err(de::Error::custom)
    }
}

macro_rules! define_id {
    ($(#[$doc:meta])* $name:ident) => {
        $(#[$doc])*
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
        pub struct $name(u64);

        impl $name {
            /// Wraps a raw 64-bit value (boundary representation or decoded wire value).
            #[must_use]
            pub const fn from_raw(raw: u64) -> Self {
                Self(raw)
            }

            /// Returns the raw 64-bit value.
            #[must_use]
            pub const fn to_raw(self) -> u64 {
                self.0
            }

            /// Composes an identifier from a document salt and a counter. `None` when either
            /// exceeds its bit width (ADR-003 #4).
            #[must_use]
            pub fn compose(salt: u32, counter: u64) -> Option<Self> {
                let salt_ok = u64::from(salt) < (1 << SALT_BITS);
                (salt_ok && counter <= COUNTER_MASK).then(|| Self((u64::from(salt) << COUNTER_BITS) | counter))
            }

            /// The salt (lineage) part.
            #[must_use]
            pub const fn salt(self) -> u32 {
                (self.0 >> COUNTER_BITS) as u32
            }

            /// The counter part.
            #[must_use]
            pub const fn counter(self) -> u64 {
                self.0 & COUNTER_MASK
            }
        }

        impl std::fmt::Display for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{:016x}", self.0)
            }
        }

        impl Serialize for $name {
            fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
                hex64::serialize(&self.0, s)
            }
        }

        impl<'de> Deserialize<'de> for $name {
            fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
                hex64::deserialize(d).map(Self)
            }
        }
    };
}

define_id! {
    /// Stable identity of a document node (CAN-SER-3: unique, stable, never reused).
    NodeId
}

define_id! {
    /// Stable identity of an asset record; same allocation rules as [`NodeId`] (ADR-003 #7).
    AssetId
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn compose_and_split_are_inverse() {
        let id = NodeId::compose(0x00ab_12cd, 977).unwrap();
        assert_eq!(id.salt(), 0x00ab_12cd);
        assert_eq!(id.counter(), 977);
        assert!(
            NodeId::compose(1 << SALT_BITS, 0).is_none(),
            "salt is 24 bits"
        );
        assert!(
            NodeId::compose(0, 1 << COUNTER_BITS).is_none(),
            "counter is 40 bits"
        );
    }

    #[test]
    fn wire_form_is_16_lowercase_hex_and_round_trips_large_values() {
        let id = NodeId::compose(0x00ff_ffff, COUNTER_MASK).unwrap();
        let json = serde_json::to_string(&id).unwrap();
        assert_eq!(json, "\"ffffffffffffffff\"");
        let back: NodeId = serde_json::from_str(&json).unwrap();
        assert_eq!(back, id);
    }

    #[test]
    fn decoder_rejects_non_canonical_text() {
        for bad in [
            "FFFFFFFFFFFFFFFF",
            "0123",
            "00000000000000000",
            "zzzzzzzzzzzzzzzz",
            "",
        ] {
            assert!(decode_hex64(bad).is_err(), "accepted {bad:?}");
        }
    }
}
