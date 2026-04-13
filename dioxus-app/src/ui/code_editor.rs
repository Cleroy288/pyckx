//! CodeEditor — thin Dioxus wrapper around CodeMirror 6.
//!
//! Delegates rendering to JS (`cmInit`, `cmSetTheme`,
//! `cmDestroy`, `cmGetValue`) and tracks the app palette.

use crate::state::use_palette;
use dioxus::prelude::*;

/// Supported editor languages.
#[derive(Clone, PartialEq)]
pub enum CodeLanguage {
    Python,
    JavaScript,
    Go,
    Rust,
}

impl CodeLanguage {
    /// JS bridge identifier for this language.
    fn as_str(&self) -> &'static str {
        match self {
            Self::Python => "python",
            Self::JavaScript => "javascript",
            Self::Go => "go",
            Self::Rust => "rust",
        }
    }

    /// Parse a backend language string into a `CodeLanguage`.
    /// Unknown strings fall back to Python.
    pub fn from_lang_str(s: &str) -> Self {
        match s {
            "javascript" => Self::JavaScript,
            "go" => Self::Go,
            "rust" => Self::Rust,
            _ => Self::Python,
        }
    }
}

/// CodeMirror 6 editor bound to a DOM id.
#[component]
pub fn CodeEditor(
    language: CodeLanguage,
    initial_code: String,
    editor_id: String,
) -> Element {
    let palette = use_palette();
    init_editor(&editor_id, &language, &initial_code, palette);
    sync_theme_on_change(editor_id.clone(), palette);
    cleanup_on_drop(editor_id.clone());
    rsx! {
        div {
            id: "{editor_id}",
            class: "w-full rounded-lg overflow-hidden \
                border border-[var(--glass-border)]",
            style: "min-height: 300px;",
        }
    }
}

/// Read the current editor contents. Empty string on error.
pub async fn get_editor_value(editor_id: &str) -> String {
    let mut result = document::eval(&format!(
        "dioxus.send(cmGetValue('{editor_id}'))"
    ));
    result.recv::<String>().await.unwrap_or_default()
}

/// Initialize the editor on mount with theme + code.
fn init_editor(
    id: &str,
    language: &CodeLanguage,
    initial_code: &str,
    palette: crate::state::palette::PaletteState,
) {
    let id = id.to_string();
    let lang = language.as_str();
    let code = escape_backticks(initial_code);
    use_effect(move || {
        let dark = palette.is_dark();
        document::eval(&format!(
            "cmInit('{id}', '{lang}', `{code}`, {dark})"
        ));
    });
}

/// Swap the editor theme when the palette flips dark/light.
fn sync_theme_on_change(
    id: String,
    palette: crate::state::palette::PaletteState,
) {
    use_effect(move || {
        let dark = palette.is_dark();
        document::eval(&format!(
            "cmSetTheme('{id}', {dark})"
        ));
    });
}

/// Tear down the editor when the component drops.
fn cleanup_on_drop(id: String) {
    use_drop(move || {
        document::eval(&format!("cmDestroy('{id}')"));
    });
}

/// Escape characters that would break the JS backtick string.
fn escape_backticks(code: &str) -> String {
    code.replace('\\', "\\\\").replace('`', "\\`")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_lang_str_rust() {
        assert!(matches!(
            CodeLanguage::from_lang_str("rust"),
            CodeLanguage::Rust
        ));
    }

    #[test]
    fn test_from_lang_str_unknown_falls_back_to_python() {
        assert!(matches!(
            CodeLanguage::from_lang_str("ruby"),
            CodeLanguage::Python
        ));
    }

    #[test]
    fn test_as_str_go_returns_go() {
        assert_eq!(CodeLanguage::Go.as_str(), "go");
    }

    #[test]
    fn test_escape_backticks_escapes_backslash() {
        assert_eq!(escape_backticks("a\\b"), "a\\\\b");
    }

    #[test]
    fn test_escape_backticks_escapes_backtick() {
        assert_eq!(escape_backticks("x`y"), "x\\`y");
    }
}
