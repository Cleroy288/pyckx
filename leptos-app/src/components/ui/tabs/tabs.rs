// ** tabs.rs **
// ==> Tab header bar + panel for conditional rendering

use leptos::prelude::*;

stylance::import_crate_style!(
    style,
    "src/components/ui/tabs/tabs.module.css"
);

/// Tab header bar with clickable labels
#[component]
pub fn Tabs(
    /// Tab labels
    #[prop(into)]
    labels: Vec<String>,
    /// Currently active tab index
    #[prop(into)]
    active: Signal<usize>,
    /// Called when tab is clicked
    #[prop(into)]
    on_change: Callback<usize>,
    #[prop(optional, into)]
    class: Option<String>,
) -> impl IntoView {
    view! {
        <div class=format!(
            "{} {}",
            style::tabs,
            class.unwrap_or_default()
        )>
            {labels.into_iter().enumerate().map(|(i, label)| {
                view! {
                    <button
                        class=move || format!(
                            "{} {}",
                            style::tab,
                            if active.get() == i { style::active } else { "" }
                        )
                        on:click=move |_| on_change.run(i)
                    >
                        {label}
                    </button>
                }
            }).collect_view()}
        </div>
    }
}

/// Conditionally renders content for a tab index
#[component]
pub fn TabPanel(
    /// Index this panel corresponds to
    index: usize,
    /// Active tab signal
    #[prop(into)]
    active: Signal<usize>,
    /// Panel content
    children: Children,
) -> impl IntoView {
    view! {
        <div
            class=style::panel
            style:display=move || {
                if active.get() == index {
                    "block"
                } else {
                    "none"
                }
            }
        >
            {children()}
        </div>
    }
}
