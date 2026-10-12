//! Deterministic performance fixtures (`fixtures/`, rules/canvas.md §26). The generator is the
//! source of truth; the committed `.gip` file is its golden output and is checked by a test.
//! Regenerate with `UPDATE_FIXTURES=1 cargo test -p gip-session fixture`.

use gip_document::{GipFile, Node, NodeKind, Transform};
use gip_history::{History, IntentLabel, Parent};

/// Fixture document salt (ADR-003 id space). Fixed so the output is byte-stable.
const FIXTURE_SALT: u32 = 2;

/// Object count of the `rects-1000` fixture (AC-24).
pub const RECTS_1000: usize = 1000;

/// Builds a page of `count` rectangles on a 40-column grid, as validated `.gip` text.
pub fn rect_grid_json(name: &str, count: usize) -> Result<String, String> {
    let file = GipFile::new(name, FIXTURE_SALT);
    let mut history = History::new(file).map_err(|e| e.to_string())?;
    history
        .transact(IntentLabel::new("generate-fixture"), |tx| {
            let page_id = tx.document().pages().first().ok_or("no page")?.id;
            for i in 0..count {
                let id = tx.alloc_id()?;
                let transform = Transform {
                    x: 40.0 + (i % 40) as f64 * 60.0,
                    y: 40.0 + (i / 40) as f64 * 60.0,
                    w: 48.0,
                    h: 36.0,
                    rotation_deg: 0.0,
                    flip_h: false,
                    flip_v: false,
                };
                let node = Node::new(NodeKind::Rectangle, format!("Rect {}", i + 1), transform);
                tx.insert(Parent::Page(page_id), i, vec![(id, node)])?;
            }
            Ok(())
        })
        .map_err(|e| e.to_string())?;
    gip_document::to_json(history.file()).map_err(|e| e.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Session;

    fn fixture_path() -> std::path::PathBuf {
        std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../../fixtures/rects-1000.gip")
    }

    #[test]
    fn committed_fixture_matches_the_generator() {
        let generated = rect_grid_json("rects-1000", RECTS_1000).expect("generator");
        if std::env::var_os("UPDATE_FIXTURES").is_some() {
            std::fs::write(fixture_path(), &generated).expect("write fixture");
        }
        let committed = std::fs::read_to_string(fixture_path()).expect("fixture file exists");
        assert_eq!(committed, generated, "regenerate with UPDATE_FIXTURES=1");
    }

    #[test]
    fn fixture_opens_with_all_objects_and_saves_stably() {
        let text = rect_grid_json("rects-1000", RECTS_1000).expect("generator");
        let mut s = Session::new();
        s.open_json(&text).expect("fixture opens");
        assert_eq!(s.object_count(), RECTS_1000);
        assert_eq!(
            s.save_json().expect("saves"),
            text,
            "open → save is byte-stable"
        );
        assert!(!s.can_undo(), "opening is not an edit");
    }
}
