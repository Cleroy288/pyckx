// ** progress.rs **
// ==> Horizontal fill bar with label

use leptos::prelude::*;

stylance::import_crate_style!(
    style,
    "src/components/ui/progress/progress.module.css"
);

/// Horizontal progress bar
#[component]
pub fn Progress(
    /// Value between 0 and 100
    #[prop(into)]
    value: Signal<f64>,
    /// Optional label above the bar
    #[prop(optional, into)]
    label: Option<String>,
    #[prop(optional, into)]
    class: Option<String>,
) -> impl IntoView {
    let pct = Signal::derive(move || {
        value.get().clamp(0.0, 100.0)
    });

    view! {
        <div class=format!(
            "{} {}",
            style::wrapper,
            class.unwrap_or_default()
        )>
            {label.map(|l| view! {
                <div class=style::label_row>
                    <span class=style::label>{l}</span>
                    <span class=style::percent>
                        {move || format!("{:.0}%", pct.get())}
                    </span>
                </div>
            })}
            <div class=style::track>
                <div
                    class=style::fill
                    style=move || format!(
                        "width: {:.0}%", pct.get()
                    )
                />
            </div>
        </div>
    }
}
