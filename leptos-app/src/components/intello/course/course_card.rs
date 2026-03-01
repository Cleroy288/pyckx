// ** course_card.rs **
// ==> Displays a single course with metadata

use crate::components::ui::button::{
    btn_click, Button, ButtonSize, ButtonVariant,
};
use leptos::prelude::*;

stylance::import_crate_style!(
    style,
    "src/components/intello/course/course_card.module.css"
);

/// Course card with name, description, actions
#[component]
pub fn CourseCard(
    /// Course name
    #[prop(into)]
    name: String,
    /// Course description
    #[prop(into)]
    description: String,
    /// Creation date
    #[prop(into)]
    created_at: String,
    /// Called when view button clicked
    #[prop(into)]
    on_view: Callback<()>,
    /// Called when delete button clicked
    #[prop(into)]
    on_delete: Callback<()>,
) -> impl IntoView {
    view! {
        <div class=style::card>
            <div class=style::content>
                <h3 class=style::name>{name}</h3>
                <p class=style::desc>{description}</p>
                <span class=style::date>{created_at}</span>
            </div>
            <div class=style::actions>
                <Button
                    text="View"
                    size=ButtonSize::Small
                    on_click=btn_click(move |_| {
                        on_view.run(())
                    })
                />
                <Button
                    text="Delete"
                    size=ButtonSize::Small
                    variant=ButtonVariant::Outline
                    on_click=btn_click(move |_| {
                        on_delete.run(())
                    })
                />
            </div>
        </div>
    }
}
