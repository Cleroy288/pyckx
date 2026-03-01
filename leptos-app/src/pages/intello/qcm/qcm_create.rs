use crate::components::intello::ManualQcmForm;
use crate::components::top_bar::HomeTopBar;
use crate::components::ui::hero_banner::HeroBanner;
use crate::components::ui::page_layout::PageLayout;
use leptos::prelude::*;
use leptos_router::hooks::use_navigate;

/// QCM manual creation page
#[component]
pub fn QcmCreatePage() -> impl IntoView {
    let navigate = use_navigate();
    let on_created = Callback::new(move |_: ()| {
        navigate(
            "/intello/qcm",
            Default::default(),
        );
    });

    view! {
        <HomeTopBar />
        <PageLayout>
            <HeroBanner title="Create QCM Set">
                <span />
            </HeroBanner>
            <ManualQcmForm on_back=on_created />
        </PageLayout>
    }
}
