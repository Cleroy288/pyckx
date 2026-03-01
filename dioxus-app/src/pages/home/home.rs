//! Home Page — Dashboard with dynamic app sections

use crate::components::dashboard::{
    AppStore, MyApps, PalettePicker, WelcomeHeader,
};
use crate::components::top_bar::HomeTopBar;
use crate::components::ui::page_layout::PageLayout;
use crate::components::ui::section_divider::SectionDivider;
use crate::state::use_auth_guard;
use dioxus::prelude::*;

/// Home page — authenticated dashboard
pub fn HomePage() -> Element {
    let auth = use_auth_guard();

    let username: Memo<String> = use_memo(move || {
        (auth.user)()
            .as_ref()
            .map(|u| u.username.clone())
            .unwrap_or_default()
    });

    rsx! {
        HomeTopBar {}
        PageLayout {
            if (auth.is_checking_session)() {
                div {
                    class: "flex justify-center items-center \
                        min-h-[50vh] \
                        text-[var(--color-text-secondary)] \
                        text-lg",
                    "Checking session..."
                }
            } else if (auth.user)().is_some() {
                WelcomeHeader {
                    username: Signal::new(
                        (username)()
                    ),
                }
                // My Apps section
                div {
                    class: "mt-12 animate-[fadeInUp_0.6s_ease_both]",
                    style: "--delay: 0.1s",
                    SectionDivider {
                        label: "My Apps",
                    }
                    div {
                        class: "mt-6",
                        MyApps {}
                    }
                }
                // Pyckx Apps section
                div {
                    class: "mt-12 animate-[fadeInUp_0.6s_ease_both]",
                    style: "--delay: 0.25s",
                    SectionDivider {
                        label: "Pyckx Apps",
                    }
                    div {
                        class: "mt-6",
                        AppStore {}
                    }
                }
                // Theme section
                div {
                    class: "mt-12 animate-[fadeInUp_0.6s_ease_both]",
                    style: "--delay: 0.4s",
                    SectionDivider {
                        label: "Theme",
                    }
                    div {
                        class: "mt-6",
                        PalettePicker {}
                    }
                }
            } else {
                div {
                    class: "flex justify-center items-center \
                        min-h-[50vh] \
                        text-[var(--color-text-secondary)] \
                        text-lg",
                    "Redirecting to login..."
                }
            }
        }
    }
}
