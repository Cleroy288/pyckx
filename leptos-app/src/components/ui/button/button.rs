// ** button.rs **
// ==> Reusable Button with Primary/Outline variants

use leptos::prelude::*;
use leptos_router::hooks::use_navigate;

stylance::import_crate_style!(
    style,
    "src/components/ui/button/button.module.css"
);

/// Button visual variant
#[derive(Clone, Copy, PartialEq, Default)]
pub enum ButtonVariant {
    #[default]
    Primary,
    Outline,
}

/// Button size variant
#[derive(Clone, Copy, PartialEq, Default)]
pub enum ButtonSize {
    Small,
    #[default]
    Medium,
    Large,
}

/// Callback type for button click handlers
pub type ButtonCallback =
    Box<dyn Fn(leptos::ev::MouseEvent) + Send + Sync>;

/// Helper to create a button callback
pub fn btn_click<F>(f: F) -> ButtonCallback
where
    F: Fn(leptos::ev::MouseEvent)
        + Send
        + Sync
        + 'static,
{
    Box::new(f)
}

/// ButtonCallback that navigates to a path.
/// Must be called during component rendering
/// (inside Router context).
pub fn nav_click(
    path: impl Into<String>,
) -> ButtonCallback {
    let nav = use_navigate();
    let path = path.into();
    btn_click(move |_| {
        nav(&path, Default::default());
    })
}

/// Callback<()> that navigates to a path.
/// Must be called during component rendering
/// (inside Router context).
pub fn nav_callback(
    path: impl Into<String>,
) -> Callback<()> {
    let nav = use_navigate();
    let path = path.into();
    Callback::new(move |_: ()| {
        nav(&path, Default::default());
    })
}

/// CSS class string for a button variant + size
pub fn btn_class(
    variant: ButtonVariant,
    size: ButtonSize,
) -> String {
    let v = match variant {
        ButtonVariant::Primary => style::primary,
        ButtonVariant::Outline => style::outline,
    };
    let s = match size {
        ButtonSize::Small => style::small,
        ButtonSize::Medium => style::medium,
        ButtonSize::Large => style::large,
    };
    format!("{} {} {}", style::btn, v, s)
}

/// Reusable button component
#[component]
pub fn Button(
    /// Button label text
    #[prop(into)]
    text: String,
    /// Visual variant (Primary or Outline)
    #[prop(optional)]
    variant: ButtonVariant,
    /// Size variant
    #[prop(optional)]
    size: ButtonSize,
    /// Click handler
    #[prop(optional)]
    on_click: Option<ButtonCallback>,
    /// Disabled state
    #[prop(optional, into)]
    disabled: Signal<bool>,
    /// HTML type attribute
    #[prop(optional, into)]
    type_: Option<String>,
    /// Extra CSS class
    #[prop(optional, into)]
    class: Option<String>,
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
    let btn_type =
        type_.unwrap_or_else(|| "button".into());

    view! {
        <button
            type=btn_type
            class=format!(
                "{} {} {} {}",
                style::btn,
                variant_class,
                size_class,
                class.unwrap_or_default()
            )
            disabled=disabled
            on:click=move |ev| {
                if let Some(cb) = &on_click {
                    cb(ev);
                }
            }
        >
            {text}
        </button>
    }
}
