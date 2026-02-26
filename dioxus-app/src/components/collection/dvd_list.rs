//! DvdList — filtered/sorted list of DVDs with search

use crate::components::collection::dvd_card::DvdCard;
use crate::components::ui::card::CardGrid;
use crate::components::ui::empty_state::EmptyState;
use crate::components::ui::input::Input;
use crate::components::ui::spinner::Spinner;
use crate::domain::collection_types::Dvd;
use dioxus::prelude::*;

/// List of DVDs with search and filtering
#[component]
pub fn DvdList(
    /// DVD data signal
    dvds: Signal<Vec<Dvd>>,
    /// Loading state signal
    loading: Signal<bool>,
    /// Called when edit button clicked
    on_edit: EventHandler<String>,
    /// Called when delete button clicked
    on_delete: EventHandler<String>,
) -> Element {
    let mut search = use_signal(String::new);

    let filtered = use_memo(move || {
        let q = search().to_lowercase();
        let all = dvds();
        if q.is_empty() {
            return all;
        }
        all.into_iter()
            .filter(|d| matches_query(d, &q))
            .collect::<Vec<_>>()
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
                    description: "Add your first DVD \
                        to start your collection."
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

/// Check if a DVD matches a search query
fn matches_query(dvd: &Dvd, q: &str) -> bool {
    dvd.name.to_lowercase().contains(q)
        || dvd
            .genre
            .as_ref()
            .is_some_and(|g| {
                g.to_lowercase().contains(q)
            })
        || dvd
            .realisator
            .as_ref()
            .is_some_and(|r| {
                r.to_lowercase().contains(q)
            })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_dvd(name: &str) -> Dvd {
        Dvd {
            id: "1".into(),
            name: name.into(),
            year: 2024,
            realisator: None,
            actors: vec![],
            genre: None,
            created_at: String::new(),
            updated_at: String::new(),
        }
    }

    #[test]
    fn test_matches_query_by_name() {
        let dvd = test_dvd("Matrix");
        assert!(matches_query(&dvd, "matrix"));
    }

    #[test]
    fn test_matches_query_by_genre() {
        let mut dvd = test_dvd("Film");
        dvd.genre = Some("Action".into());
        assert!(matches_query(&dvd, "action"));
    }

    #[test]
    fn test_matches_query_no_match() {
        let dvd = test_dvd("Matrix");
        assert!(!matches_query(&dvd, "xyz"));
    }

    #[test]
    fn test_matches_query_by_director() {
        let mut dvd = test_dvd("Film");
        dvd.realisator = Some("Nolan".into());
        assert!(matches_query(&dvd, "nolan"));
    }
}
