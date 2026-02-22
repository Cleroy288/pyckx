// ** dvd_form.rs **
// ==> Create/edit form for DVDs

use crate::components::ui::button::{
    btn_click, Button,
};
use crate::components::ui::input::Input;
use crate::domain::collection_types::Dvd;
use leptos::prelude::*;

stylance::import_crate_style!(
    style,
    "src/components/collection/dvd_form.module.css"
);

/// Form for creating or editing a DVD
#[component]
pub fn DvdForm(
    /// Existing DVD for edit mode (None for create)
    #[prop(optional)]
    dvd: Option<Dvd>,
    /// Called on submit with (name, year, director, actors, genre)
    #[prop(into)]
    on_submit: Callback<DvdFormData>,
    /// Loading state
    #[prop(optional, into)]
    loading: Signal<bool>,
) -> impl IntoView {
    let is_edit = dvd.is_some();
    let dvd = dvd.unwrap_or_default_dvd();

    let name = RwSignal::new(dvd.name);
    let year_str = if is_edit {
        dvd.year.to_string()
    } else {
        String::new()
    };
    let year = RwSignal::new(year_str);
    let director = RwSignal::new(
        dvd.realisator.unwrap_or_default(),
    );
    let actors = RwSignal::new(dvd.actors.join(", "));
    let genre = RwSignal::new(
        dvd.genre.unwrap_or_default(),
    );

    let is_valid = Signal::derive(move || {
        !name.get().trim().is_empty()
            && !year.get().trim().is_empty()
    });

    let disabled = Signal::derive(move || {
        !is_valid.get() || loading.get()
    });

    let handle_submit = move |_| {
        let actors_vec: Vec<String> = actors
            .get()
            .split(',')
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty())
            .collect();
        let data = DvdFormData {
            name: name.get(),
            year: year.get(),
            director: non_empty(director.get()),
            actors: actors_vec,
            genre: non_empty(genre.get()),
        };
        on_submit.run(data);
    };

    let btn_text = if is_edit { "Save" } else { "Add" };

    view! {
        <div class=style::form>
            <div class=style::inputs>
                <Input
                    type_="text"
                    id="dvd-name"
                    placeholder="Movie title".to_string()
                    value=name
                    on_input=Callback::new(move |v| name.set(v))
                    required=true
                />
                <Input
                    type_="text"
                    id="dvd-year"
                    placeholder="Year".to_string()
                    value=year
                    on_input=Callback::new(move |v| year.set(v))
                    required=true
                />
                <Input
                    type_="text"
                    id="dvd-director"
                    placeholder="Director".to_string()
                    value=director
                    on_input=Callback::new(move |v| director.set(v))
                />
                <Input
                    type_="text"
                    id="dvd-actors"
                    placeholder="Actor 1, Actor 2".to_string()
                    value=actors
                    on_input=Callback::new(move |v| actors.set(v))
                />
                <Input
                    type_="text"
                    id="dvd-genre"
                    placeholder="Action, Drama...".to_string()
                    value=genre
                    on_input=Callback::new(move |v| genre.set(v))
                />
            </div>
            <Button
                text=btn_text.to_string()
                disabled=disabled
                on_click=btn_click(handle_submit)
                class=style::submit.to_string()
            />
        </div>
    }
}

/// Data emitted by the form on submit
#[derive(Debug, Clone)]
pub struct DvdFormData {
    pub name: String,
    pub year: String,
    pub director: Option<String>,
    pub actors: Vec<String>,
    pub genre: Option<String>,
}

/// Convert empty string to None
fn non_empty(s: String) -> Option<String> {
    if s.trim().is_empty() {
        None
    } else {
        Some(s)
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
        let result = non_empty("hello".to_string());
        assert_eq!(result, Some("hello".to_string()));
    }

    #[test]
    fn test_non_empty_with_empty_returns_none() {
        assert!(non_empty(String::new()).is_none());
    }

    #[test]
    fn test_non_empty_with_whitespace_returns_none() {
        assert!(non_empty("   ".to_string()).is_none());
    }

    #[test]
    fn test_unwrap_default_with_none_returns_empty_dvd() {
        let dvd: Option<Dvd> = None;
        let result = dvd.unwrap_or_default_dvd();
        assert!(result.name.is_empty());
        assert_eq!(result.year, 0);
    }

    #[test]
    fn test_unwrap_default_with_some_returns_dvd() {
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
        let result = Some(dvd).unwrap_or_default_dvd();
        assert_eq!(result.name, "Test");
        assert_eq!(result.year, 2024);
    }
}
