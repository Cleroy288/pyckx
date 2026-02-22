// ** session_card.rs **
// ==> Displays a study session with status and actions

use crate::components::ui::badge::{Badge, BadgeVariant};
use crate::components::ui::button::{
    btn_click, Button, ButtonSize,
};
use leptos::prelude::*;

stylance::import_crate_style!(
    style,
    "src/components/intello/course/session_card.module.css"
);

/// Study session card
#[component]
pub fn SessionCard(
    /// Session topic
    #[prop(into)]
    topic: String,
    /// Session status (pending, completed, etc.)
    #[prop(into)]
    status: String,
    /// Creation date
    #[prop(into)]
    created_at: String,
    /// Called when view button clicked
    #[prop(into)]
    on_view: Callback<()>,
) -> impl IntoView {
    let variant = match status.as_str() {
        "completed" => BadgeVariant::Success,
        "failed" => BadgeVariant::Error,
        _ => BadgeVariant::Warning,
    };

    view! {
        <div class=style::card>
            <div class=style::info>
                <h4 class=style::topic>{topic}</h4>
                <Badge text=status variant=variant />
            </div>
            <span class=style::date>{created_at}</span>
            <div class=style::actions>
                <Button
                    text="View"
                    size=ButtonSize::Small
                    on_click=btn_click(move |_| {
                        on_view.run(())
                    })
                />
            </div>
        </div>
    }
}
