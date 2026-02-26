//! Button — Reusable button with variant and size

use dioxus::prelude::*;

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

/// Base classes shared by all buttons
const BASE: &str = "\
    inline-flex items-center justify-center \
    font-semibold font-[var(--principal-font-family,sans-serif)] \
    border-none cursor-pointer no-underline leading-relaxed \
    transition-all duration-[var(--transition-base)] \
    focus-visible:outline-2 \
    focus-visible:outline-[var(--color-primary)] \
    focus-visible:outline-offset-2 \
    disabled:opacity-60 disabled:cursor-not-allowed";

/// Tailwind classes for a button variant
fn variant_class(v: ButtonVariant) -> &'static str {
    match v {
        ButtonVariant::Primary => "\
            text-[var(--color-background)] \
            bg-[var(--color-primary)] \
            hover:enabled:opacity-85 \
            active:enabled:opacity-70",
        ButtonVariant::Outline => "\
            bg-transparent \
            border border-[var(--color-primary)] \
            text-[var(--color-primary)] \
            hover:enabled:bg-[var(--color-primary)] \
            hover:enabled:text-[var(--color-background)] \
            active:enabled:opacity-80",
    }
}

/// Tailwind classes for a button size
fn size_class(s: ButtonSize) -> &'static str {
    match s {
        ButtonSize::Small => "py-2 px-5 text-sm",
        ButtonSize::Medium => "py-3 px-7 text-[0.9375rem]",
        ButtonSize::Large => "py-4 px-10 text-base",
    }
}

/// Build the full CSS class string for a button
pub fn btn_class(
    variant: ButtonVariant,
    size: ButtonSize,
) -> String {
    format!(
        "{} {} {}",
        BASE,
        variant_class(variant),
        size_class(size),
    )
}

/// Reusable button component
#[component]
pub fn Button(
    /// Button label text
    text: String,
    /// Visual variant (Primary or Outline)
    #[props(default)]
    variant: ButtonVariant,
    /// Size variant
    #[props(default)]
    size: ButtonSize,
    /// Click handler
    #[props(default)]
    on_click: EventHandler<MouseEvent>,
    /// Disabled state (accepts Signal or Memo)
    #[props(default = Signal::new(false))]
    disabled: ReadSignal<bool>,
    /// HTML type attribute
    #[props(default = "button".to_string())]
    type_: String,
    /// Extra CSS class
    #[props(default)]
    class: String,
) -> Element {
    let cls = format!(
        "{} {} {} {}",
        BASE,
        variant_class(variant),
        size_class(size),
        class,
    );

    rsx! {
        button {
            r#type: "{type_}",
            class: "{cls}",
            disabled: "{disabled}",
            onclick: move |ev| on_click.call(ev),
            "{text}"
        }
    }
}
