//! The `.gip` v1 container (ADR-003 #1, #2, #6, #8, #11): `{ format, version, meta, document,
//! assets }`, UTF-8 JSON, schema field order, explicit fields, pretty-printed with a trailing
//! newline. Save and load are pure functions of the value — no clocks, no randomness.

use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use crate::document::Document;
use crate::document::Page;
use crate::error::GipError;
use crate::id::{AssetId, NodeId, hex64};
use crate::integrity::{check_nfc, document_hash};

/// Format tag written in every file.
pub const FORMAT_TAG: &str = "gip";
/// Current container version (ADR-003 #8: integer, starts at 1).
pub const CURRENT_VERSION: u32 = 1;

/// Document metadata (ADR-003 #11).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Meta {
    /// Document name.
    pub name: String,
    /// Lineage salt, 24 bits, written as 16 hex digits.
    #[serde(with = "hex64")]
    pub doc_salt: u64,
    /// Counter high-water mark (next counter to allocate), written as 16 hex digits.
    #[serde(with = "hex64")]
    pub next_id: u64,
    /// Information only; never influences rendering or identity.
    pub app_version: String,
    /// SHA-256 of the canonical document section. `null` until the hashing dependency is
    /// registered (ADR-003 #11 — tracked in the task summary, not silently skipped).
    pub document_hash: Option<String>,
    /// Unrecognized keys preserved verbatim (ADR-003 #9).
    #[serde(flatten, default, skip_serializing_if = "BTreeMap::is_empty")]
    pub ext: BTreeMap<String, serde_json::Value>,
}

/// One asset manifest entry (ADR-003 #6/#7). Assets are immutable in v0.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AssetRecord {
    /// Content hash (lowercase SHA-256 hex, 64 digits) — the store key.
    pub hash: String,
    /// Media kind, e.g. `image`.
    pub kind: String,
    /// Byte length of the stored content.
    pub bytes: u64,
    /// True when the content is absent from the store (explicit, survives save/load).
    pub missing: bool,
    /// Unrecognized keys preserved verbatim (ADR-003 #9).
    #[serde(flatten, default, skip_serializing_if = "BTreeMap::is_empty")]
    pub ext: BTreeMap<String, serde_json::Value>,
}

/// The whole file.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GipFile {
    /// Always [`FORMAT_TAG`]. Crate-private fields: read through accessors; written only inside
    /// this crate (`edit` and the constructors).
    pub(crate) format: String,
    /// Container version.
    pub(crate) version: u32,
    /// Metadata.
    pub(crate) meta: Meta,
    /// The document body.
    pub(crate) document: Document,
    /// Asset manifest keyed by asset id.
    pub(crate) assets: BTreeMap<AssetId, AssetRecord>,
    /// Unrecognized keys preserved verbatim (ADR-003 #9).
    #[serde(flatten, default, skip_serializing_if = "BTreeMap::is_empty")]
    pub(crate) ext: BTreeMap<String, serde_json::Value>,
}

impl GipFile {
    /// A new document of the given lineage salt (24 bits) and name, with one empty page
    /// (`Page 1`, id counter 1) so that every new document has an edit target.
    #[must_use]
    pub fn new(name: impl Into<String>, doc_salt: u32) -> Self {
        // An out-of-range salt yields a placeholder id; `validate` rejects that document.
        let page_id = NodeId::compose(doc_salt, 1).unwrap_or(NodeId::from_raw(0));
        let document = Document {
            pages: vec![Page {
                id: page_id,
                name: "Page 1".into(),
                children: Vec::new(),
                ext: BTreeMap::new(),
            }],
            nodes: BTreeMap::new(),
            ext: BTreeMap::new(),
        };
        Self {
            format: FORMAT_TAG.into(),
            version: CURRENT_VERSION,
            meta: Meta {
                name: name.into(),
                doc_salt: u64::from(doc_salt),
                next_id: 2,
                app_version: env!("CARGO_PKG_VERSION").into(),
                document_hash: None,
                ext: BTreeMap::new(),
            },
            document,
            assets: BTreeMap::new(),
            ext: BTreeMap::new(),
        }
    }

    /// Container version (read-only).
    #[must_use]
    pub fn version(&self) -> u32 {
        self.version
    }

    /// Metadata (read-only).
    #[must_use]
    pub fn meta(&self) -> &Meta {
        &self.meta
    }

    /// The document body (read-only). Writes go through `edit`.
    #[must_use]
    pub fn document(&self) -> &Document {
        &self.document
    }

    /// Asset manifest (read-only).
    #[must_use]
    pub fn assets(&self) -> &BTreeMap<AssetId, AssetRecord> {
        &self.assets
    }

    /// Allocates a fresh node id from this lineage (ids are never reused, CAN-SER-3). Not an
    /// undoable write: an undone insert leaves a gap in the counter, by design.
    ///
    /// # Errors
    /// Returns a message when the 40-bit counter space is exhausted.
    pub fn allocate_id(&mut self) -> Result<NodeId, String> {
        let salt = u32::try_from(self.meta.doc_salt).map_err(|_| "salt out of range")?;
        let counter = self.meta.next_id;
        let id = NodeId::compose(salt, counter).ok_or("id space exhausted")?;
        self.meta.next_id = counter.checked_add(1).ok_or("id counter overflow")?;
        Ok(id)
    }

    /// Validates the whole file: tag, version, metadata ranges, document invariants, assets.
    ///
    /// # Errors
    /// Returns the first violated rule.
    pub fn validate(&self) -> Result<(), GipError> {
        if self.format != FORMAT_TAG {
            return Err(GipError::UnsupportedFormat(self.format.clone()));
        }
        if self.version != CURRENT_VERSION {
            return Err(GipError::UnsupportedVersion(self.version));
        }
        if self.meta.doc_salt >= 1 << crate::id::SALT_BITS {
            return Err(GipError::Invalid("docSalt exceeds 24 bits".into()));
        }
        let salt = self.meta.doc_salt as u32;
        self.document.validate(salt, self.meta.next_id)?;
        for (id, asset) in &self.assets {
            if id.salt() != salt || id.counter() >= self.meta.next_id {
                return Err(GipError::Invalid(format!(
                    "asset {id}: id outside lineage or above nextId"
                )));
            }
            if self
                .document
                .nodes
                .keys()
                .any(|n| n.to_raw() == id.to_raw())
                || self
                    .document
                    .pages
                    .iter()
                    .any(|p| p.id.to_raw() == id.to_raw())
            {
                return Err(GipError::Invalid(format!(
                    "asset {id} collides with a document id"
                )));
            }
            let hash_ok = asset.hash.len() == 64
                && asset
                    .hash
                    .bytes()
                    .all(|b| b.is_ascii_digit() || (b'a'..=b'f').contains(&b));
            if !hash_ok {
                return Err(GipError::Invalid(format!(
                    "asset {id}: hash must be 64 lowercase hex digits"
                )));
            }
        }
        Ok(())
    }
}

/// Migration steps, keyed by the version they upgrade FROM. Empty at v1: the first schema
/// change adds its step here in the same task (ADR-003 #8). Forward-only.
type MigrationStep = fn(&mut serde_json::Value) -> Result<(), GipError>;
const MIGRATIONS: &[(u32, MigrationStep)] = &[];

/// Serializes a file to canonical text: validates first (never writes NaN/Inf, which
/// serde_json would silently turn into `null`), then emits schema order with a trailing newline.
///
/// # Errors
/// Returns the first validation failure, or a serialization error.
pub fn to_json(file: &GipFile) -> Result<String, GipError> {
    file.validate()?;
    check_nfc(&serde_json::to_value(file)?)?;
    let mut stamped = file.clone();
    stamped.meta.document_hash = Some(document_hash(&file.document)?);
    let mut text = serde_json::to_string_pretty(&stamped)?;
    text.push('\n');
    Ok(text)
}

/// Parses and validates a file, applying forward migrations to the current version.
///
/// # Errors
/// Returns [`GipError::Json`] for malformed text, [`GipError::UnsupportedFormat`] /
/// [`GipError::UnsupportedVersion`] for foreign or newer files, [`GipError::Invalid`] for
/// violated rules.
pub fn from_json(text: &str) -> Result<GipFile, GipError> {
    let mut value: serde_json::Value = serde_json::from_str(text)?;
    let format = value
        .get("format")
        .and_then(|f| f.as_str())
        .unwrap_or_default()
        .to_owned();
    if format != FORMAT_TAG {
        return Err(GipError::UnsupportedFormat(format));
    }
    let mut version = value
        .get("version")
        .and_then(serde_json::Value::as_u64)
        .and_then(|v| u32::try_from(v).ok())
        .ok_or_else(|| GipError::Invalid("missing or invalid version".into()))?;
    if version > CURRENT_VERSION {
        return Err(GipError::UnsupportedVersion(version));
    }
    while version < CURRENT_VERSION {
        let (_, step) = MIGRATIONS
            .iter()
            .find(|(from, _)| *from == version)
            .ok_or_else(|| GipError::Invalid(format!("no migration from version {version}")))?;
        step(&mut value)?;
        version += 1;
        value["version"] = serde_json::Value::from(version);
    }
    check_nfc(&value)?;
    let file: GipFile = serde_json::from_value(value)?;
    file.validate()?;
    if let Some(stored) = &file.meta.document_hash
        && *stored != document_hash(&file.document)?
    {
        return Err(GipError::Invalid("document hash mismatch".into()));
    }
    Ok(file)
}
