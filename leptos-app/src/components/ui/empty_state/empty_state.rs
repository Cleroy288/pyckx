// ** empty_state.rs **
// ==> Centered icon + message + optional CTA

use leptos::prelude::*;

stylance::import_crate_style!(
    style,
    "src/components/ui/empty_state/empty_state.module.css"
);

/// Empty state placeholder
#[component]
pub fn EmptyState(
    /// Icon name to display
    #[prop(into)]
    icon: String,
    /// Main message
    #[prop(into)]
    message: String,
    /// Optional description below message
    #[prop(optional, into)]
    description: Option<String>,
    /// Optional CTA slot
    #[prop(optional)]
    children: Option<Children>,
    #[prop(optional, into)]
    class: Option<String>,
) -> impl IntoView {
    view! {
        <div class=format!(
            "{} {}",
            style::empty,
            class.unwrap_or_default()
        )>
            <div class=style::icon_wrapper>
                <crate::components::ui::icon::Icon
                    icon_name=icon
                />
            </div>
            <h3 class=style::message>{message}</h3>
            {description.map(|d| view! {
                <p class=style::description>{d}</p>
            })}
            {children.map(|c| view! {
                <div class=style::action>{c()}</div>
            })}
        </div>
    }
}
