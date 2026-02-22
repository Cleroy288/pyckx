//! WASM integration tests — runs in Node.js or browser
//!
//! Run with: wasm-pack test --node
//! Or:       wasm-pack test --headless --chrome

use wasm_bindgen_test::*;

// -- Domain smoke tests (WASM target) --

#[wasm_bindgen_test]
fn test_level_roundtrip_wasm() {
    use leptos_app::domain::intello_types::Level;

    let cases =
        [Level::Easy, Level::Medium, Level::Hard];
    for level in cases {
        assert_eq!(
            Level::parse(level.as_str()),
            level,
        );
    }
}

#[wasm_bindgen_test]
fn test_palette_lookup_wasm() {
    use leptos_app::domain::palette_types::palette_by_id;

    let p = palette_by_id("teal-tech");
    assert!(p.is_some());
    assert_eq!(p.unwrap().name, "Teal Tech");
}

// -- API endpoint constants --

#[wasm_bindgen_test]
fn test_endpoints_defined_wasm() {
    use leptos_app::api::endpoints;

    assert!(!endpoints::LOGIN.is_empty());
    assert!(!endpoints::REGISTER.is_empty());
    assert!(!endpoints::ME.is_empty());
}

// -- Serde roundtrips in WASM --

#[wasm_bindgen_test]
fn test_deserialize_qcm_set_wasm() {
    use leptos_app::domain::qcm_types::QcmSet;

    let json = r#"{
        "id":"1","user_id":"u","name":"Q",
        "description":"D","level":"easy",
        "language":"en","subjects":[],
        "questions":[]
    }"#;
    let s: QcmSet =
        serde_json::from_str(json).unwrap();
    assert_eq!(s.name, "Q");
}

#[wasm_bindgen_test]
fn test_deserialize_content_block_wasm() {
    use leptos_app::domain::course_types::ContentBlock;

    let json =
        r#"{"type":"Text","content":"hello"}"#;
    let b: ContentBlock =
        serde_json::from_str(json).unwrap();
    assert!(matches!(
        b,
        ContentBlock::Text { content }
        if content == "hello"
    ));
}

// NOTE: ToastState.show()/success()/error() tests
// require a browser environment (set_timeout needs
// web_sys::Window). Run with --headless --chrome if
// Chrome + matching ChromeDriver are available.
// State management logic is covered by native tests.
