# Component Templates for Dioxus

Copy-paste starter templates for common component types. Adapt to your needs.

---

## Template 1: Simple UI Component

A basic reusable component with variants and optional props.

```rust
use dioxus::prelude::*;

/// Visual variants for the widget.
#[derive(Default, Clone, PartialEq)]
pub enum WidgetVariant {
    #[default]
    Default,
    Accent,
}

/// Base Tailwind classes shared across variants.
const BASE: &str = "\
    flex items-center gap-2 \
    rounded-xl px-4 py-2 \
    transition-all duration-200";

/// Returns variant-specific classes.
fn variant_class(v: &WidgetVariant) -> &'static str {
    match v {
        WidgetVariant::Default => "\
            bg-white/5 border border-white/10 \
            text-white/80",
        WidgetVariant::Accent => "\
            bg-[var(--color-primary)]/10 \
            border border-[var(--color-primary)]/20 \
            text-[var(--color-primary)]",
    }
}

/// A reusable widget component.
#[component]
pub fn Widget(
    text: String,
    #[props(default)]
    variant: WidgetVariant,
    #[props(default)]
    class: String,
) -> Element {
    let css = format!(
        "{BASE} {} {class}",
        variant_class(&variant)
    );
    rsx! {
        div { class: "{css}", "{text}" }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_variant_class_default_returns_white() {
        let result = variant_class(&WidgetVariant::Default);
        assert!(result.contains("bg-white"));
    }

    #[test]
    fn test_variant_class_accent_returns_primary() {
        let result = variant_class(&WidgetVariant::Accent);
        assert!(result.contains("--color-primary"));
    }
}
```

### File structure:
```
components/ui/widget/
├── mod.rs        → pub mod widget; pub use widget::*;
└── widget.rs     → the code above
```

---

## Template 2: Form Input Component

Two-way bound input with label, error state, and validation.

```rust
use dioxus::prelude::*;

/// Styled text input with label and error.
#[component]
pub fn FormInput(
    id: String,
    input_type: String,
    label: String,
    placeholder: String,
    value: Signal<String>,
    on_input: EventHandler<String>,
    #[props(default = Signal::new(None))]
    error: Signal<Option<String>>,
    #[props(default = false)]
    required: bool,
) -> Element {
    /// Base input classes.
    const INPUT: &str = "\
        w-full bg-white/5 border rounded-xl \
        px-4 py-3 text-white \
        placeholder-white/40 outline-none \
        transition-all duration-200";
    /// Normal state border.
    const NORMAL: &str = "\
        border-white/10 \
        focus:border-[var(--color-primary)] \
        focus:ring-1 \
        focus:ring-[var(--color-primary)]/50";
    /// Error state border.
    const ERR: &str = "\
        border-red-500/50 bg-red-500/5";

    let border = if error().is_some() { ERR } else { NORMAL };

    rsx! {
        div { class: "flex flex-col gap-1.5",
            label {
                r#for: "{id}",
                class: "text-sm font-medium text-white/80",
                "{label}"
            }
            input {
                id: "{id}",
                r#type: "{input_type}",
                class: "{INPUT} {border}",
                placeholder: "{placeholder}",
                value: "{value}",
                required: required,
                oninput: move |e| on_input.call(e.value()),
            }
            if let Some(err) = error() {
                p {
                    class: "text-sm text-red-400",
                    "{err}"
                }
            }
        }
    }
}
```

---

## Template 3: Card with Actions

A card that displays data and has action buttons.

```rust
use dioxus::prelude::*;

/// Displays an item with edit and delete actions.
#[component]
pub fn ItemCard(
    title: String,
    description: String,
    on_edit: EventHandler<()>,
    on_delete: EventHandler<()>,
    #[props(default)]
    class: String,
) -> Element {
    /// Glass card base.
    const CARD: &str = "\
        bg-white/5 backdrop-blur-md \
        border border-white/10 rounded-2xl \
        p-5 flex flex-col gap-3 \
        hover:border-white/20 \
        transition-all duration-200";

    rsx! {
        div { class: "{CARD} {class}",
            // Header
            h3 {
                class: "text-lg font-semibold text-white",
                "{title}"
            }
            // Body
            p {
                class: "text-white/60 text-sm \
                        leading-relaxed line-clamp-3",
                "{description}"
            }
            // Actions
            div { class: "flex justify-end gap-2 mt-auto pt-2",
                button {
                    class: "px-3 py-1.5 rounded-lg \
                            text-sm text-white/60 \
                            hover:text-white \
                            hover:bg-white/10 \
                            transition-all duration-200",
                    onclick: move |_| on_edit.call(()),
                    "Edit"
                }
                button {
                    class: "px-3 py-1.5 rounded-lg \
                            text-sm text-red-400 \
                            hover:bg-red-500/10 \
                            transition-all duration-200",
                    onclick: move |_| on_delete.call(()),
                    "Delete"
                }
            }
        }
    }
}
```

---

## Template 4: Modal Dialog

Overlay modal with backdrop dismiss.

```rust
use dioxus::prelude::*;

/// Overlay modal dialog.
#[component]
pub fn Dialog(
    open: Signal<bool>,
    title: String,
    children: Element,
    #[props(default)]
    on_close: EventHandler<()>,
) -> Element {
    if !open() {
        return rsx! {};
    }

    /// Handles closing the dialog.
    let close = move |_| {
        open.set(false);
        on_close.call(());
    };

    rsx! {
        // Backdrop
        div {
            class: "fixed inset-0 z-50 \
                    flex items-center justify-center \
                    bg-black/60 backdrop-blur-sm",
            onclick: close,

            // Modal
            div {
                class: "relative w-full max-w-lg mx-4 \
                        bg-[var(--color-surface)] \
                        border border-white/10 \
                        rounded-2xl shadow-2xl p-6",
                onclick: move |e| e.stop_propagation(),

                // Header
                div {
                    class: "flex items-center \
                            justify-between mb-4",
                    h2 {
                        class: "text-xl font-semibold \
                                text-white",
                        "{title}"
                    }
                    button {
                        class: "p-1 rounded-lg \
                                text-white/40 \
                                hover:text-white \
                                hover:bg-white/10",
                        onclick: close,
                        "✕"
                    }
                }

                // Body
                {children}
            }
        }
    }
}
```

---

## Template 5: List Page

Page that fetches and displays a list of items.

```rust
use dioxus::prelude::*;
use crate::components::ui::page_layout::PageLayout;
use crate::components::ui::spinner::Spinner;
use crate::components::ui::empty_state::EmptyState;

/// Page listing all items in a feature.
#[component]
pub fn ItemListPage() -> Element {
    let mut items: Signal<Vec<Item>> = use_signal(|| vec![]);
    let mut loading = use_signal(|| true);

    use_effect(move || {
        spawn(async move {
            match api::fetch_items().await {
                Ok(data) => items.set(data),
                Err(e) => log::error!("{e}"),
            }
            loading.set(false);
        });
    });

    rsx! {
        PageLayout {
            // Header
            div {
                class: "flex items-center \
                        justify-between mb-6",
                h1 {
                    class: "text-2xl font-bold text-white",
                    "My Items"
                }
                Button {
                    text: "New Item".into(),
                    on_click: move |_| {
                        navigator().push(Route::NewItem {});
                    },
                }
            }
            // Content
            if loading() {
                Spinner {}
            } else if items().is_empty() {
                EmptyState {
                    message: "No items yet".into()
                }
            } else {
                div {
                    class: "grid grid-cols-1 \
                            md:grid-cols-2 \
                            lg:grid-cols-3 gap-6",
                    for item in items() {
                        ItemCard {
                            key: "{item.id}",
                            title: item.name.clone(),
                            description: item.desc.clone(),
                            on_edit: move |_| {},
                            on_delete: move |_| {},
                        }
                    }
                }
            }
        }
    }
}
```

---

## Template 6: Context Provider

Global state with context provider pattern.

```rust
use dioxus::prelude::*;

/// State shared across the app.
#[derive(Clone, Copy)]
pub struct FeatureState {
    /// Current items.
    pub items: Signal<Vec<Item>>,
    /// Whether data is loading.
    pub loading: Signal<bool>,
}

impl FeatureState {
    /// Refreshes items from the API.
    pub fn refresh(&self) {
        let mut items = self.items;
        let mut loading = self.loading;
        spawn(async move {
            loading.set(true);
            if let Ok(data) = api::fetch_items().await {
                items.set(data);
            }
            loading.set(false);
        });
    }
}

/// Provides FeatureState to the subtree.
#[component]
pub fn FeatureProvider(children: Element) -> Element {
    let state = FeatureState {
        items: use_signal(|| vec![]),
        loading: use_signal(|| true),
    };
    use_context_provider(|| state);

    // Initial fetch
    use_effect(move || { state.refresh(); });

    rsx! { {children} }
}

/// Reads the FeatureState from context.
pub fn use_feature() -> FeatureState {
    use_context::<FeatureState>()
}
```
