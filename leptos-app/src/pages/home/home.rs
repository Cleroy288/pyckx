//! Home Page — Dashboard with dynamic app sections

use crate::components::dashboard::{
    AppStore, MyApps, PalettePicker, WelcomeHeader,
};
use crate::components::top_bar::HomeTopBar;
use crate::components::ui::page_layout::PageLayout;
use crate::components::ui::section_divider::SectionDivider;
use crate::state::use_auth_guard;
use leptos::prelude::*;

stylance::import_crate_style!(
    style,
    "src/pages/home/home.module.css"
);

#[component]
pub fn HomePage() -> impl IntoView {
    let auth = use_auth_guard();

    let username = Signal::derive(move || {
        auth.user.with(|u| {
            u.as_ref()
                .map(|a| a.username.clone())
                .unwrap_or_default()
        })
    });

    view! {
        <>
            <HomeTopBar />
            <PageLayout>
                <Show
                    when=move || {
                        !auth.is_checking_session.get()
                    }
                    fallback=|| view! {
                        <div class=style::loading>
                            "Checking session..."
                        </div>
                    }
                >
                    <Show
                        when=move || {
                            auth.user
                                .with(|u| u.is_some())
                        }
                        fallback=|| view! {
                            <div class=style::loading>
                                "Redirecting to login..."
                            </div>
                        }
                    >
                        <WelcomeHeader
                            username=username
                        />
                        <div
                            class=style::section
                            style="--delay: 0.1s"
                        >
                            <SectionDivider
                                label="My Apps"
                            />
                            <div class=style::section_body>
                                <MyApps />
                            </div>
                        </div>
                        <div
                            class=style::section
                            style="--delay: 0.25s"
                        >
                            <SectionDivider
                                label="Pyckx Apps"
                            />
                            <div class=style::section_body>
                                <AppStore />
                            </div>
                        </div>
                        <div
                            class=style::section
                            style="--delay: 0.4s"
                        >
                            <SectionDivider
                                label="Theme"
                            />
                            <div class=style::section_body>
                                <PalettePicker />
                            </div>
                        </div>
                    </Show>
                </Show>
            </PageLayout>
        </>
    }
}
