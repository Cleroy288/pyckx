//! DvdForm — create/edit form for DVDs

use crate::components::ui::button::Button;
use crate::components::ui::input::Input;
use crate::domain::collection_types::Dvd;
use dioxus::prelude::*;

/// Data emitted by the form on submit
#[derive(Debug, Clone)]
pub struct DvdFormData {
    pub name: String,
    pub year: String,
    pub director: Option<String>,
    pub actors: Vec<String>,
    pub genre: Option<String>,
}

/// Form for creating or editing a DVD
#[component]
pub fn DvdForm(
    /// Existing DVD for edit mode (None = create)
    #[props(default)]
    dvd: Option<Dvd>,
    /// Called on submit with form data
    on_submit: EventHandler<DvdFormData>,
    /// Loading state
    #[props(default)]
    loading: Signal<bool>,
) -> Element {
    let is_edit = dvd.is_some();
    let dvd = dvd.unwrap_or_default_dvd();

    let mut name = use_signal(|| dvd.name);
    let year_str = if is_edit {
        dvd.year.to_string()
    } else {
        String::new()
    };
    let mut year = use_signal(|| year_str);
    let mut director = use_signal(|| {
        dvd.realisator.unwrap_or_default()
    });
    let mut actors =
        use_signal(|| dvd.actors.join(", "));
    let mut genre = use_signal(|| {
        dvd.genre.unwrap_or_default()
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

    rsx! {
        div { class: "flex flex-col gap-4",
            div { class: "flex flex-col gap-4",
                Input {
                    input_type: "text".to_string(),
                    id: "dvd-name".to_string(),
                    placeholder: "Movie title"
                        .to_string(),
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
                    placeholder: "Director"
                        .to_string(),
                    value: director,
                    on_input: EventHandler::new(
                        move |v: String| {
                            director.set(v)
                        },
                    ),
                }
                Input {
                    input_type: "text".to_string(),
                    id: "dvd-actors".to_string(),
                    placeholder: "Actor 1, Actor 2"
                        .to_string(),
                    value: actors,
                    on_input: EventHandler::new(
                        move |v: String| {
                            actors.set(v)
                        },
                    ),
                }
                Input {
                    input_type: "text".to_string(),
                    id: "dvd-genre".to_string(),
                    placeholder: "Action, Drama..."
                        .to_string(),
                    value: genre,
                    on_input: EventHandler::new(
                        move |v: String| genre.set(v),
                    ),
                }
            }
            Button {
                text: btn_text.to_string(),
                disabled: disabled,
                on_click: EventHandler::new(
                    move |_| {
                        let data = build_form_data(
                            &name(),
                            &year(),
                            &director(),
                            &actors(),
                            &genre(),
                        );
                        on_submit.call(data);
                    },
                ),
                class: "w-full".to_string(),
            }
        }
    }
}

/// Build DvdFormData from form field values
fn build_form_data(
    name: &str,
    year: &str,
    director: &str,
    actors: &str,
    genre: &str,
) -> DvdFormData {
    let actors_vec: Vec<String> = actors
        .split(',')
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .collect();
    DvdFormData {
        name: name.to_string(),
        year: year.to_string(),
        director: non_empty(director),
        actors: actors_vec,
        genre: non_empty(genre),
    }
}

/// Convert empty string to None
fn non_empty(s: &str) -> Option<String> {
    if s.trim().is_empty() {
        None
    } else {
        Some(s.to_string())
    }
}

/// Default DVD for create mode
trait UnwrapDefault {
    fn unwrap_or_default_dvd(self) -> Dvd;
}

impl UnwrapDefault for Option<Dvd> {
    fn unwrap_or_default_dvd(self) -> Dvd {
        self.unwrap_or(Dvd {
            id: String::new(),
            name: String::new(),
            year: 0,
            realisator: None,
            actors: Vec::new(),
            genre: None,
            created_at: String::new(),
            updated_at: String::new(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_non_empty_with_text_returns_some() {
        let result = non_empty("hello");
        assert_eq!(
            result,
            Some("hello".to_string()),
        );
    }

    #[test]
    fn test_non_empty_with_empty_returns_none() {
        assert!(non_empty("").is_none());
    }

    #[test]
    fn test_non_empty_with_whitespace_returns_none() {
        assert!(non_empty("   ").is_none());
    }

    #[test]
    fn test_unwrap_default_none_returns_empty_dvd() {
        let dvd: Option<Dvd> = None;
        let result = dvd.unwrap_or_default_dvd();
        assert!(result.name.is_empty());
        assert_eq!(result.year, 0);
    }

    #[test]
    fn test_unwrap_default_some_returns_dvd() {
        let dvd = Dvd {
            id: "1".into(),
            name: "Test".into(),
            year: 2024,
            realisator: Some("Dir".into()),
            actors: vec!["A".into()],
            genre: Some("Action".into()),
            created_at: String::new(),
            updated_at: String::new(),
        };
        let result =
            Some(dvd).unwrap_or_default_dvd();
        assert_eq!(result.name, "Test");
        assert_eq!(result.year, 2024);
    }

    #[test]
    fn test_build_form_data_parses_actors() {
        let data = build_form_data(
            "Movie",
            "2024",
            "Dir",
            "Alice, Bob",
            "Action",
        );
        assert_eq!(data.actors.len(), 2);
        assert_eq!(data.actors[0], "Alice");
        assert_eq!(data.actors[1], "Bob");
    }

    #[test]
    fn test_build_form_data_empty_actors() {
        let data = build_form_data(
            "Movie", "2024", "", "", "",
        );
        assert!(data.actors.is_empty());
        assert!(data.director.is_none());
        assert!(data.genre.is_none());
    }
}
