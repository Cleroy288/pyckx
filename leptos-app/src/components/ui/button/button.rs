// ** button.rs **
// ==> Reusable Button Component with style and size variants

use leptos::prelude::*;

// Import scoped styles
stylance::import_crate_style!(style, "src/components/ui/button/button.module.css");

#[derive(Clone, Copy, PartialEq, Default)]
pub enum ButtonVariant {
    #[default]
    Primary,
    Outline,
}

#[derive(Clone, Copy, PartialEq, Default)]
pub enum ButtonSize {
    Small,
    #[default]
    Medium,
    Large,
}

#[component]
pub fn Button(
    #[prop(into)] text: String,
    #[prop(optional)] icon: Option<&'static str>,
    #[prop(optional, default = ButtonVariant::Primary)] variant: ButtonVariant,
    #[prop(optional, default = ButtonSize::Medium)] size: ButtonSize,
    #[prop(optional, into)] on_click: Option<Box<dyn Fn(leptos::ev::MouseEvent) + Send + Sync>>,
    #[prop(optional, into)] disabled: Signal<bool>,
    #[prop(optional, into)] type_: Option<String>,
    #[prop(optional, into)] class: Option<String>,
) -> impl IntoView {
    let variant_class = match variant {
        ButtonVariant::Primary => style::primary,
        ButtonVariant::Outline => style::outline,
    };

    let size_class = match size {
        ButtonSize::Small => style::small,
        ButtonSize::Medium => style::medium,
        ButtonSize::Large => style::large,
    };

    let btn_type = type_.unwrap_or_else(|| "button".to_string());

    view! {
        <button
            type=btn_type
            class=format!("{} {} {} {}", style::btn, variant_class, size_class, class.unwrap_or_default())
            disabled=disabled
            on:click=move |ev| {
                if let Some(callback) = &on_click {
                    callback(ev);
                }
            }
        >
            <span>{text}</span>
            {move || icon.map(|i| view! {
                <span class=style::btn_icon>
                    <crate::components::ui::icon::Icon icon_name=i.to_string() />
                </span>
            })}
        </button>
    }
}
