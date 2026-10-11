//! Node data (document-tree.md §2): kind, name, transform, presentation flags, children, and
//! the path payload for vector paths. Structural rules (which kinds may hold children, which
//! carry paths) are enforced by `Document::validate`, not by this type.

use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use crate::id::NodeId;
use crate::path::Path;
use crate::transform::Transform;

/// The node-type set (document-tree.md §2). `#[non_exhaustive]`: additions go through the
/// feature process, never speculatively (CQ1.2). File names are kebab-case.
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum NodeKind {
    /// Frame — clip-capable container with its own child coordinate space (CAN-FRAME-1).
    Frame,
    /// Group — no clip, no layout; bounds are derived (CAN-FRAME-1).
    Group,
    /// Rectangle.
    Rectangle,
    /// Rounded rectangle.
    RoundedRect,
    /// Ellipse.
    Ellipse,
    /// Line.
    Line,
    /// Arrow.
    Arrow,
    /// Polygon.
    Polygon,
    /// Star.
    Star,
    /// Editable vector path (canonical path model — ADR-006).
    VectorPath,
    /// Non-destructive boolean group of children (VG2).
    BooleanGroup,
    /// Text node (CAN-TEXT contract; text engine not yet implemented).
    Text,
    /// Raster image (media is referenced, never inlined — CAN-MEDIA-1).
    Image,
    /// Video (media reference).
    Video,
    /// Component definition (main).
    Component,
    /// Component set / variants.
    ComponentSet,
    /// Component instance with property-level override records (CAN-COMP-1).
    Instance,
    /// Mask container (CAN-MASK-1).
    MaskContainer,
}

impl NodeKind {
    /// Kinds that may hold child nodes.
    #[must_use]
    pub const fn is_container(self) -> bool {
        matches!(
            self,
            NodeKind::Frame
                | NodeKind::Group
                | NodeKind::BooleanGroup
                | NodeKind::Component
                | NodeKind::ComponentSet
                | NodeKind::MaskContainer
        )
    }
}

/// One node of the document. Every field is written explicitly (ADR-003 #2).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Node {
    /// Node type.
    pub kind: NodeKind,
    /// Human name.
    pub name: String,
    /// Placement in the parent's space.
    pub transform: Transform,
    /// Opacity in `[0, 1]`.
    pub opacity: f64,
    /// Visibility flag.
    pub visible: bool,
    /// Lock flag.
    pub locked: bool,
    /// Ordered children (non-empty only for container kinds).
    pub children: Vec<NodeId>,
    /// Path payload (required for `VectorPath`, forbidden otherwise).
    pub path: Option<Path>,
    /// Unrecognized keys preserved verbatim on load → save (ADR-003 #9).
    #[serde(flatten, default, skip_serializing_if = "BTreeMap::is_empty")]
    pub ext: BTreeMap<String, serde_json::Value>,
}

impl Node {
    /// A visible, unlocked node with the given kind, name, and transform; no children or path.
    #[must_use]
    pub fn new(kind: NodeKind, name: impl Into<String>, transform: Transform) -> Self {
        Self {
            kind,
            name: name.into(),
            transform,
            opacity: 1.0,
            visible: true,
            locked: false,
            children: Vec::new(),
            path: None,
            ext: BTreeMap::new(),
        }
    }
}

/// Scalar node fields addressable by edit operations (ADR-004 `SetField`). The set is closed;
/// adding one is a schema change (same task as its serializer field).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FieldKey {
    /// Transform x.
    X,
    /// Transform y.
    Y,
    /// Transform width.
    W,
    /// Transform height.
    H,
    /// Transform rotation in degrees.
    Rotation,
    /// Horizontal mirror flag.
    FlipH,
    /// Vertical mirror flag.
    FlipV,
    /// Opacity.
    Opacity,
    /// Visibility flag.
    Visible,
    /// Lock flag.
    Locked,
    /// Name.
    Name,
}

/// A value of one [`FieldKey`]. The variant must match the key's type.
#[derive(Debug, Clone, PartialEq)]
pub enum FieldValue {
    /// Numeric field.
    Number(f64),
    /// Boolean field.
    Flag(bool),
    /// Text field.
    Text(String),
}

impl Node {
    /// Reads one scalar field.
    #[must_use]
    pub fn field(&self, key: FieldKey) -> FieldValue {
        let t = &self.transform;
        match key {
            FieldKey::X => FieldValue::Number(t.x),
            FieldKey::Y => FieldValue::Number(t.y),
            FieldKey::W => FieldValue::Number(t.w),
            FieldKey::H => FieldValue::Number(t.h),
            FieldKey::Rotation => FieldValue::Number(t.rotation_deg),
            FieldKey::FlipH => FieldValue::Flag(t.flip_h),
            FieldKey::FlipV => FieldValue::Flag(t.flip_v),
            FieldKey::Opacity => FieldValue::Number(self.opacity),
            FieldKey::Visible => FieldValue::Flag(self.visible),
            FieldKey::Locked => FieldValue::Flag(self.locked),
            FieldKey::Name => FieldValue::Text(self.name.clone()),
        }
    }

    /// Writes one scalar field. Range rules are NOT checked here; `Document::validate` runs at
    /// every transaction commit.
    ///
    /// # Errors
    /// Returns a message when the value's type does not match the key.
    pub fn set_field(&mut self, key: FieldKey, value: FieldValue) -> Result<(), String> {
        let t = &mut self.transform;
        match (key, value) {
            (FieldKey::X, FieldValue::Number(v)) => t.x = v,
            (FieldKey::Y, FieldValue::Number(v)) => t.y = v,
            (FieldKey::W, FieldValue::Number(v)) => t.w = v,
            (FieldKey::H, FieldValue::Number(v)) => t.h = v,
            (FieldKey::Rotation, FieldValue::Number(v)) => t.rotation_deg = v,
            (FieldKey::FlipH, FieldValue::Flag(v)) => t.flip_h = v,
            (FieldKey::FlipV, FieldValue::Flag(v)) => t.flip_v = v,
            (FieldKey::Opacity, FieldValue::Number(v)) => self.opacity = v,
            (FieldKey::Visible, FieldValue::Flag(v)) => self.visible = v,
            (FieldKey::Locked, FieldValue::Flag(v)) => self.locked = v,
            (FieldKey::Name, FieldValue::Text(v)) => self.name = v,
            (key, value) => return Err(format!("{key:?} does not accept {value:?}")),
        }
        Ok(())
    }

    /// Estimated in-memory size in bytes, used by the undo byte budget (ADR-004 #9). Estimate,
    /// not exact: it counts fixed overhead plus the variable-length parts.
    #[must_use]
    pub fn approx_bytes(&self) -> usize {
        let vertices: usize = self
            .path
            .as_ref()
            .map_or(0, |p| p.subpaths.iter().map(|s| s.vertices.len()).sum());
        256 + self.name.len() + 16 * self.children.len() + 64 * vertices
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn only_documented_kinds_are_containers() {
        assert!(NodeKind::Frame.is_container());
        assert!(NodeKind::Group.is_container());
        assert!(!NodeKind::Rectangle.is_container());
        assert!(!NodeKind::VectorPath.is_container());
        assert!(!NodeKind::Instance.is_container());
    }

    #[test]
    fn kinds_use_kebab_case_file_names() {
        assert_eq!(
            serde_json::to_string(&NodeKind::RoundedRect).unwrap(),
            "\"rounded-rect\""
        );
        assert_eq!(
            serde_json::to_string(&NodeKind::MaskContainer).unwrap(),
            "\"mask-container\""
        );
    }
}
