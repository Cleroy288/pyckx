//! Collection list page and DvdList component.

use crate::api::log;
use crate::auth::use_auth_guard;
use crate::collection::add::{DvdForm, DvdFormData};
use crate::collection::api as dvd_api;
use crate::collection::card::DvdCard;
use crate::collection::types::{Dvd, UpdateDvdRequest};
use crate::home::HomeTopBar;
use crate::state::hooks::{use_fetch_on_mount, FetchFn};
use crate::ui::{
    Button, CardGrid, EmptyState, HeroBanner, Input,
    Modal, PageLayout, Spinner,
};
use dioxus::prelude::*;

/// Route component at /collection. Lists DVDs and
/// hosts the edit modal.
pub fn CollectionPage() -> Element {
    let _auth = use_auth_guard();
    let nav = navigator();

    let dvds: Signal<Vec<Dvd>> = use_signal(Vec::new);
    let loading = use_signal(|| true);
    let edit_id: Signal<Option<String>> =
        use_signal(|| None);
    let form_loading = use_signal(|| false);

    let fetch: FetchFn<Vec<Dvd>> =
        Box::new(|| Box::pin(dvd_api::get_dvds()));
    use_fetch_on_mount(dvds, loading, fetch);

    let state = PageState {
        dvds,
        loading,
        edit_id,
        form_loading,
    };
    rsx! {
        div { class: "home-page",
            HomeTopBar {}
            PageLayout {
                HeroBanner {
                    title: "My DVDs",
                    div {
                        class: "absolute bottom-3 left-4 \
                            z-[1] flex gap-2",
                        Button {
                            text: "Add DVD",
                            on_click: move |_| {
                                nav.push("/collection/add");
                            },
                        }
                    }
                }
                {list_body(state)}
            }
        }
        {edit_modal(state)}
    }
}

/// Bundle of signals passed through CollectionPage.
#[derive(Clone, Copy)]
struct PageState {
    dvds: Signal<Vec<Dvd>>,
    loading: Signal<bool>,
    edit_id: Signal<Option<String>>,
    form_loading: Signal<bool>,
}

/// Render the DvdList bound to page handlers.
fn list_body(state: PageState) -> Element {
    let mut edit_id = state.edit_id;
    let dvds = state.dvds;
    let on_edit =
        move |id: String| edit_id.set(Some(id));
    let on_delete = move |id: String| {
        spawn(async move {
            delete_and_remove(id, dvds).await;
        });
    };
    rsx! {
        DvdList {
            dvds: state.dvds,
            loading: state.loading,
            on_edit: on_edit,
            on_delete: on_delete,
        }
    }
}

/// Delete the DVD remotely then remove it locally.
async fn delete_and_remove(
    id: String,
    mut dvds: Signal<Vec<Dvd>>,
) {
    if let Ok(true) = dvd_api::delete_dvd(&id).await {
        dvds.write().retain(|d| d.id != id);
    }
}

/// Render the edit modal when an id is selected.
fn edit_modal(state: PageState) -> Element {
    let mut edit_id = state.edit_id;
    if edit_id().is_none() {
        return rsx! {};
    }
    let editing = find_editing(state);
    let on_submit = move |data: DvdFormData| {
        submit_edit(data, state);
    };
    rsx! {
        Modal {
            open: Signal::new(true),
            title: "Edit DVD",
            on_close: move |_| edit_id.set(None),
            if let Some(dvd) = editing() {
                DvdForm {
                    dvd: dvd,
                    on_submit: on_submit,
                    loading: state.form_loading,
                }
            }
        }
    }
}

/// Memo: the DVD currently being edited, if any.
fn find_editing(
    state: PageState,
) -> Memo<Option<Dvd>> {
    let edit_id = state.edit_id;
    let dvds = state.dvds;
    use_memo(move || {
        let eid = edit_id()?;
        dvds().iter().find(|d| d.id == eid).cloned()
    })
}

/// Fire the update request and reconcile local state.
fn submit_edit(data: DvdFormData, state: PageState) {
    let Some(id) = (state.edit_id)() else { return };
    let mut form_loading = state.form_loading;
    let mut edit_id = state.edit_id;
    let mut dvds = state.dvds;
    form_loading.set(true);
    spawn(async move {
        let req = to_update_request(data);
        match dvd_api::update_dvd(&id, &req).await {
            Ok(updated) => {
                replace_in_list(&mut dvds, &id, updated);
                edit_id.set(None);
            }
            Err(e) => log::log_error(
                &format!("DVD update failed: {e}"),
                "collection",
            ),
        }
        form_loading.set(false);
    });
}

/// Turn form data into an API update payload.
fn to_update_request(
    data: DvdFormData,
) -> UpdateDvdRequest {
    UpdateDvdRequest {
        name: Some(data.name),
        year: Some(data.year),
        realisator: data.director,
        actors: Some(data.actors),
        genre: data.genre,
    }
}

/// Replace a DVD in the list by id, if present.
fn replace_in_list(
    dvds: &mut Signal<Vec<Dvd>>,
    id: &str,
    updated: Dvd,
) {
    let idx = dvds().iter().position(|d| d.id == id);
    if let Some(i) = idx {
        dvds.write()[i] = updated;
    }
}

/// List of DVDs with a live search input.
#[component]
pub fn DvdList(
    /// Signal carrying the DVD collection.
    dvds: Signal<Vec<Dvd>>,
    /// Loading flag from the parent page.
    loading: Signal<bool>,
    /// Fired with the DVD id when Edit is clicked.
    on_edit: EventHandler<String>,
    /// Fired with the DVD id when Delete is clicked.
    on_delete: EventHandler<String>,
) -> Element {
    let mut search = use_signal(String::new);
    let filtered = use_memo(move || {
        filter_dvds(dvds(), &search())
    });

    rsx! {
        div { class: "flex flex-col gap-6",
            Input {
                input_type: "text".to_string(),
                id: "dvd-search".to_string(),
                placeholder: "Search DVDs..."
                    .to_string(),
                value: search,
                on_input: EventHandler::new(
                    move |v: String| search.set(v),
                ),
            }
            if *loading.read() {
                Spinner {}
            } else if filtered().is_empty() {
                EmptyState {
                    icon: "Disc".to_string(),
                    message: "No DVDs found"
                        .to_string(),
                    description:
                        "Add your first DVD to start \
                         your collection."
                            .to_string(),
                }
            } else {
                CardGrid {
                    for dvd in filtered() {
                        DvdCard {
                            key: "{dvd.id}",
                            dvd: dvd.clone(),
                            on_edit: on_edit,
                            on_delete: on_delete,
                        }
                    }
                }
            }
        }
    }
}

/// Keep DVDs whose name/genre/director matches query.
fn filter_dvds(all: Vec<Dvd>, query: &str) -> Vec<Dvd> {
    let q = query.to_lowercase();
    if q.is_empty() {
        return all;
    }
    all.into_iter()
        .filter(|d| matches_query(d, &q))
        .collect()
}

/// Check if a DVD matches the lowercased query.
fn matches_query(dvd: &Dvd, q: &str) -> bool {
    dvd.name.to_lowercase().contains(q)
        || matches_opt(&dvd.genre, q)
        || matches_opt(&dvd.realisator, q)
}

/// True when an optional string contains the query.
fn matches_opt(
    field: &Option<String>,
    q: &str,
) -> bool {
    field
        .as_ref()
        .is_some_and(|s| s.to_lowercase().contains(q))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample(name: &str) -> Dvd {
        Dvd {
            name: name.into(),
            ..Dvd::empty()
        }
    }

    #[test]
    fn test_matches_query_by_name_substring() {
        let dvd = sample("The Matrix");
        assert!(matches_query(&dvd, "matrix"));
    }

    #[test]
    fn test_matches_query_by_genre() {
        let mut dvd = sample("Film");
        dvd.genre = Some("Action".into());
        assert!(matches_query(&dvd, "action"));
    }

    #[test]
    fn test_matches_query_by_director() {
        let mut dvd = sample("Film");
        dvd.realisator = Some("Nolan".into());
        assert!(matches_query(&dvd, "nolan"));
    }

    #[test]
    fn test_matches_query_no_match_returns_false() {
        let dvd = sample("Matrix");
        assert!(!matches_query(&dvd, "xyz"));
    }

    #[test]
    fn test_filter_dvds_empty_query_returns_all() {
        let all =
            vec![sample("A"), sample("B")];
        let out = filter_dvds(all.clone(), "");
        assert_eq!(out.len(), 2);
    }

    #[test]
    fn test_filter_dvds_filters_by_name() {
        let all =
            vec![sample("Matrix"), sample("Inception")];
        let out = filter_dvds(all, "matrix");
        assert_eq!(out.len(), 1);
        assert_eq!(out[0].name, "Matrix");
    }

    #[test]
    fn test_matches_opt_with_none_returns_false() {
        assert!(!matches_opt(&None, "x"));
    }

    #[test]
    fn test_to_update_request_wraps_fields() {
        let data = DvdFormData {
            name: "N".into(),
            year: "2024".into(),
            director: Some("D".into()),
            actors: vec!["A".into()],
            genre: Some("G".into()),
        };
        let req = to_update_request(data);
        assert_eq!(req.name, Some("N".into()));
        assert_eq!(req.realisator, Some("D".into()));
        assert_eq!(req.actors, Some(vec!["A".into()]));
    }
}
