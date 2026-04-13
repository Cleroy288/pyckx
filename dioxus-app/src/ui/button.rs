//! Button — reusable button with variant and size.

use dioxus::prelude::*;

/// Visual style of a Button.
#[derive(Clone, Copy, PartialEq, Default)]
pub enum ButtonVariant {
    #[default]
    Primary,
    Outline,
    Destructive,
}

/// Size of a Button.
#[derive(Clone, Copy, PartialEq, Default)]
pub enum ButtonSize {
    Small,
    #[default]
    Medium,
    Large,
}

/// Reusable button component.
#[component]
pub fn Button(
    text: String,
    #[props(default)] variant: ButtonVariant,
    #[props(default)] size: ButtonSize,
    #[props(default)] on_click: EventHandler<MouseEvent>,
    #[props(default = Signal::new(false))] disabled: ReadSignal<bool>,
    #[props(default = "button".to_string())] type_: String,
    #[props(default)] class: String,
) -> Element {
    let full_class = build_class(variant, size, &class);
    rsx! {
        button {
            r#type: "{type_}",
            class: "{full_class}",
            disabled: "{disabled}",
            onclick: move |ev| on_click.call(ev),
            "{text}"
        }
    }
}

/// Build the full CSS class string for a Button.
pub fn btn_class(variant: ButtonVariant, size: ButtonSize) -> String {
    build_class(variant, size, "")
}

/// Compose base + variant + size + extra classes.
fn build_class(
    variant: ButtonVariant,
    size: ButtonSize,
    extra: &str,
) -> String {
    format!(
        "{BASE} {} {} {}",
        variant_class(variant),
        size_class(size),
        extra,
    )
}

/// Shared shell classes (shape, border, shadow).
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

/// Tailwind classes for a single variant.
fn variant_class(variant: ButtonVariant) -> &'static str {
    match variant {
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

/// Tailwind classes for a single size.
fn size_class(size: ButtonSize) -> &'static str {
    match size {
        ButtonSize::Small => "py-1.5 px-4 text-[11px]",
        ButtonSize::Medium => "py-2.5 px-6 text-[12px]",
        ButtonSize::Large => "py-3 px-8 text-[13px]",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_btn_class_primary_medium_has_base() {
        let cls = btn_class(
            ButtonVariant::Primary,
            ButtonSize::Medium,
        );
        assert!(cls.contains("inline-flex"));
    }

    #[test]
    fn test_build_class_appends_extra() {
        let cls = build_class(
            ButtonVariant::Outline,
            ButtonSize::Small,
            "extra-y",
        );
        assert!(cls.ends_with("extra-y"));
    }

    #[test]
    fn test_variant_class_destructive_uses_destructive_bg() {
        let cls = variant_class(ButtonVariant::Destructive);
        assert!(cls.contains("bg-[var(--destructive)]"));
    }

    #[test]
    fn test_size_class_large_uses_px_8() {
        let cls = size_class(ButtonSize::Large);
        assert!(cls.contains("px-8"));
    }
}
