// ** form_field.rs **
// ==> Label + child input + error message wrapper

use leptos::prelude::*;

stylance::import_crate_style!(
    style,
    "src/components/ui/form_field/form_field.module.css"
);

/// Form field wrapper with label and error
#[component]
pub fn FormField(
    /// Label text above the field
    #[prop(into)]
    label: String,
    /// Optional error message below the field
    #[prop(optional, into)]
    error: Signal<Option<String>>,
    /// The input element(s) inside
    children: Children,
    #[prop(optional, into)]
    class: Option<String>,
) -> impl IntoView {
    view! {
        <div class=format!(
            "{} {}",
            style::field,
            class.unwrap_or_default()
        )>
            <label class=style::label>{label}</label>
            {children()}
            <Show when=move || error.get().is_some()>
                <span class=style::error>
                    {move || error.get().unwrap_or_default()}
                </span>
            </Show>
        </div>
    }
}
