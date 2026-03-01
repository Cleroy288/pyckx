//! Select Component - Dropdown selection with label

use leptos::prelude::*;

stylance::import_crate_style!(style, "src/components/ui/select/select.module.css");

#[derive(Clone)]
pub struct SelectOption {
    pub value: String,
    pub label: String,
}

impl SelectOption {
    pub fn new(value: impl Into<String>, label: impl Into<String>) -> Self {
        Self {
            value: value.into(),
            label: label.into(),
        }
    }
}

#[component]
pub fn Select(
    #[prop(into)] id: String,
    #[prop(optional, into)] label: Option<String>,
    #[prop(optional, into)]
    placeholder: Option<String>,
    #[prop(into)] value: Signal<String>,
    #[prop(into)] on_change: Callback<String>,
    #[prop(into)] options: Vec<SelectOption>,
    #[prop(optional, into)] class: Option<String>,
    #[prop(optional, into)] disabled: Signal<bool>,
) -> impl IntoView {
    // Capture initial value for `selected` attr
    let initial = value.get_untracked();
    view! {
        <div class=format!(
            "{} {}",
            style::form_group,
            class.unwrap_or_default()
        )>
            {label.map(|l| view! {
                <label
                    for=id.clone()
                    class=style::label
                >
                    {l}
                </label>
            })}
            <select
                id=id
                class=style::select
                disabled=disabled
                prop:value=value
                on:change=move |ev| {
                    on_change.run(event_target_value(&ev))
                }
            >
                {placeholder.map(|p| view! {
                    <option value="" disabled=true>
                        {p}
                    </option>
                })}
                {options.into_iter().map(|opt| {
                    let selected =
                        opt.value == initial;
                    view! {
                        <option
                            value=opt.value
                            selected=selected
                        >
                            {opt.label}
                        </option>
                    }
                }).collect_view()}
            </select>
        </div>
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_select_option_new_sets_fields() {
        let opt = SelectOption::new("val", "Label");
        assert_eq!(opt.value, "val");
        assert_eq!(opt.label, "Label");
    }

    #[test]
    fn test_select_option_new_with_string() {
        let opt = SelectOption::new(
            String::from("k"),
            String::from("Key"),
        );
        assert_eq!(opt.value, "k");
        assert_eq!(opt.label, "Key");
    }
}
