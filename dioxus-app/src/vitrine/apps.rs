//! Apps showcase section for the landing page.

use dioxus::prelude::*;

/// Showcases the different learning apps available.
#[component]
pub fn AppsSection() -> Element {
    rsx! {
        section { class: "vitrine-apps",
            h2 { class: "vitrine-apps-title", "Learning apps" }
            p { class: "vitrine-apps-desc",
                "Turn your notes into quizzes, flashcards, \
                 and more."
            }
        }
    }
}
