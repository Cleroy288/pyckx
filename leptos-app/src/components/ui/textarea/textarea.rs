//! Textarea Component - Multi-line text input

use leptos::prelude::*;

stylance::import_crate_style!(
    style,
    "src/components/ui/textarea/textarea.module.css"
);

/// Multi-line text input with label and error
#[component]
pub fn Textarea(
    #[prop(into)] id: String,
    #[prop(optional, into)] label: Option<String>,
    #[prop(optional, into)]
    placeholder: Option<String>,
    #[prop(into)] value: Signal<String>,
    #[prop(into)] on_input: Callback<String>,
    #[prop(optional, default = 3)] rows: u32,
    #[prop(optional, into)] required: bool,
    #[prop(optional, into)]
    error: Signal<Option<String>>,
    #[prop(optional, into)]
    class: Option<String>,
    #[prop(optional, into)]
    disabled: Signal<bool>,
) -> impl IntoView {
    let id_for_label = id.clone();
    let extra = class.unwrap_or_default();

    view! {
        <div class=format!(
            "{} {}",
            style::form_group,
            extra
        )>
            {label.map(|l| view! {
                <label
                    for=id_for_label.clone()
                    class=style::label
                >
                    {l}
                </label>
            })}
            <textarea
                id=id
                class=move || format!(
                    "{} {}",
                    style::textarea,
                    if error.get().is_some() {
                        style::error
                    } else {
                        ""
                    }
                )
                placeholder=placeholder
                    .unwrap_or_default()
                rows=rows.to_string()
                required=required
                disabled=disabled
                prop:value=value
                on:input=move |ev| {
                    on_input
                        .run(event_target_value(&ev))
                }
            ></textarea>
            {move || error.get().map(|e| view! {
                <span class=style::error_text>
                    {e}
                </span>
            })}
        </div>
    }
}
