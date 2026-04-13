//! Collection page — DVD management with modals

use crate::api;
use crate::components::collection::dvd_form::{
    DvdForm, DvdFormData,
};
use crate::components::collection::dvd_list::DvdList;
use crate::components::top_bar::HomeTopBar;
use crate::components::ui::button::Button;
use crate::components::ui::hero_banner::HeroBanner;
use crate::components::ui::modal::Modal;
use crate::components::ui::page_layout::PageLayout;
use crate::domain::collection_types::{
    Dvd, UpdateDvdRequest,
};
use crate::state::hooks::{use_fetch_on_mount, FetchFn};
use crate::state::use_auth_guard;
use dioxus::prelude::*;

/// Collection page with DVD CRUD
pub fn CollectionPage() -> Element {
    let _auth = use_auth_guard();
    let nav = navigator();

    let mut dvds: Signal<Vec<Dvd>> =
        use_signal(Vec::new);
    let loading = use_signal(|| true);
    let mut edit_id: Signal<Option<String>> =
        use_signal(|| None);
    let mut form_loading = use_signal(|| false);

    let fetch: FetchFn<Vec<Dvd>> = Box::new(|| {
        Box::pin(api::collection::get_dvds())
    });
    use_fetch_on_mount(dvds, loading, fetch);

    // Edit handler
    let on_edit = move |data: DvdFormData| {
        let Some(id) = (edit_id)() else { return };
        form_loading.set(true);
        spawn(async move {
            let req = UpdateDvdRequest {
                name: Some(data.name),
                year: Some(data.year),
                realisator: data.director,
                actors: Some(data.actors),
                genre: data.genre,
            };
            match api::collection::update_dvd(
                &id, &req,
            )
            .await
            {
                Ok(updated) => {
                    let idx = (dvds)()
                        .iter()
                        .position(|d| d.id == id);
                    if let Some(i) = idx {
                        dvds.write()[i] = updated;
                    }
                    edit_id.set(None);
                }
                Err(e) => {
                    crate::api::log::log_error(
                        &format!(
                            "DVD update failed: {e}"
                        ),
                        "collection",
                    );
                }
            }
            form_loading.set(false);
        });
    };

    // Delete handler
    let on_delete = move |id: String| {
        spawn(async move {
            if let Ok(true) = api::collection::delete_dvd(&id)
                .await {
                dvds.write()
                    .retain(|d| d.id != id);
            }
        });
    };

    // Open edit modal
    let open_edit = move |id: String| {
        edit_id.set(Some(id));
    };

    // Derive editing DVD
    let editing_dvd = use_memo(move || {
        let eid = (edit_id)()?;
        (dvds)().iter().find(|d| d.id == eid).cloned()
    });

    rsx! {
        div { class: "home-page",
        HomeTopBar {}
        PageLayout {
            HeroBanner {
                title: "My DVDs",
                div {
                    class: "absolute bottom-3 left-4 z-[1] \
                        flex gap-2",
                    Button {
                        text: "Add DVD",
                        on_click: move |_| {
                            nav.push("/collection/add");
                        },
                    }
                }
            }
            DvdList {
                dvds: dvds,
                loading: loading,
                on_edit: open_edit,
                on_delete: on_delete,
            }
        }
        }
        // Edit modal
        if (edit_id)().is_some() {
            Modal {
                open: Signal::new(true),
                title: "Edit DVD",
                on_close: move |_| {
                    edit_id.set(None);
                },
                if let Some(dvd) = (editing_dvd)() {
                    DvdForm {
                        dvd: dvd,
                        on_submit: on_edit,
                        loading: form_loading,
                    }
                }
            }
        }
    }
}
