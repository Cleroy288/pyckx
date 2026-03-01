// ** dvd_add.rs **
// ==> Full-page DVD creation form

use crate::api;
use crate::components::collection::dvd_form::{
    DvdForm, DvdFormData,
};
use crate::components::top_bar::HomeTopBar;
use crate::components::ui::hero_banner::HeroBanner;
use crate::components::ui::page_layout::PageLayout;
use crate::components::ui::toast::use_toast;
use crate::domain::collection_types::AddDvdRequest;
use crate::state::use_auth_guard;
use leptos::prelude::*;
use leptos::task::spawn_local;
use leptos_router::hooks::use_navigate;

/// Add DVD page
#[component]
pub fn DvdAddPage() -> impl IntoView {
    let _auth = use_auth_guard();
    let toast = use_toast();
    let navigate = use_navigate();
    let loading = RwSignal::new(false);

    let on_submit = move |data: DvdFormData| {
        loading.set(true);
        let nav = navigate.clone();
        spawn_local(async move {
            let req = AddDvdRequest {
                name: data.name,
                year: data.year,
                realisator: data.director,
                actors: data.actors,
                genre: data.genre,
            };
            match api::collection::add_dvd(&req).await
            {
                Ok(_) => {
                    toast.success(
                        "DVD added".into(),
                    );
                    nav(
                        "/collection",
                        Default::default(),
                    );
                }
                Err(e) => toast.error(e),
            }
            loading.set(false);
        });
    };

    view! {
        <HomeTopBar />
        <PageLayout>
            <HeroBanner title="Add DVD">
                <span />
            </HeroBanner>
            <DvdForm
                on_submit=Callback::new(on_submit)
                loading=loading
            />
        </PageLayout>
    }
}
