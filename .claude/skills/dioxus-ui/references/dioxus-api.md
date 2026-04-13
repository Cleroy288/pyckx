# Dioxus 0.7 API Reference

Quick reference for the most-used Dioxus APIs. Consult the official docs at https://dioxuslabs.com/learn/0.7/ for full details.

---

## Hooks

| Hook | Purpose | Signature |
|---|---|---|
| `use_signal` | Create reactive state | `use_signal(\|\| initial) -> Signal<T>` |
| `use_memo` | Derived/computed value | `use_memo(move \|\| expr) -> Memo<T>` |
| `use_effect` | Side effect on mount/change | `use_effect(move \|\| { ... })` |
| `use_context` | Read context from ancestor | `use_context::<T>() -> T` |
| `use_context_provider` | Provide context to subtree | `use_context_provider(\|\| value)` |
| `use_hook` | Raw state (no reactivity) | `use_hook(\|\| value) -> &T` |

### use_signal

```rust
// Create
let mut count = use_signal(|| 0);

// Read (in RSX or closures — automatically tracked)
let value = count();

// Write
count.set(42);
count += 1;
*count.write() = 42;

// Read without tracking
let value = count.peek();
```

### use_effect

Runs on mount and whenever tracked signals change:

```rust
use_effect(move || {
    log::info!("count changed to {}", count());
});

// For async work, use spawn inside effect
use_effect(move || {
    spawn(async move {
        let data = api::fetch().await;
        items.set(data);
    });
});
```

### use_memo

Creates a derived signal that recalculates only when dependencies change:

```rust
let filtered = use_memo(move || {
    items()
        .into_iter()
        .filter(|i| i.active)
        .collect::<Vec<_>>()
});
```

---

## Context API

```rust
// 1. Define state struct
#[derive(Clone, Copy)]
pub struct ThemeState {
    pub dark: Signal<bool>,
}

// 2. Provide at root
#[component]
pub fn ThemeProvider(children: Element) -> Element {
    let state = ThemeState {
        dark: use_signal(|| false),
    };
    use_context_provider(|| state);
    rsx! { {children} }
}

// 3. Consume anywhere below
pub fn use_theme() -> ThemeState {
    use_context::<ThemeState>()
}
```

---

## Global Signals

For simple global state that doesn't need a provider:

```rust
static COUNT: GlobalSignal<i32> = Signal::global(|| 0);

fn App() -> Element {
    rsx! {
        p { "Count: {COUNT}" }
        button { onclick: move |_| *COUNT.write() += 1 }
    }
}
```

---

## Router

### Route Definition

```rust
use dioxus::prelude::*;

#[derive(Routable, Clone, PartialEq)]
pub enum Route {
    // Simple route
    #[route("/")]
    Home {},

    // Route with parameter
    #[route("/user/:id")]
    UserProfile { id: String },

    // Layout wrapper
    #[layout(DashboardLayout)]
        #[route("/dashboard")]
        Dashboard {},
        #[route("/settings")]
        Settings {},
    #[end_layout]

    // Catch-all / 404
    #[route("/:..segments")]
    NotFound { segments: Vec<String> },
}
```

### Navigation

```rust
// Declarative
Link { to: Route::Home {},
    "Go Home"
}

// Programmatic
let nav = navigator();
nav.push(Route::UserProfile { id: "42".into() });

// From async context (Pyckx pattern)
let target = use_async_navigate();
spawn(async move {
    api::do_something().await;
    target.set(Some("/home".into()));
});
```

### Layouts

```rust
#[component]
fn DashboardLayout() -> Element {
    rsx! {
        div { class: "flex",
            Sidebar {}
            main { class: "flex-1",
                Outlet::<Route> {}  // renders active child
            }
        }
    }
}
```

---

## Events

| Event | Type | Common use |
|---|---|---|
| `onclick` | `MouseEvent` | Button clicks |
| `oninput` | `FormEvent` | Text input changes |
| `onsubmit` | `FormEvent` | Form submission |
| `onchange` | `FormEvent` | Select/checkbox changes |
| `onkeydown` | `KeyboardEvent` | Key presses |
| `onfocus` / `onblur` | `FocusEvent` | Focus management |
| `onmouseenter` / `onmouseleave` | `MouseEvent` | Hover effects |

```rust
// Get input value
oninput: move |e: FormEvent| {
    value.set(e.value());
}

// Prevent default
onsubmit: move |e: FormEvent| {
    e.prevent_default();
}

// Stop propagation
onclick: move |e: MouseEvent| {
    e.stop_propagation();
}
```

---

## Async Patterns

```rust
// Spawn async task from event handler
onclick: move |_| {
    spawn(async move {
        let result = api::save(data()).await;
        match result {
            Ok(_) => toast.success("Saved!".into()),
            Err(e) => toast.error(e),
        }
    });
}

// Resource-like pattern (fetch on mount)
let mut items = use_signal(|| Vec::new());
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
```

---

## JS Interop

```rust
use wasm_bindgen::prelude::*;

// Eval JS
js_sys::eval("console.log('hello')");

// Web APIs via web-sys
let window = web_sys::window().unwrap();
let storage = window.local_storage().unwrap().unwrap();
storage.set_item("key", "value").unwrap();

// DOM manipulation
let doc = window.document().unwrap();
let root = doc.document_element().unwrap();
root.style().set_property("--color-primary", "red");
```

---

## Component Children & Slots

```rust
// Single slot (children)
#[component]
fn Wrapper(children: Element) -> Element {
    rsx! {
        div { class: "wrapper", {children} }
    }
}

// Named slots (via props)
#[component]
fn Layout(
    header: Element,
    children: Element,
    #[props(default)]
    footer: Option<Element>,
) -> Element {
    rsx! {
        header { {header} }
        main { {children} }
        if let Some(f) = footer {
            footer { {f} }
        }
    }
}

// Usage
Layout {
    header: rsx! { h1 { "Title" } },
    footer: rsx! { p { "Footer" } },
    p { "Main content" }
}
```
