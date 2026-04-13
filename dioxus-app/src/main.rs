#![allow(non_snake_case)]
#![allow(clippy::module_inception)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::type_complexity)]
#![allow(clippy::excessive_nesting)]
//! Entry point — App root and main

use dioxus::prelude::*;

use dioxus_app::components;
use dioxus_app::routes::Route;
use dioxus_app::state;

/// Grid-line background wrapper
#[component]
fn Background(children: Element) -> Element {
    rsx! {
        div {
            class: "relative w-full min-h-screen \
                bg-[var(--color-background)]",
            div {
                class: "pointer-events-none fixed \
                    inset-0 z-0",
                style: "background-image: \
                    var(--grid-overlay-image); \
                    background-size: \
                    var(--grid-overlay-size); \
                    opacity: 0.12;",
            }
            {children}
        }
    }
}

/// JS snippet that polls location.pathname every 500ms
/// and saves it to localStorage for post-login redirect.
/// Only saves authenticated app routes.
const TRACK_PAGE_JS: &str = r#"
setInterval(function() {
    var p = location.pathname;
    var prefixes = [
        '/home', '/courses', '/collection',
        '/admin', '/qcm', '/flashcards',
        '/true-false', '/open-questions',
        '/keywords', '/order-phrases',
        '/fill-blanks'
    ];
    for (var i = 0; i < prefixes.length; i++) {
        if (p.startsWith(prefixes[i])) {
            localStorage.setItem(
                'pyckx-last-page', p
            );
            break;
        }
    }
}, 500);
"#;

/// Root component with providers and router
fn App() -> Element {
    // Track current page via JS interval — works
    // regardless of Dioxus SPA routing internals.
    use_effect(move || {
        let _ = js_sys::eval(TRACK_PAGE_JS);
    });
    rsx! {
        components::ui::toast::ToastProvider {
            state::PaletteProvider {
                state::AuthProvider {
                    state::UserAppsProvider {
                        state::CodingExerciseProvider {
                            Background {
                                Router::<Route> {}
                            }
                        }
                    }
                }
            }
        }
    }
}

/// Application entry point
fn main() {
    console_error_panic_hook::set_once();
    dioxus::launch(App);
}
