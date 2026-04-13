//! Admin dashboard page — usage statistics for admins only.

use crate::admin::api::{
    self, AdminStats, FeatureStats, ModelStats,
};
use crate::auth::use_auth;
use crate::ui::{HeroBanner, LoadingBoundary, PageLayout};
use dioxus::prelude::*;

const ADMIN_ROLE: &str = "admin";
const FALLBACK_ROUTE: &str = "/home";

/// Admin statistics page — protected route.
#[component]
pub fn AdminPage() -> Element {
    use_admin_guard();
    let stats = use_signal(|| None::<AdminStats>);
    let loading = use_signal(|| true);
    load_stats(stats, loading);
    rsx! {
        PageLayout {
            HeroBanner {
                title: "Admin Dashboard",
                span {}
            }
            LoadingBoundary { loading,
                if let Some(data) = (stats)() {
                    Dashboard { stats: data }
                }
            }
        }
    }
}

/// Redirect non-admin users to the home route.
fn use_admin_guard() {
    let auth = use_auth();
    let nav = navigator();
    use_effect(move || {
        if (auth.is_checking_session)() {
            return;
        }
        let is_admin = (auth.user)()
            .as_ref()
            .is_some_and(|u| u.role == ADMIN_ROLE);
        if !is_admin {
            nav.push(FALLBACK_ROUTE);
        }
    });
}

/// Fetch stats once on mount, toggle `loading` off when done.
fn load_stats(
    mut stats: Signal<Option<AdminStats>>,
    mut loading: Signal<bool>,
) {
    use_effect(move || {
        spawn(async move {
            if let Ok(data) = api::fetch_stats().await {
                stats.set(Some(data));
            }
            loading.set(false);
        });
    });
}

/// Renders the full dashboard content once stats are loaded.
#[component]
fn Dashboard(stats: AdminStats) -> Element {
    rsx! {
        div { class: GRID_CLS,
            StatCard { label: "Total Users",
                value: stats.total_users.to_string() }
            StatCard { label: "Total Generations",
                value: stats.total_generations.to_string() }
        }
        h2 { class: H2_CLS, "Feature Usage" }
        div { class: "flex flex-col gap-1",
            for feat in stats.features.iter() {
                UsageRow {
                    key: "{feat.feature}",
                    label: feat.feature.clone(),
                    value: feat.count.to_string(),
                }
            }
        }
        h2 { class: H2_CLS, "Model Usage" }
        div { class: "flex flex-col gap-1",
            for model in stats.models.iter() {
                UsageRow {
                    key: "{model.model}",
                    label: model.model.clone(),
                    value: api::format_model_line(model),
                }
            }
        }
    }
}

/// Large metric card — a label above a bold value.
#[component]
fn StatCard(label: String, value: String) -> Element {
    rsx! {
        div { class: CARD_CLS,
            span { class: CARD_LABEL_CLS, "{label}" }
            span { class: CARD_VALUE_CLS, "{value}" }
        }
    }
}

/// One row in a usage list — label on the left, value on the right.
#[component]
fn UsageRow(label: String, value: String) -> Element {
    rsx! {
        div { class: ROW_CLS,
            span { "{label}" }
            span { "{value}" }
        }
    }
}

const GRID_CLS: &str = "\
    grid grid-cols-[repeat(auto-fill,minmax(200px,1fr))] gap-4";

const H2_CLS: &str = "\
    text-[1.1rem] font-semibold \
    text-[var(--color-text-primary)] mt-2 mb-0";

const CARD_CLS: &str = "\
    flex flex-col gap-2 p-5 \
    border-2 border-[var(--color-border)] \
    bg-[var(--card)] rounded-[20px]";

const CARD_LABEL_CLS: &str = "\
    text-[0.85rem] text-[var(--color-text-secondary)]";

const CARD_VALUE_CLS: &str = "\
    text-[1.75rem] font-bold \
    text-[var(--color-text-primary)]";

const ROW_CLS: &str = "\
    flex justify-between items-center px-4 py-3 \
    border border-[var(--color-border)] \
    bg-[var(--glass-bg)] backdrop-blur-[20px] \
    text-[0.9rem] text-[var(--color-text-secondary)]";

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_admin_role_constant() {
        assert_eq!(ADMIN_ROLE, "admin");
    }

    #[test]
    fn test_fallback_route_constant() {
        assert_eq!(FALLBACK_ROUTE, "/home");
    }
}
