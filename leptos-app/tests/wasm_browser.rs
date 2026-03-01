//! Browser-only WASM tests — need real DOM/localStorage
//!
//! Run with: wasm-pack test --headless --chrome
//! Requires: Chrome + matching ChromeDriver version

use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

// -- palette_dom localStorage tests --

#[wasm_bindgen_test]
fn test_write_and_read_palette_id() {
    use leptos_app::state::palette_dom::*;

    write_palette_id("ocean-blue");
    let id = read_palette_id();
    assert_eq!(id, Some("ocean-blue".to_string()));
}

#[wasm_bindgen_test]
fn test_write_and_read_is_dark_true() {
    use leptos_app::state::palette_dom::*;

    write_is_dark(true);
    assert_eq!(read_is_dark(), Some(true));
}

#[wasm_bindgen_test]
fn test_write_and_read_is_dark_false() {
    use leptos_app::state::palette_dom::*;

    write_is_dark(false);
    assert_eq!(read_is_dark(), Some(false));
}

#[wasm_bindgen_test]
fn test_apply_palette_to_dom_sets_css_vars() {
    use leptos_app::domain::palette_types::*;
    use leptos_app::state::palette_dom::*;

    let p = default_palette();
    apply_palette_to_dom(&p.colors.light, false);

    // Verify a CSS variable was set
    let doc = web_sys::window()
        .unwrap()
        .document()
        .unwrap();
    let root = doc.document_element().unwrap();
    let style = web_sys::window()
        .unwrap()
        .get_computed_style(&root)
        .unwrap()
        .unwrap();
    let val = style
        .get_property_value("--primary")
        .unwrap();
    assert!(
        !val.is_empty(),
        "--primary should be set after apply",
    );
}

// -- Toast show/success/error (needs browser) --

#[wasm_bindgen_test]
fn test_toast_show_in_browser() {
    use leptos_app::components::ui::toast::*;

    let state = ToastState::new_for_test();
    state.show("Hi".into(), ToastVariant::Info);
    let toasts = state.toasts_for_test();
    assert_eq!(toasts.len(), 1);
    assert_eq!(toasts[0].message, "Hi");
}

#[wasm_bindgen_test]
fn test_toast_success_in_browser() {
    use leptos_app::components::ui::toast::*;

    let state = ToastState::new_for_test();
    state.success("done".into());
    let t = &state.toasts_for_test()[0];
    assert_eq!(t.variant, ToastVariant::Success);
}

#[wasm_bindgen_test]
fn test_toast_error_in_browser() {
    use leptos_app::components::ui::toast::*;

    let state = ToastState::new_for_test();
    state.error("oops".into());
    let t = &state.toasts_for_test()[0];
    assert_eq!(t.variant, ToastVariant::Error);
}
