//! Keyword chip rendering for the player.

use crate::keywords::types::KeywordQuestion;
use dioxus::prelude::*;

const CHIP: &str = "px-5 py-2.5 text-sm font-medium \
     bg-[var(--primary-tint-subtle)] \
     border border-[var(--color-border)] \
     text-[var(--color-text-primary)] cursor-pointer \
     transition-all hover:border-[var(--color-primary)] \
     hover:bg-[var(--primary-tint-light)] \
     disabled:cursor-default disabled:opacity-70";

const CHIP_SELECTED: &str =
    "!bg-[var(--primary-tint-medium)] \
     !border-[var(--color-primary)] \
     !text-[var(--color-primary)]";

/// Render every keyword chip for the current question.
pub fn render_chips(
    questions: &[KeywordQuestion],
    current: Signal<usize>,
    selected: Signal<Vec<String>>,
    toggle: impl Fn(String) + Copy + 'static,
) -> Element {
    let Some(q) = questions.get((current)()) else {
        return rsx! {};
    };
    rsx! {
        for k in q.keywords.iter() {
            div { key: "{k.id}",
                {render_chip(k.id.clone(), k.word.clone(), selected, toggle)}
            }
        }
    }
}

/// Render a single keyword chip button.
fn render_chip(
    id: String,
    word: String,
    selected: Signal<Vec<String>>,
    toggle: impl Fn(String) + Copy + 'static,
) -> Element {
    let id_for_class = id.clone();
    let class = use_memo(move || {
        chip_class((selected)().contains(&id_for_class))
    });
    rsx! {
        button {
            class: "{class}",
            onclick: move |_| toggle(id.clone()),
            "{word}"
        }
    }
}

/// Return the chip class string for the given state.
fn chip_class(selected: bool) -> String {
    if selected {
        format!("{CHIP} {CHIP_SELECTED}")
    } else {
        CHIP.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_chip_class_when_selected_includes_marker() {
        let class = chip_class(true);
        assert!(class.contains(CHIP_SELECTED));
    }

    #[test]
    fn test_chip_class_when_idle_omits_marker() {
        let class = chip_class(false);
        assert!(!class.contains(CHIP_SELECTED));
    }

    #[test]
    fn test_chip_class_idle_only_has_base() {
        let class = chip_class(false);
        assert!(class.contains("cursor-pointer"));
    }
}
