// ** input.rs **
// ==> Reusable Input Component with label and error handling

use leptos::prelude::*;

// Import scoped styles
stylance::import_crate_style!(style, "src/components/ui/input/input.module.css");

#[component]
pub fn Input(
    #[prop(into)] type_: String,
    #[prop(into)] id: String,
    #[prop(optional, into)] label: Option<String>,
    #[prop(optional, into)] placeholder: Option<String>,
    #[prop(into)] value: Signal<String>,
    #[prop(into)] on_input: Callback<String>,
    #[prop(optional, into)] required: bool,
    #[prop(optional, into)] minlength: Option<u32>,
    #[prop(optional, into)] error: Option<String>,
    #[prop(optional, into)] class: Option<String>,
) -> impl IntoView {
    view! {
        <div class=format!("{} {}", style::form_group, class.unwrap_or_default())>
            {label.map(|l| view! {
                <label for=id.clone() class=style::label>{l}</label>
            })}
            <input
                type=type_
                id=id
                class=format!("{} {}", style::input, if error.is_some() { style::error } else { "" })
                placeholder=placeholder.unwrap_or_default()
                required=required
                minlength=minlength.unwrap_or(0).to_string()
                prop:value=value
                on:input=move |ev| on_input.run(event_target_value(&ev))
            />
            {error.map(|e| view! {
                <span class=style::error_text>{e}</span>
            })}
        </div>
    }
}
