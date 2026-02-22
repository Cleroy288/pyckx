// ** admin.rs **
// ==> Admin dashboard with usage statistics

use crate::api;
use crate::components::top_bar::HomeTopBar;
use crate::components::ui::hero_banner::HeroBanner;
use crate::components::ui::page_layout::PageLayout;
use crate::components::ui::loading_boundary::LoadingBoundary;
use crate::components::ui::toast::use_toast;
use crate::domain::admin_types::AdminStatsResponse;
use crate::state::hooks::{use_fetch_on_mount, FetchFn};
use crate::state::use_auth;
use leptos::prelude::*;
use leptos_router::hooks::use_navigate;

stylance::import_crate_style!(
    style,
    "src/pages/admin/admin.module.css"
);

/// Admin statistics page
#[component]
pub fn AdminPage() -> impl IntoView {
    let auth = use_auth();
    let toast = use_toast();
    let navigate = use_navigate();

    // Auth guard — admin only
    Effect::new(move |_| {
        if !auth.is_checking_session.get() {
            let is_admin = auth
                .user
                .get()
                .as_ref()
                .is_some_and(|u| u.role == "admin");
            if !is_admin {
                navigate("/home", Default::default());
            }
        }
    });

    let loading = RwSignal::new(true);
    let stats: RwSignal<Option<AdminStatsResponse>> =
        RwSignal::new(None);

    let fetch: FetchFn<Option<AdminStatsResponse>> =
        Box::new(|| {
            Box::pin(async {
                api::admin::fetch_admin_stats()
                    .await
                    .map(Some)
            })
        });
    use_fetch_on_mount(stats, loading, toast, fetch);

    view! {
        <HomeTopBar />
        <PageLayout>
            <HeroBanner title="Admin Dashboard">
                <span />
            </HeroBanner>
            <LoadingBoundary loading=loading>
                {move || {
                    stats.get().map(|s| {
                        view! {
                            <div class=style::cards>
                                <div
                                    class=style::stat_card
                                >
                                    <span
                                        class=style::stat_label
                                    >
                                        "Total Users"
                                    </span>
                                    <span
                                        class=style::stat_value
                                    >
                                        {s.total_users}
                                    </span>
                                </div>
                                <div
                                    class=style::stat_card
                                >
                                    <span
                                        class=style::stat_label
                                    >
                                        "Total Generations"
                                    </span>
                                    <span
                                        class=style::stat_value
                                    >
                                        {s.total_generations}
                                    </span>
                                </div>
                            </div>
                            <h2 class=style::section>
                                "Feature Usage"
                            </h2>
                            <div class=style::table>
                                <For
                                    each=move || {
                                        s.features.clone()
                                    }
                                    key=|f| {
                                        f.feature.clone()
                                    }
                                    let:feat
                                >
                                    <div
                                        class=style::row
                                    >
                                        <span>
                                            {feat.feature}
                                        </span>
                                        <span>
                                            {feat.count}
                                        </span>
                                    </div>
                                </For>
                            </div>
                            <h2 class=style::section>
                                "Model Usage"
                            </h2>
                            <div class=style::table>
                                <For
                                    each=move || {
                                        s.models.clone()
                                    }
                                    key=|m| {
                                        m.model.clone()
                                    }
                                    let:model
                                >
                                    <div
                                        class=style::row
                                    >
                                        <span>
                                            {model.model}
                                        </span>
                                        <span>
                                            {model.count}
                                            " calls, "
                                            {model
                                                .total_tokens}
                                            " tokens"
                                        </span>
                                    </div>
                                </For>
                            </div>
                        }
                    })
                }}
            </LoadingBoundary>
        </PageLayout>
    }
}
