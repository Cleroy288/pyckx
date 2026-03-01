// ** dvd_card.rs **
// ==> Single DVD display with edit/delete buttons

use crate::components::ui::badge::Badge;
use crate::components::ui::button::{
    btn_click, Button, ButtonSize, ButtonVariant,
};
use crate::components::ui::card::ItemCard;
use crate::domain::collection_types::Dvd;
use leptos::prelude::*;

stylance::import_crate_style!(
    style,
    "src/components/collection/dvd_card.module.css"
);

/// Display a single DVD with actions
#[component]
pub fn DvdCard(
    /// DVD data to display
    dvd: Dvd,
    /// Called when edit is clicked
    #[prop(into)]
    on_edit: Callback<String>,
    /// Called when delete is clicked
    #[prop(into)]
    on_delete: Callback<String>,
) -> impl IntoView {
    let edit_id = dvd.id.clone();
    let del_id = dvd.id.clone();

    view! {
        <ItemCard
            name=dvd.name.clone()
            header_end=view! {
                <Badge text=dvd.year.to_string() />
            }.into_any()
            actions=view! {
                <Button
                    text="Edit"
                    size=ButtonSize::Small
                    on_click=btn_click(move |_| {
                        on_edit.run(edit_id.clone())
                    })
                />
                <Button
                    text="Delete"
                    size=ButtonSize::Small
                    variant=ButtonVariant::Outline
                    on_click=btn_click(move |_| {
                        on_delete.run(del_id.clone())
                    })
                />
            }.into_any()
        >
            {dvd.realisator.as_ref().map(|r| view! {
                <p class=style::director>
                    {r.clone()}
                </p>
            })}
            {
                let actors = dvd.actors.clone();
                let actors2 = dvd.actors.clone();
                view! {
                    <Show when=move || {
                        !actors.is_empty()
                    }>
                        <p class=style::actors>
                            {actors2.join(", ")}
                        </p>
                    </Show>
                }
            }
            {dvd.genre.as_ref().map(|g| view! {
                <Badge text=g.clone() />
            })}
        </ItemCard>
    }
}
