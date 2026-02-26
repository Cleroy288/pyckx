//! Admin dashboard with usage statistics

use crate::api;
use crate::components::top_bar::HomeTopBar;
use crate::components::ui::hero_banner::HeroBanner;
use crate::components::ui::loading_boundary::LoadingBoundary;
use crate::components::ui::page_layout::PageLayout;
use crate::domain::admin_types::AdminStatsResponse;
use crate::state::hooks::{use_fetch_on_mount, FetchFn};
use crate::state::use_auth;
use dioxus::prelude::*;

/// Admin statistics page
pub fn AdminPage() -> Element {
    let auth = use_auth();
    let nav = navigator();

    // Auth guard — admin only
    use_effect(move || {
        if !(auth.is_checking_session)() {
            let is_admin = (auth.user)()
                .as_ref()
                .is_some_and(|u| u.role == "admin");
            if !is_admin {
                nav.push("/home");
            }
        }
    });

    let loading = use_signal(|| true);
    let stats: Signal<Option<AdminStatsResponse>> =
        use_signal(|| None);

    let fetch: FetchFn<Option<AdminStatsResponse>> =
        Box::new(|| {
            Box::pin(async {
                api::admin::fetch_admin_stats()
                    .await
                    .map(Some)
            })
        });
    use_fetch_on_mount(stats, loading, fetch);

    rsx! {
        HomeTopBar {}
        PageLayout {
            HeroBanner {
                title: "Admin Dashboard",
                span {}
            }
            LoadingBoundary {
                loading: loading,
                if let Some(s) = (stats)() {
                    // Stat cards
                    div {
                        class: "grid grid-cols-[repeat(auto-fill,minmax(200px,1fr))] \
                            gap-4",
                        div {
                            class: "flex flex-col gap-2 p-5 \
                                border border-[var(--color-border)] \
                                bg-[var(--glass-bg)] \
                                backdrop-blur-[20px]",
                            span {
                                class: "text-[0.85rem] \
                                    text-[var(--color-text-secondary)]",
                                "Total Users"
                            }
                            span {
                                class: "text-[1.75rem] font-bold \
                                    text-[var(--color-text-primary)]",
                                "{s.total_users}"
                            }
                        }
                        div {
                            class: "flex flex-col gap-2 p-5 \
                                border border-[var(--color-border)] \
                                bg-[var(--glass-bg)] \
                                backdrop-blur-[20px]",
                            span {
                                class: "text-[0.85rem] \
                                    text-[var(--color-text-secondary)]",
                                "Total Generations"
                            }
                            span {
                                class: "text-[1.75rem] font-bold \
                                    text-[var(--color-text-primary)]",
                                "{s.total_generations}"
                            }
                        }
                    }
                    // Feature usage
                    h2 {
                        class: "text-[1.1rem] font-semibold \
                            text-[var(--color-text-primary)] \
                            mt-2 mb-0",
                        "Feature Usage"
                    }
                    div {
                        class: "flex flex-col gap-1",
                        for feat in s.features.iter() {
                            div {
                                class: "flex justify-between \
                                    items-center px-4 py-3 \
                                    border border-[var(--color-border)] \
                                    bg-[var(--glass-bg)] \
                                    backdrop-blur-[20px] \
                                    text-[0.9rem] \
                                    text-[var(--color-text-secondary)]",
                                span { "{feat.feature}" }
                                span { "{feat.count}" }
                            }
                        }
                    }
                    // Model usage
                    h2 {
                        class: "text-[1.1rem] font-semibold \
                            text-[var(--color-text-primary)] \
                            mt-2 mb-0",
                        "Model Usage"
                    }
                    div {
                        class: "flex flex-col gap-1",
                        for model in s.models.iter() {
                            div {
                                class: "flex justify-between \
                                    items-center px-4 py-3 \
                                    border border-[var(--color-border)] \
                                    bg-[var(--glass-bg)] \
                                    backdrop-blur-[20px] \
                                    text-[0.9rem] \
                                    text-[var(--color-text-secondary)]",
                                span { "{model.model}" }
                                span {
                                    "{model.count} calls, \
                                        {model.total_tokens} tokens"
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
