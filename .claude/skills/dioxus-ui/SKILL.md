---
name: dioxus-ui
description: |
  Build beautiful, production-quality UIs with the Dioxus framework in Rust. Use this skill whenever the user wants to create, modify, or improve frontend components, pages, layouts, forms, or any visual element using Dioxus. Triggers include: any mention of "component", "page", "UI", "frontend", "view", "layout", "form", "button", "card", "modal", "input", "select", "toast", "navigation bar", "sidebar", "dashboard view", "landing page", or any request to build something visual in the Dioxus app. Also use when the user mentions "rsx", "tailwind", "styling", "CSS classes", "theme", "dark mode", "responsive", or asks to make something "look good" or "look better". Even if the user doesn't say "Dioxus" explicitly — if they're working in the dioxus-app crate or asking about frontend Rust code, use this skill.
---

# Dioxus UI — Building Beautiful Frontends in Rust

This skill teaches you how to build polished, consistent UIs with Dioxus (0.7+), Tailwind CSS, and CSS custom properties. It's tailored to the Pyckx project patterns but applies to any Dioxus app.

Read this file first for the core patterns. For detailed reference on specific topics, read the files in `references/`.

---

## I. The Mental Model

Dioxus is a reactive UI framework for Rust. Think of it as React, but in Rust with type safety. The core loop is:

1. **Components** are functions that return `Element`
2. **Signals** hold reactive state — when they change, the UI re-renders
3. **RSX** is the HTML-like macro (`rsx!`) that describes your UI
4. **Tailwind CSS** handles styling — utility classes directly in RSX
5. **CSS custom properties** enable runtime theming (dark mode, color palettes)

Every UI you build follows this flow: define domain types → create signals for state → write RSX with Tailwind classes → wire events to signal mutations.

---

## II. Project Structure

The Dioxus app follows a 4-layer architecture. Know where things go:

```
dioxus-app/src/
├── domain/          # Pure types — no Dioxus imports
├── api/             # HTTP calls — returns domain types
├── state/           # Signals, contexts, hooks
├── components/
│   ├── ui/          # Reusable generic components (button, card, modal…)
│   └── {feature}/   # Feature-specific components (qcm_player, dvd_form…)
├── pages/           # One file per route — wires state + components
├── hooks/           # Custom hooks (use_is_mobile, etc.)
├── routes.rs        # Route enum (all app routes)
├── main.rs          # Root component + context provider stack
└── lib.rs           # Public module exports
```

**Rules:**
- UI components (`components/ui/`) are generic and reusable. They receive data via props. No API calls.
- Feature components (`components/{feature}/`) can use signals and context.
- Pages (`pages/`) are thin wrappers: fetch data, pass to components. No business logic.
- Domain types (`domain/`) have zero Dioxus imports. Pure data + validation.

---

## III. Component Anatomy

Every Dioxus component follows this structure:

```rust
use dioxus::prelude::*;

/// Short description of what this component renders.
#[component]
pub fn MyComponent(
    // Required props first
    title: String,
    // Optional props with defaults
    #[props(default)]
    variant: MyVariant,
    #[props(default)]
    class: String,
    #[props(default)]
    on_click: EventHandler<MouseEvent>,
    // Children last
    children: Element,
) -> Element {
    // Derive CSS classes from props
    let css = format!("{BASE} {}", variant_class(&variant));

    rsx! {
        div { class: "{css} {class}",
            onclick: move |e| on_click.call(e),
            h2 { "{title}" }
            {children}
        }
    }
}
```

### Props Rules

| Pattern | When to use |
|---|---|
| `title: String` | Required prop — caller must provide |
| `#[props(default)]` | Optional with type's Default impl |
| `#[props(default = false)]` | Optional with explicit default |
| `#[props(default = Signal::new(false))]` | Reactive optional prop |
| `EventHandler<MouseEvent>` | Callback from child to parent |
| `ReadSignal<bool>` | Read-only reactive binding |
| `Signal<String>` | Two-way reactive binding |
| `children: Element` | Slot for child content |
| `Option<String>` | Truly optional (renders nothing if None) |

### Variant Pattern

Use enums to control visual variants. This is the standard pattern in Pyckx:

```rust
/// Visual variants for the button.
#[derive(Default, Clone, PartialEq)]
pub enum ButtonVariant {
    #[default]
    Primary,
    Outline,
    Destructive,
}

/// Size options for the button.
#[derive(Default, Clone, PartialEq)]
pub enum ButtonSize {
    Small,
    #[default]
    Medium,
    Large,
}
```

Then map variants to Tailwind class strings:

```rust
/// Base classes shared by all button variants.
const BASE: &str = "\
    inline-flex items-center justify-center \
    font-semibold rounded-xl \
    transition-all duration-200 \
    focus:outline-none focus:ring-2";

/// Returns variant-specific Tailwind classes.
fn variant_class(v: &ButtonVariant) -> &'static str {
    match v {
        ButtonVariant::Primary => "\
            bg-[var(--color-primary)] \
            text-[var(--color-background)] \
            hover:brightness-110",
        ButtonVariant::Outline => "\
            border border-white/20 \
            text-white/80 hover:bg-white/10",
        ButtonVariant::Destructive => "\
            bg-red-500/80 text-white \
            hover:bg-red-600",
    }
}
```

This keeps RSX clean — no long class strings inline.

---

## IV. Styling Strategy

### Tailwind CSS — The Primary Tool

All styling uses Tailwind utility classes in RSX. The setup is already configured:

```css
/* input.css */
@import "tailwindcss";
@source "./src/**/*.{rs,html,css}";
@import "./assets/global.css" layer(base);
```

Tailwind scans your `.rs` files for class names. Use them directly:

```rust
rsx! {
    div {
        class: "flex flex-col gap-4 p-6 \
                rounded-2xl bg-white/5 \
                border border-white/10",
        // content
    }
}
```

### CSS Custom Properties — For Theming

The Pyckx design system uses CSS custom properties (set by `PaletteProvider`):

| Variable | Purpose | Example value |
|---|---|---|
| `--color-primary` | Primary brand color | `oklch(0.75 0.18 280)` |
| `--color-background` | Page background | `oklch(0.15 0.02 280)` |
| `--color-surface` | Card/panel surface | `oklch(0.20 0.03 280)` |
| `--color-text` | Main text color | `oklch(0.95 0.01 280)` |
| `--glass-bg` | Glass morphism background | `rgba(...)` |
| `--shadow-card` | Card shadow | box-shadow value |

Use them in Tailwind with bracket notation:

```rust
class: "bg-[var(--color-primary)] text-[var(--color-background)]"
class: "border-[var(--color-primary)]/30"
class: "shadow-[var(--shadow-card)]"
```

### When to Use Custom CSS

Tailwind covers 95% of cases. Use a co-located `.css` file only for:
- Complex animations (`@keyframes`)
- Pseudo-elements (`:before`, `:after`)
- Grid layouts that need `grid-template-areas`

Name it `{component}.module.css` and place it next to the component.

### Glass Morphism — The Pyckx Signature Look

Many components use a "glass" effect. Here's the pattern:

```rust
const GLASS: &str = "\
    bg-white/5 backdrop-blur-md \
    border border-white/10 \
    rounded-2xl shadow-xl";
```

---

## V. State Management

### Signals — Local State

```rust
// Create mutable state
let mut count = use_signal(|| 0);

// Read in RSX (automatic tracking)
rsx! { p { "Count: {count}" } }

// Mutate in event handlers
button { onclick: move |_| count += 1 }
button { onclick: move |_| count.set(0) }
```

### Context — Global State

For state shared across the app (auth, theme, toasts):

```rust
// Provider component (wraps the tree)
#[component]
pub fn AuthProvider(children: Element) -> Element {
    let auth = AuthState {
        user: use_signal(|| None),
        is_loading: use_signal(|| true),
    };
    use_context_provider(|| auth);

    // Fetch session on mount
    use_effect(move || {
        spawn(async move {
            if let Some(u) = api::get_me().await {
                auth.user.set(Some(u));
            }
            auth.is_loading.set(false);
        });
    });

    rsx! { {children} }
}

// Consumer hook
pub fn use_auth() -> AuthState {
    use_context::<AuthState>()
}

// Usage in any component
let auth = use_auth();
if let Some(user) = auth.user() {
    rsx! { p { "Hello, {user.username}" } }
}
```

### Derived State — Memos

When a value is computed from other signals:

```rust
let items: Signal<Vec<Item>> = use_signal(|| vec![]);
let has_items: Memo<bool> = use_memo(move || !items().is_empty());
```

### Async Data Loading

Pattern for fetching data on component mount:

```rust
let mut data: Signal<Option<Vec<Item>>> = use_signal(|| None);
let mut loading = use_signal(|| true);

use_effect(move || {
    spawn(async move {
        match api::fetch_items().await {
            Ok(items) => data.set(Some(items)),
            Err(e) => log::error!("fetch failed: {e}"),
        }
        loading.set(false);
    });
});
```

---

## VI. RSX Patterns

### Conditional Rendering

```rust
rsx! {
    // Simple if
    if loading() {
        Spinner {}
    }

    // If-else
    if let Some(user) = auth.user() {
        p { "Welcome, {user.username}" }
    } else {
        p { "Please log in" }
    }

    // Optional element
    if let Some(title) = &title {
        h2 { class: "text-xl font-bold", "{title}" }
    }
}
```

### Iteration

```rust
rsx! {
    // Always use key for lists
    for item in items() {
        div { key: "{item.id}",
            ItemCard { item: item.clone() }
        }
    }
}
```

### Dynamic Classes

```rust
// Conditional class
class: if active() { "border-[var(--color-primary)]" }
         else { "border-white/10" },

// Multiple class attributes merge
div {
    class: "flex gap-2 p-4",
    class: if highlighted() { "bg-yellow-500/10" },
    class: "{extra_class}",
}
```

### Event Handling

```rust
// Click
button { onclick: move |_| count += 1, "Click" }

// Input (two-way binding)
input {
    value: "{value}",
    oninput: move |e| value.set(e.value()),
}

// Form submit
form {
    onsubmit: move |e| {
        e.prevent_default();
        spawn(async move { api::submit(data()).await; });
    },
}

// Stop propagation (modals)
div { onclick: move |e| e.stop_propagation() }
```

---

## VII. Routing

Routes are defined as an enum in `routes.rs`:

```rust
#[derive(Routable, Clone, PartialEq)]
pub enum Route {
    #[route("/")]
    Vitrine {},

    #[route("/login")]
    Login {},

    #[route("/qcm")]
    QcmList {},

    #[route("/qcm/play/:id")]
    QcmPlay { id: String },

    #[not_found]
    NotFound {},
}
```

Each variant maps to a component of the same name. Parameters (`:id`) become struct fields.

**Navigation:**

```rust
// Declarative (Link component)
use dioxus::router::prelude::*;
Link { to: Route::QcmPlay { id: "123".into() },
    "Play QCM"
}

// Programmatic
let nav = navigator();
nav.push(Route::Login {});

// Async-safe navigation (from spawn blocks)
let target = use_async_navigate();
spawn(async move {
    api::logout().await;
    target.set(Some("/login".into()));
});
```

---

## VIII. Reusable UI Components Catalog

Before building a new component, check `components/ui/`. These exist:

| Component | Path | What it does |
|---|---|---|
| `Button` | `ui/button/` | Primary, Outline, Destructive variants + sizes |
| `Card` | `ui/card/` | Glass, Solid, Popular variants |
| `Modal` | `ui/modal/` | Overlay dialog with backdrop |
| `Input` | `ui/input/` | Text input with label + error state |
| `Select` | `ui/select/` | Dropdown with options |
| `Textarea` | `ui/textarea/` | Multiline input |
| `Toast` | `ui/toast/` | Notification system (context-based) |
| `Spinner` | `ui/spinner/` | Loading indicator |
| `Skeleton` | `ui/skeleton/` | Loading placeholder |
| `Badge` | `ui/badge/` | Status labels |
| `Tabs` | `ui/tabs/` | Tab navigation |
| `Progress` | `ui/progress/` | Progress bar |
| `Icon` | `ui/icon/` | SVG icon wrapper |
| `FormField` | `ui/form_field/` | Label + input + error wrapper |
| `PageLayout` | `ui/page_layout/` | Page content wrapper |
| `HeroBanner` | `ui/hero_banner/` | Hero section |
| `EmptyState` | `ui/empty_state/` | Empty content placeholder |
| `FileUpload` | `ui/file_upload/` | File input |
| `LoadingBoundary` | `ui/loading_boundary.rs` | Loading/error wrapper |
| `ConfirmDialog` | `ui/modal/` | Confirmation modal |
| `SectionDivider` | `ui/section_divider/` | Visual separator |

**Always reuse these before creating new ones.**

---

## IX. Building a New Component — Step by Step

1. **Decide the layer.** Generic UI → `components/ui/{name}/`. Feature-specific → `components/{feature}/{name}.rs`.

2. **Create the folder and files:**
   ```
   components/ui/my_widget/
   ├── mod.rs          # pub mod my_widget; pub use my_widget::*;
   └── my_widget.rs    # The component
   ```

3. **Register in parent mod.rs:** Add `pub mod my_widget;` to `components/ui/mod.rs`.

4. **Write the component** following the anatomy in Section III.

5. **Style with Tailwind.** Use constants for repeated class groups. Use CSS custom properties for themed colors.

6. **Add to routes** (if it's a page) in `routes.rs`.

7. **Write a unit test** in a `#[cfg(test)] mod tests` block.

---

## X. Building a New Page — Step by Step

1. **Create the page file:**
   ```
   pages/{feature}/
   ├── mod.rs
   └── {feature}_page.rs
   ```

2. **Wire the route** in `routes.rs`:
   ```rust
   #[route("/my-feature")]
   MyFeature {},
   ```

3. **Page structure follows this template:**
   ```rust
   use dioxus::prelude::*;
   use crate::state::auth_hooks::use_auth_guard;
   use crate::components::ui::page_layout::PageLayout;

   /// Page description.
   #[component]
   pub fn MyFeaturePage() -> Element {
       use_auth_guard(); // if protected

       let mut data = use_signal(|| None);
       let mut loading = use_signal(|| true);

       use_effect(move || {
           spawn(async move {
               match api::fetch_data().await {
                   Ok(d) => data.set(Some(d)),
                   Err(e) => log::error!("{e}"),
               }
               loading.set(false);
           });
       });

       rsx! {
           HomeTopBar {}
           PageLayout {
               if loading() {
                   Spinner {}
               } else if let Some(d) = data() {
                   MyFeatureContent { data: d }
               } else {
                   EmptyState {
                       message: "No data found".into()
                   }
               }
           }
       }
   }
   ```

4. **Register** in `pages/mod.rs` and re-export.

---

## XI. Design Principles — Making Things Beautiful

### Color & Contrast
- Use the CSS custom properties (`--color-primary`, etc.) for brand colors.
- White text on dark backgrounds: `text-white` or `text-white/80` for secondary.
- Subtle borders: `border border-white/10` or `border-white/20`.

### Spacing & Layout
- Use `flex` and `gap` over manual margins: `flex flex-col gap-4`.
- Standard padding: `p-4` (compact), `p-6` (normal), `p-8` (spacious).
- Page max-width: `max-w-4xl mx-auto` or `max-w-6xl`.

### Typography
- Headings: `text-2xl font-bold` (page), `text-xl font-semibold` (section).
- Body text: default size, `text-white/70` for secondary.
- Gradient text: `bg-gradient-to-r from-[var(--color-primary)] to-purple-400 bg-clip-text text-transparent`.

### Depth & Surface
- Cards: glass morphism (`bg-white/5 backdrop-blur-md border border-white/10 rounded-2xl`).
- Shadows: `shadow-xl` for elevation, `shadow-[var(--shadow-card)]` for themed.
- Hover states: `hover:bg-white/10`, `hover:brightness-110`, `hover:scale-[1.02]`.

### Animation
- Transitions on everything interactive: `transition-all duration-200`.
- Scale on hover for cards: `hover:scale-[1.02]`.
- Fade-in for loaded content: use CSS `@keyframes` in a module CSS file.

### Responsive
- Mobile-first: default styles are mobile, add `md:` and `lg:` for larger.
- Grid: `grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6`.
- Hide on mobile: `hidden md:block`. Show on mobile: `md:hidden`.

---

## XII. Common Pitfalls

1. **Forgetting `key` on lists.** Always add `key: "{item.id}"` in `for` loops. Without it, Dioxus can't efficiently diff the list and you get state bugs.

2. **Mutating signals inside render.** Never call `.set()` directly in the RSX body — it causes infinite re-renders. Only mutate in event handlers or `use_effect`.

3. **Missing `move` on closures.** Event handler closures need `move` to capture signals: `onclick: move |_| count += 1`.

4. **Forgetting `spawn` for async.** Dioxus event handlers are sync. Wrap async calls: `spawn(async move { ... })`.

5. **Long class strings in RSX.** Extract to `const` or helper functions. Keeps RSX readable.

6. **Not checking existing components.** Always check `components/ui/` before building something new.

7. **Putting business logic in pages.** Pages are composition only. Logic goes in `domain/` or `api/`.

8. **Using `println!` instead of `log`.** In WASM, use `log::info!()`, `log::error!()` etc.

---

## XIII. Reference Files

For deeper dives, read these files in `references/`:

- `references/dioxus-api.md` — Dioxus 0.7 API reference: hooks, signals, effects, context, router
- `references/tailwind-patterns.md` — Common Tailwind patterns for Dioxus: responsive, dark mode, animations, forms
- `references/component-templates.md` — Copy-paste templates for common component types

---

## XIV. Quick Checklist Before Delivering UI Code

- [ ] Component follows the standard anatomy (Section III)
- [ ] Props use `#[props(default)]` for optional values
- [ ] Variant enums drive styling (not if/else in RSX)
- [ ] Tailwind classes use CSS custom properties for themed colors
- [ ] No inline magic strings — constants for repeated class groups
- [ ] Lists have `key` attributes
- [ ] Async calls wrapped in `spawn`
- [ ] Existing UI components reused (check Section VIII catalog)
- [ ] File placed in correct layer (components/ui/ vs components/{feature}/ vs pages/)
- [ ] Registered in parent `mod.rs`
- [ ] Unit test written
