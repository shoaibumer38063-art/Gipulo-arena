//! ADR-009 integrity rules: documentHash (SHA-256 of the document section) and NFC-only strings.

use gip_document::file::{GipFile, from_json, to_json};

fn saved_text() -> String {
    to_json(&GipFile::new("Integrity", 7)).expect("new file saves")
}

#[test]
fn save_writes_a_64_hex_document_hash() {
    let text = saved_text();
    let line = text
        .lines()
        .find(|l| l.contains("\"documentHash\""))
        .expect("documentHash field present");
    let hash = line.split('"').nth(3).expect("hash value");
    assert_eq!(hash.len(), 64, "SHA-256 hex is 64 digits");
    assert!(
        hash.bytes()
            .all(|b| b.is_ascii_hexdigit() && !b.is_ascii_uppercase())
    );
    assert!(!text.contains("\"documentHash\": null"));
}

#[test]
fn save_does_not_mutate_the_caller_value() {
    let file = GipFile::new("Integrity", 7);
    let _ = to_json(&file).expect("saves");
    assert_eq!(
        file.meta().document_hash,
        None,
        "caller's GipFile stays unstamped"
    );
}

#[test]
fn round_trip_keeps_the_stamped_hash_and_content() {
    let file = GipFile::new("Integrity", 7);
    let text = to_json(&file).expect("saves");
    let loaded = from_json(&text).expect("loads");
    assert!(loaded.meta().document_hash.is_some());
    assert_eq!(
        to_json(&loaded).expect("re-saves"),
        text,
        "stable: save(load(save(x))) == save(x)"
    );
}

#[test]
fn null_hash_from_older_files_still_loads() {
    let text = saved_text().replace(
        &format!("\"documentHash\": \"{}\"", hash_of(&saved_text())),
        "\"documentHash\": null",
    );
    assert!(from_json(&text).is_ok());
}

#[test]
fn tampered_document_is_detected_on_load() {
    let text = saved_text();
    let tampered = text.replacen("\"Page 1\"", "\"Page X\"", 1);
    assert_ne!(
        tampered, text,
        "the page name exists in the document section"
    );
    let err = from_json(&tampered).expect_err("hash mismatch must be rejected");
    assert!(
        err.to_string().contains("document hash mismatch"),
        "got: {err}"
    );
}

#[test]
fn non_nfc_string_is_rejected_on_save_and_load() {
    // "e" + combining acute (NFD) is not NFC; "é" (U+00E9) is.
    let decomposed = "Cafe\u{301}";
    let file = GipFile::new(decomposed, 7);
    let err = to_json(&file).expect_err("NFD name must be rejected on save");
    assert!(err.to_string().contains("NFC"), "got: {err}");

    let nfc_text = saved_text().replacen("\"Integrity\"", "\"Cafe\u{301}\"", 1);
    let err = from_json(&nfc_text).expect_err("NFD name must be rejected on load");
    assert!(err.to_string().contains("NFC"), "got: {err}");
}

#[test]
fn nfc_composed_text_is_accepted() {
    let file = GipFile::new("Caf\u{e9}", 7);
    assert!(to_json(&file).is_ok());
}

fn hash_of(text: &str) -> String {
    let line = text
        .lines()
        .find(|l| l.contains("\"documentHash\""))
        .expect("field");
    line.split('"').nth(3).expect("value").to_owned()
}
