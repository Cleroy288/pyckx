//! Button — Reusable button with variant and size

use dioxus::prelude::*;

/// Button visual variant
#[derive(Clone, Copy, PartialEq, Default)]
pub enum ButtonVariant {
    #[default]
    Primary,
    Outline,
    Destructive,
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
    font-['Unbounded',sans-serif] font-bold \
    border-2 border-[var(--foreground)] \
    rounded-[10px] cursor-pointer \
    no-underline leading-relaxed \
    shadow-[3px_3px_0_var(--foreground)] \
    transition-all duration-150 ease-out \
    active:translate-y-[1px] \
    active:shadow-[2px_2px_0_var(--foreground)] \
    focus-visible:outline-2 \
    focus-visible:outline-[var(--primary)] \
    focus-visible:outline-offset-2 \
    disabled:opacity-60 \
    disabled:cursor-not-allowed";

/// Tailwind classes for a button variant
fn variant_class(v: ButtonVariant) -> &'static str {
    match v {
        ButtonVariant::Primary => "\
            text-[var(--primary-foreground)] \
            bg-[var(--primary)] \
            hover:-translate-y-0.5 \
            hover:shadow-[4px_4px_0_var(--foreground)]",
        ButtonVariant::Outline => "\
            bg-[var(--card)] \
            text-[var(--foreground)] \
            hover:-translate-y-0.5 \
            hover:shadow-[4px_4px_0_var(--foreground)]",
        ButtonVariant::Destructive => "\
            text-[var(--primary-foreground)] \
            bg-[var(--destructive)] \
            hover:-translate-y-0.5 \
            hover:shadow-[4px_4px_0_var(--foreground)]",
    }
}

/// Tailwind classes for a button size
fn size_class(s: ButtonSize) -> &'static str {
    match s {
        ButtonSize::Small => "\
            py-1.5 px-4 text-[11px]",
        ButtonSize::Medium => "\
            py-2.5 px-6 text-[12px]",
        ButtonSize::Large => "\
            py-3 px-8 text-[13px]",
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
