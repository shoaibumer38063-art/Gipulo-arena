//! Integrity rules (ADR-009): `documentHash` = SHA-256 over the canonical `document` section, and
//! NFC normalization of every string in a file. Pure functions of their input.

use sha2::{Digest, Sha256};
use unicode_normalization::is_nfc;

use crate::document::Document;
use crate::error::GipError;

/// Lowercase hex SHA-256 over the compact, schema-ordered JSON bytes of the document section
/// (ADR-009 #1–#2).
///
/// # Errors
/// Returns a serialization error only if the document cannot be serialized.
pub(crate) fn document_hash(document: &Document) -> Result<String, GipError> {
    let bytes = serde_json::to_vec(document)?;
    let digest = Sha256::digest(&bytes);
    Ok(digest.iter().map(|b| format!("{b:02x}")).collect())
}

/// Rejects any non-NFC string, keys included, naming the first offending JSON path (ADR-009 #5).
/// Strings are never rewritten.
///
/// # Errors
/// Returns [`GipError::Invalid`] at the first non-NFC string.
pub(crate) fn check_nfc(value: &serde_json::Value) -> Result<(), GipError> {
    check_nfc_at(value, "$")
}

fn check_nfc_at(value: &serde_json::Value, path: &str) -> Result<(), GipError> {
    match value {
        serde_json::Value::String(s) if !is_nfc(s) => Err(GipError::Invalid(format!(
            "string at {path} is not NFC-normalized"
        ))),
        serde_json::Value::Array(items) => items
            .iter()
            .enumerate()
            .try_for_each(|(i, item)| check_nfc_at(item, &format!("{path}[{i}]"))),
        serde_json::Value::Object(map) => map.iter().try_for_each(|(key, item)| {
            if !is_nfc(key) {
                return Err(GipError::Invalid(format!(
                    "key at {path} is not NFC-normalized"
                )));
            }
            check_nfc_at(item, &format!("{path}.{key}"))
        }),
        _ => Ok(()),
    }
}
