//! Admin dashboard with usage statistics

use crate::api;
use crate::components::top_bar::HomeTopBar;
use crate::components::ui::hero_banner::HeroBanner;
use crate::components::ui::loading_boundary::LoadingBoundary;
use crate::components::ui::page_layout::PageLayout;
use crate::domain::admin_types::AdminStatsResponse;
use crate::state::hooks::{
    use_admin_guard, use_fetch_on_mount, FetchFn,
};
use dioxus::prelude::*;

/// Admin statistics page
pub fn AdminPage() -> Element {
    let _auth = use_admin_guard();

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
        div { class: "home-page",
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
                                border-2 border-[var(--color-border)] \
                                bg-[var(--card)] \
                                rounded-[20px]",
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
                                border-2 border-[var(--color-border)] \
                                bg-[var(--card)] \
                                rounded-[20px]",
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
                                key: "{feat.feature}",
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
                                key: "{model.model}",
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
}
