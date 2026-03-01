//! Full-page DVD creation form

use crate::api;
use crate::components::collection::dvd_form::{
    DvdForm, DvdFormData,
};
use crate::components::top_bar::HomeTopBar;
use crate::components::ui::hero_banner::HeroBanner;
use crate::components::ui::page_layout::PageLayout;
use crate::domain::collection_types::AddDvdRequest;
use crate::state::use_auth_guard;
use dioxus::prelude::*;

/// Add DVD page
pub fn DvdAddPage() -> Element {
    let _auth = use_auth_guard();
    let nav = navigator();
    let mut loading = use_signal(|| false);

    let on_submit = move |data: DvdFormData| {
        loading.set(true);
        spawn(async move {
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
                    nav.push("/collection");
                }
                Err(_e) => {}
            }
            loading.set(false);
        });
    };

    rsx! {
        HomeTopBar {}
        PageLayout {
            HeroBanner {
                title: "Add DVD",
                span {}
            }
            DvdForm {
                on_submit: on_submit,
                loading: loading,
            }
        }
    }
}
