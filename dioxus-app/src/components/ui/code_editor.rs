//! CodeEditor — CodeMirror 6 wrapper for Dioxus
//!
//! Desktop-only code editor with syntax highlighting.
//! Respects the app's light/dark mode via CSS variables.

use crate::state::use_palette;
use dioxus::prelude::*;

/// Supported languages for the code editor
#[derive(Clone, PartialEq)]
pub enum CodeLanguage {
    Python,
    JavaScript,
    Go,
    Rust,
}

impl CodeLanguage {
    /// JS bridge language identifier
    fn as_str(&self) -> &'static str {
        match self {
            Self::Python => "python",
            Self::JavaScript => "javascript",
            Self::Go => "go",
            Self::Rust => "rust",
        }
    }

    /// Parse from backend language string
    pub fn from_lang_str(s: &str) -> Self {
        match s {
            "python" => Self::Python,
            "javascript" => Self::JavaScript,
            "go" => Self::Go,
            "rust" => Self::Rust,
            _ => Self::Python,
        }
    }
}

/// Code editor powered by CodeMirror 6
#[component]
pub fn CodeEditor(
    language: CodeLanguage,
    initial_code: String,
    editor_id: String,
) -> Element {
    let palette = use_palette();
    let id = editor_id.clone();

    // Init editor with current theme
    use_effect(move || {
        let id = id.clone();
        let lang = language.as_str();
        let dark = palette.is_dark();
        let code = initial_code
            .replace('\\', "\\\\")
            .replace('`', "\\`");

        document::eval(&format!(
            "cmInit('{id}', '{lang}', `{code}`, {dark})"
        ));
    });

    // Switch theme when dark mode toggles
    let theme_id = editor_id.clone();
    use_effect(move || {
        let dark = palette.is_dark();
        document::eval(&format!(
            "cmSetTheme('{theme_id}', {dark})"
        ));
    });

    // Cleanup on drop
    let cleanup_id = editor_id.clone();
    use_drop(move || {
        document::eval(&format!(
            "cmDestroy('{cleanup_id}')"
        ));
    });

    rsx! {
        div {
            id: "{editor_id}",
            class: "w-full rounded-lg overflow-hidden \
                border border-[var(--glass-border)]",
            style: "min-height: 300px;",
        }
    }
}

/// Get the current code from an editor instance
pub async fn get_editor_value(
    editor_id: &str,
) -> String {
    let mut result = document::eval(&format!(
        "dioxus.send(cmGetValue('{editor_id}'))"
    ));
    result
        .recv::<String>()
        .await
        .unwrap_or_default()
}
