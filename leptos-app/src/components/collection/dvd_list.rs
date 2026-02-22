// ** dvd_list.rs **
// ==> Filtered/sorted list of DVDs with search

use crate::components::collection::dvd_card::DvdCard;
use crate::components::ui::card::CardGrid;
use crate::components::ui::empty_state::EmptyState;
use crate::components::ui::input::Input;
use crate::components::ui::spinner::Spinner;
use crate::domain::collection_types::Dvd;
use leptos::prelude::*;

stylance::import_crate_style!(
    style,
    "src/components/collection/dvd_list.module.css"
);

/// List of DVDs with search and filtering
#[component]
pub fn DvdList(
    /// DVD data
    #[prop(into)]
    dvds: Signal<Vec<Dvd>>,
    /// Loading state
    #[prop(into)]
    loading: Signal<bool>,
    /// Called when edit button clicked
    #[prop(into)]
    on_edit: Callback<String>,
    /// Called when delete button clicked
    #[prop(into)]
    on_delete: Callback<String>,
) -> impl IntoView {
    let search = RwSignal::new(String::new());

    let filtered = Signal::derive(move || {
        let q = search.get().to_lowercase();
        let all = dvds.get();
        if q.is_empty() {
            return all;
        }
        all.into_iter()
            .filter(|d| {
                d.name.to_lowercase().contains(&q)
                    || d.genre
                        .as_ref()
                        .is_some_and(|g| {
                            g.to_lowercase().contains(&q)
                        })
                    || d.realisator
                        .as_ref()
                        .is_some_and(|r| {
                            r.to_lowercase().contains(&q)
                        })
            })
            .collect::<Vec<_>>()
    });

    view! {
        <div class=style::list_wrapper>
            <Input
                type_="text"
                id="dvd-search"
                placeholder="Search DVDs...".to_string()
                value=search
                on_input=Callback::new(move |v| search.set(v))
            />
            <Show
                when=move || !loading.get()
                fallback=|| view! { <Spinner /> }
            >
                <Show
                    when=move || !filtered.get().is_empty()
                    fallback=|| view! {
                        <EmptyState
                            icon="Disc".to_string()
                            message="No DVDs found".to_string()
                            description="Add your first DVD to start your collection.".to_string()
                        />
                    }
                >
                    <CardGrid>
                        <For
                            each=move || filtered.get()
                            key=|d| d.id.clone()
                            let:dvd
                        >
                            <DvdCard
                                dvd=dvd
                                on_edit=on_edit
                                on_delete=on_delete
                            />
                        </For>
                    </CardGrid>
                </Show>
            </Show>
        </div>
    }
}
