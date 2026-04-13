//! Card family — Card, GameCard, NavCard, ItemCard, CardGrid.

use super::icon::Icon;
use dioxus::prelude::*;

/// Visual variant for the generic `Card`.
#[derive(Clone, Copy, PartialEq, Default)]
pub enum CardVariant {
    #[default]
    Glass,
    Solid,
    Popular,
}

/// Generic card wrapper with a variant.
#[component]
pub fn Card(
    children: Element,
    #[props(default)] variant: CardVariant,
    #[props(default)] class: String,
) -> Element {
    let full_class = format!(
        "{CARD_BASE} {} {}",
        card_variant_class(variant),
        class,
    );
    rsx! { div { class: "{full_class}", {children} } }
}

/// Brutalist game card (featured variant adds accent).
/// Rendered as `<button>` when `on_click` is set.
#[component]
pub fn GameCard(
    tag: String,
    name: String,
    desc: String,
    #[props(default = false)] featured: bool,
    #[props(optional)] on_click: Option<EventHandler<()>>,
) -> Element {
    let card_class = if featured { "gcard gcard-feat" } else { "gcard" };
    let inner = game_card_inner(&tag, &name, &desc, featured);
    match on_click {
        Some(handler) => rsx! {
            button {
                class: "{card_class}",
                onclick: move |_| handler.call(()),
                {inner}
            }
        },
        None => rsx! { div { class: "{card_class}", {inner} } },
    }
}

/// Clickable navigation tile (icon + title + description).
#[component]
pub fn NavCard(
    title: String,
    description: String,
    icon: String,
    on_click: EventHandler<()>,
) -> Element {
    rsx! {
        button {
            class: "{NAV_CARD_CLS}",
            onclick: move |_| on_click.call(()),
            div { class: "{NAV_ICON_CLS}",
                Icon { icon_name: icon.clone() }
            }
            span { class: "text-[1.1rem] font-semibold", "{title}" }
            span {
                class: "text-[0.85rem] \
                    text-[var(--color-text-secondary)] \
                    leading-relaxed",
                "{description}"
            }
        }
    }
}

/// Brutalist card for list items — header, body, actions.
#[component]
pub fn ItemCard(
    name: String,
    header_end: Element,
    actions: Element,
    children: Element,
) -> Element {
    rsx! {
        div { class: "{ITEM_CARD_CLS}",
            div { class: "flex items-center justify-between gap-3",
                h3 { class: "{ITEM_TITLE_CLS}", "{name}" }
                {header_end}
            }
            div { class: "flex flex-col gap-2 flex-1", {children} }
            div { class: "flex gap-3 mt-auto pt-2", {actions} }
        }
    }
}

/// Responsive 3-column grid for card lists.
#[component]
pub fn CardGrid(children: Element) -> Element {
    rsx! {
        div {
            class: "grid \
                grid-cols-[repeat(auto-fill,minmax(min(300px,100%),1fr))] \
                auto-rows-fr gap-5",
            {children}
        }
    }
}

/// Inner content rendered inside a GameCard.
fn game_card_inner(
    tag: &str,
    name: &str,
    desc: &str,
    featured: bool,
) -> Element {
    let body = rsx! {
        span { class: "gcard-tag", "{tag}" }
        h3 { "{name}" }
        p { "{desc}" }
        span { class: "gcard-arrow", "\u{2192}" }
    };
    if featured {
        rsx! { div { class: "gcard-feat-content", {body} } }
    } else {
        body
    }
}

/// Tailwind classes for a single `CardVariant`.
fn card_variant_class(variant: CardVariant) -> &'static str {
    match variant {
        CardVariant::Glass | CardVariant::Solid => "\
            hover:border-[var(--color-primary)]",
        CardVariant::Popular => "\
            border-[var(--color-primary)] \
            hover:-translate-y-1",
    }
}

/// Shell classes for the generic `Card`.
const CARD_BASE: &str = "\
    relative p-10 px-8 overflow-hidden \
    bg-[var(--card)] rounded-[12px] \
    border-[1.5px] border-[var(--border)] \
    transition-[transform,border-color] \
    duration-200 ease-out \
    hover:-translate-y-0.5";

/// Shell classes for the `NavCard`.
const NAV_CARD_CLS: &str = "\
    flex flex-col items-center gap-3 \
    py-8 px-6 w-full \
    bg-[var(--card)] \
    border-[1.5px] border-[var(--border)] \
    rounded-[12px] \
    text-[var(--color-text-primary)] \
    cursor-pointer font-[inherit] \
    transition-all duration-200 \
    hover:-translate-y-0.5 \
    hover:border-[var(--color-primary)]";

/// Icon tile classes for `NavCard`.
const NAV_ICON_CLS: &str = "\
    w-14 h-14 flex items-center justify-center \
    rounded-[12px] \
    bg-[var(--secondary)] \
    text-[var(--color-primary)] \
    transition-[background] \
    duration-[var(--transition-fast)] \
    [&_svg]:w-7 [&_svg]:h-7";

/// Shell classes for the `ItemCard`.
const ITEM_CARD_CLS: &str = "\
    item-card \
    flex flex-col gap-4 \
    p-6 h-full \
    bg-[var(--card)] \
    border-2 border-[var(--foreground)] \
    rounded-xl overflow-hidden \
    relative cursor-pointer \
    shadow-[4px_4px_0_var(--foreground)] \
    transition-all duration-200 \
    hover:-translate-y-1 \
    hover:shadow-[6px_6px_0_var(--foreground)]";

/// Title classes for the `ItemCard` header.
const ITEM_TITLE_CLS: &str = "\
    item-card-title \
    m-0 text-[1.125rem] \
    text-[var(--foreground)] \
    overflow-hidden text-ellipsis \
    whitespace-nowrap";

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_card_variant_class_popular_has_primary_border() {
        let cls = card_variant_class(CardVariant::Popular);
        assert!(cls.contains("border-[var(--color-primary)]"));
    }

    #[test]
    fn test_card_variant_class_glass_has_hover_border() {
        let cls = card_variant_class(CardVariant::Glass);
        assert!(cls.contains("hover:border-[var(--color-primary)]"));
    }
}
