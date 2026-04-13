//! Add DVD — full-page create form and shared DvdForm.

use crate::api::log;
use crate::auth::use_auth_guard;
use crate::collection::api as dvd_api;
use crate::collection::types::{AddDvdRequest, Dvd};
use crate::home::HomeTopBar;
use crate::ui::{Button, HeroBanner, Input, PageLayout};
use dioxus::prelude::*;
use dioxus::router::Navigator;

/// Values collected by DvdForm and emitted on submit.
#[derive(Debug, Clone)]
pub struct DvdFormData {
    pub name: String,
    pub year: String,
    pub director: Option<String>,
    pub actors: Vec<String>,
    pub genre: Option<String>,
}

/// Route component at POST /collection/add.
pub fn DvdAddPage() -> Element {
    let _auth = use_auth_guard();
    let nav = navigator();
    let mut loading = use_signal(|| false);

    let on_submit = move |data: DvdFormData| {
        loading.set(true);
        spawn(async move {
            run_create(data, nav).await;
            loading.set(false);
        });
    };

    rsx! {
        HomeTopBar {}
        PageLayout {
            HeroBanner { title: "Add DVD", span {} }
            DvdForm {
                on_submit: on_submit,
                loading: loading,
            }
        }
    }
}

/// Send POST /dvds; on success navigate to list.
async fn run_create(data: DvdFormData, nav: Navigator) {
    let req = AddDvdRequest {
        name: data.name,
        year: data.year,
        realisator: data.director,
        actors: data.actors,
        genre: data.genre,
    };
    match dvd_api::add_dvd(&req).await {
        Ok(_) => {
            nav.push("/collection");
        }
        Err(e) => log::log_error(
            &format!("Add DVD failed: {e}"),
            "dvd_add",
        ),
    }
}

/// Form used to create or edit a DVD.
///
/// Pass `dvd: Some(_)` to prefill fields (edit mode)
/// or leave it at the default `None` for create.
#[component]
pub fn DvdForm(
    /// Existing DVD to edit, or None for create mode.
    #[props(default)]
    dvd: Option<Dvd>,
    /// Called with the entered values on submit.
    on_submit: EventHandler<DvdFormData>,
    /// External loading flag. Disables the submit btn.
    #[props(default)]
    loading: Signal<bool>,
) -> Element {
    let is_edit = dvd.is_some();
    let base = dvd.unwrap_or(Dvd::empty());
    let initial_year = if is_edit {
        base.year.to_string()
    } else {
        String::new()
    };

    let mut name = use_signal(|| base.name);
    let mut year = use_signal(|| initial_year);
    let mut director = use_signal(|| {
        base.realisator.unwrap_or_default()
    });
    let mut actors =
        use_signal(|| base.actors.join(", "));
    let mut genre = use_signal(|| {
        base.genre.unwrap_or_default()
    });

    let is_valid = use_memo(move || {
        !name().trim().is_empty()
            && !year().trim().is_empty()
    });
    let mut disabled = use_signal(|| false);
    use_effect(move || {
        disabled.set(!is_valid() || loading());
    });

    let btn_text =
        if is_edit { "Save" } else { "Add" };
    let submit = EventHandler::new(move |_| {
        let data = build_form_data(
            &name(),
            &year(),
            &director(),
            &actors(),
            &genre(),
        );
        on_submit.call(data);
    });

    rsx! {
        div { class: "flex flex-col gap-4",
            div { class: "flex flex-col gap-4",
                Input {
                    input_type: "text".to_string(),
                    id: "dvd-name".to_string(),
                    placeholder: "Movie title".to_string(),
                    value: name,
                    on_input: EventHandler::new(
                        move |v: String| name.set(v),
                    ),
                    required: true,
                }
                Input {
                    input_type: "text".to_string(),
                    id: "dvd-year".to_string(),
                    placeholder: "Year".to_string(),
                    value: year,
                    on_input: EventHandler::new(
                        move |v: String| year.set(v),
                    ),
                    required: true,
                }
                Input {
                    input_type: "text".to_string(),
                    id: "dvd-director".to_string(),
                    placeholder: "Director".to_string(),
                    value: director,
                    on_input: EventHandler::new(
                        move |v: String| director.set(v),
                    ),
                }
                Input {
                    input_type: "text".to_string(),
                    id: "dvd-actors".to_string(),
                    placeholder:
                        "Actor 1, Actor 2".to_string(),
                    value: actors,
                    on_input: EventHandler::new(
                        move |v: String| actors.set(v),
                    ),
                }
                Input {
                    input_type: "text".to_string(),
                    id: "dvd-genre".to_string(),
                    placeholder:
                        "Action, Drama...".to_string(),
                    value: genre,
                    on_input: EventHandler::new(
                        move |v: String| genre.set(v),
                    ),
                }
            }
            Button {
                text: btn_text.to_string(),
                disabled: disabled,
                on_click: submit,
                class: "w-full".to_string(),
            }
        }
    }
}

/// Build form submission data from raw field values.
fn build_form_data(
    name: &str,
    year: &str,
    director: &str,
    actors: &str,
    genre: &str,
) -> DvdFormData {
    DvdFormData {
        name: name.to_string(),
        year: year.to_string(),
        director: non_empty(director),
        actors: parse_actors(actors),
        genre: non_empty(genre),
    }
}

/// Split a comma-separated list into trimmed entries.
fn parse_actors(raw: &str) -> Vec<String> {
    raw.split(',')
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .collect()
}

/// Convert blank strings to None, else Some(raw).
fn non_empty(raw: &str) -> Option<String> {
    if raw.trim().is_empty() {
        None
    } else {
        Some(raw.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_non_empty_with_text_returns_some() {
        assert_eq!(
            non_empty("hello"),
            Some("hello".to_string()),
        );
    }

    #[test]
    fn test_non_empty_with_blank_returns_none() {
        assert!(non_empty("   ").is_none());
    }

    #[test]
    fn test_parse_actors_splits_and_trims() {
        let out = parse_actors("Alice, Bob ,Carol");
        assert_eq!(
            out,
            vec!["Alice", "Bob", "Carol"],
        );
    }

    #[test]
    fn test_parse_actors_skips_empty_entries() {
        let out = parse_actors(", ,Alice,");
        assert_eq!(out, vec!["Alice"]);
    }

    #[test]
    fn test_build_form_data_maps_all_fields() {
        let data = build_form_data(
            "Movie", "2024", "Dir", "A", "Action",
        );
        assert_eq!(data.name, "Movie");
        assert_eq!(data.year, "2024");
        assert_eq!(data.director, Some("Dir".into()));
        assert_eq!(data.actors, vec!["A"]);
        assert_eq!(data.genre, Some("Action".into()));
    }

    #[test]
    fn test_build_form_data_empty_optional_fields() {
        let data =
            build_form_data("M", "2024", "", "", "");
        assert!(data.director.is_none());
        assert!(data.actors.is_empty());
        assert!(data.genre.is_none());
    }
}
