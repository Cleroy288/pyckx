//! AuthLayout — shared card wrapper for auth pages

use crate::pages::vitrine::VitrineNav;
use dioxus::prelude::*;

/// Card layout for login / register pages
#[component]
pub fn AuthLayout(
    title: &'static str,
    subtitle: &'static str,
    error: ReadSignal<Option<String>>,
    children: Element,
) -> Element {
    rsx! {
        div { class: "vitrine-page",
            VitrineNav {}
            div { class: "auth-wrap",
                div { class: "auth-card",
                    h1 { "{title}" }
                    p { class: "auth-sub",
                        "{subtitle}"
                    }
                    if let Some(err) = error() {
                        div { class: "auth-error",
                            "{err}"
                        }
                    }
                    {children}
                }
            }
        }
    }
}
