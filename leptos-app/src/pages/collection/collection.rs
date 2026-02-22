// ** collection.rs **
// ==> Collection page — DVD management with modals

use crate::api;
use crate::components::collection::dvd_form::{
    DvdForm, DvdFormData,
};
use crate::components::collection::dvd_list::DvdList;
use crate::components::top_bar::HomeTopBar;
use crate::components::ui::button::{
    nav_click, Button,
};
use crate::components::ui::hero_banner::HeroBanner;
use crate::components::ui::modal::{on_close, Modal};
use crate::components::ui::page_layout::PageLayout;
use crate::components::ui::toast::use_toast;
use crate::domain::collection_types::{
    Dvd, UpdateDvdRequest,
};
use crate::state::hooks::{use_fetch_on_mount, FetchFn};
use crate::state::use_auth_guard;
use leptos::prelude::*;
use leptos::task::spawn_local;

stylance::import_crate_style!(
    style,
    "src/pages/collection/collection.module.css"
);

/// Collection page with DVD CRUD
#[component]
pub fn CollectionPage() -> impl IntoView {
    let _auth = use_auth_guard();
    let toast = use_toast();

    let dvds: RwSignal<Vec<Dvd>> =
        RwSignal::new(Vec::new());
    let loading = RwSignal::new(true);
    let edit_id: RwSignal<Option<String>> =
        RwSignal::new(None);
    let form_loading = RwSignal::new(false);

    let fetch: FetchFn<Vec<Dvd>> = Box::new(|| {
        Box::pin(api::collection::get_dvds())
    });
    use_fetch_on_mount(dvds, loading, toast, fetch);

    let on_edit = move |data: DvdFormData| {
        let Some(id) = edit_id.get() else { return };
        form_loading.set(true);
        spawn_local(async move {
            let req = UpdateDvdRequest {
                name: Some(data.name),
                year: Some(data.year),
                realisator: data.director,
                actors: Some(data.actors),
                genre: data.genre,
            };
            let updated = match
                api::collection::update_dvd(
                    &id, &req,
                )
                .await
            {
                Ok(v) => v,
                Err(e) => {
                    toast.error(e);
                    form_loading.set(false);
                    return;
                }
            };
            let idx = dvds.with(|v| {
                v.iter()
                    .position(|d| d.id == id)
            });
            if let Some(i) = idx {
                dvds.update(|v| v[i] = updated);
            }
            edit_id.set(None);
            toast.success(
                "DVD updated".into(),
            );
            form_loading.set(false);
        });
    };

    let on_delete = Callback::new(move |id: String| {
        spawn_local(async move {
            match api::collection::delete_dvd(&id).await {
                Ok(true) => {
                    dvds.update(
                        |v| v.retain(|d| d.id != id),
                    );
                    toast.success("DVD deleted".into());
                }
                Ok(false) => {
                    toast.error("DVD not found".into())
                }
                Err(e) => toast.error(e),
            }
        });
    });

    let open_edit = Callback::new(move |id: String| {
        edit_id.set(Some(id));
    });

    let editing_dvd = Signal::derive(move || {
        let eid = edit_id.get()?;
        dvds.with(|v| {
            v.iter().find(|d| d.id == eid).cloned()
        })
    });

    view! {
        <HomeTopBar />
        <PageLayout>
            <HeroBanner title="My DVDs">
                <div class=style::banner_actions>
                    <Button
                        text="Add DVD"
                        on_click=nav_click(
                            "/collection/add",
                        )
                    />
                </div>
            </HeroBanner>
            <DvdList
                dvds=dvds
                loading=loading
                on_edit=open_edit
                on_delete=on_delete
            />
        </PageLayout>
        // Edit modal outside page stacking context
        <Modal
            open=Signal::derive(move || edit_id.get().is_some())
            title="Edit DVD".to_string()
            on_close=on_close(move || edit_id.set(None))
        >
            {move || editing_dvd.get().map(|dvd| view! {
                <DvdForm
                    dvd=dvd
                    on_submit=Callback::new(on_edit)
                    loading=form_loading
                />
            })}
        </Modal>
    }
}
