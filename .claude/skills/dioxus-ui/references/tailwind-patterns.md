# Tailwind CSS Patterns for Dioxus

Common Tailwind patterns used in the Pyckx project. Copy-paste these into your components.

---

## Layout Patterns

### Page Container
```rust
div { class: "min-h-screen flex flex-col",
    // Navbar, content, footer
}
```

### Centered Content
```rust
div { class: "max-w-4xl mx-auto px-4 py-8",
    // content
}
```

### Responsive Grid
```rust
div { class: "grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6",
    // cards
}
```

### Flex Row with Gap
```rust
div { class: "flex items-center gap-3",
    // inline items
}
```

### Flex Column
```rust
div { class: "flex flex-col gap-4",
    // stacked items
}
```

### Sidebar Layout
```rust
div { class: "flex min-h-screen",
    aside { class: "w-64 shrink-0 border-r border-white/10 p-4" }
    main { class: "flex-1 p-6" }
}
```

---

## Glass Morphism (Pyckx Signature)

### Glass Card
```rust
const GLASS_CARD: &str = "\
    bg-white/5 backdrop-blur-md \
    border border-white/10 \
    rounded-2xl shadow-xl \
    p-6";
```

### Glass Surface (lighter)
```rust
const GLASS_SURFACE: &str = "\
    bg-white/[0.03] backdrop-blur-sm \
    border border-white/[0.06] \
    rounded-xl";
```

### Glass Input
```rust
const GLASS_INPUT: &str = "\
    bg-white/5 border border-white/10 \
    rounded-xl px-4 py-3 \
    text-white placeholder-white/40 \
    focus:border-[var(--color-primary)] \
    focus:ring-1 focus:ring-[var(--color-primary)]/50 \
    outline-none transition-all duration-200";
```

---

## Button Styles

### Primary Button
```rust
const BTN_PRIMARY: &str = "\
    inline-flex items-center justify-center \
    px-6 py-3 rounded-xl font-semibold \
    bg-[var(--color-primary)] \
    text-[var(--color-background)] \
    hover:brightness-110 \
    transition-all duration-200 \
    focus:outline-none focus:ring-2 \
    focus:ring-[var(--color-primary)]/50";
```

### Ghost Button
```rust
const BTN_GHOST: &str = "\
    inline-flex items-center justify-center \
    px-4 py-2 rounded-lg \
    text-white/70 hover:text-white \
    hover:bg-white/10 \
    transition-all duration-200";
```

### Icon Button
```rust
const BTN_ICON: &str = "\
    p-2 rounded-lg \
    text-white/60 hover:text-white \
    hover:bg-white/10 \
    transition-all duration-200";
```

---

## Typography

### Page Title
```rust
h1 { class: "text-3xl font-bold text-white" }
```

### Gradient Title
```rust
h1 {
    class: "text-3xl font-bold \
            bg-gradient-to-r \
            from-[var(--color-primary)] \
            to-purple-400 \
            bg-clip-text text-transparent",
}
```

### Section Heading
```rust
h2 { class: "text-xl font-semibold text-white" }
```

### Body Text
```rust
p { class: "text-white/70 leading-relaxed" }
```

### Small / Caption
```rust
span { class: "text-sm text-white/50" }
```

---

## Form Patterns

### Form Group
```rust
div { class: "flex flex-col gap-1.5",
    label { class: "text-sm font-medium text-white/80",
        "Email"
    }
    input {
        class: GLASS_INPUT,
        r#type: "email",
        placeholder: "you@example.com",
        value: "{email}",
        oninput: move |e| email.set(e.value()),
    }
    // Error message
    if let Some(err) = error() {
        p { class: "text-sm text-red-400", "{err}" }
    }
}
```

### Form Layout
```rust
form {
    class: "flex flex-col gap-6 max-w-md",
    onsubmit: move |e| {
        e.prevent_default();
        spawn(async move { /* submit */ });
    },
    // form groups here
    Button { text: "Submit".into(), type_: "submit".into() }
}
```

---

## Card Patterns

### Clickable Card
```rust
div {
    class: "{GLASS_CARD} \
            cursor-pointer \
            hover:scale-[1.02] \
            hover:border-[var(--color-primary)]/30 \
            transition-all duration-200",
    onclick: move |_| { /* navigate */ },
}
```

### Card with Header + Body
```rust
div { class: GLASS_CARD,
    // Header
    div { class: "flex items-center justify-between mb-4",
        h3 { class: "text-lg font-semibold text-white",
            "Card Title"
        }
        Badge { text: "New".into() }
    }
    // Body
    p { class: "text-white/70", "Card content..." }
    // Footer
    div { class: "flex justify-end gap-2 mt-4",
        Button { text: "Cancel".into(), variant: ButtonVariant::Outline }
        Button { text: "Save".into() }
    }
}
```

---

## Modal / Overlay

### Backdrop
```rust
div {
    class: "fixed inset-0 z-50 \
            flex items-center justify-center \
            bg-black/60 backdrop-blur-sm",
    onclick: move |_| open.set(false),
}
```

### Modal Container
```rust
div {
    class: "relative w-full max-w-lg mx-4 \
            bg-[var(--color-surface)] \
            border border-white/10 \
            rounded-2xl shadow-2xl \
            p-6",
    onclick: move |e| e.stop_propagation(),
}
```

---

## Responsive Patterns

### Hide on Mobile
```rust
div { class: "hidden md:block" }
```

### Stack on Mobile, Row on Desktop
```rust
div { class: "flex flex-col md:flex-row gap-4" }
```

### Full Width on Mobile
```rust
div { class: "w-full md:w-auto" }
```

---

## Animation / Transition

### Hover Scale
```rust
class: "transition-transform duration-200 hover:scale-[1.02]"
```

### Hover Glow
```rust
class: "transition-shadow duration-200 \
        hover:shadow-[0_0_20px_var(--color-primary)/20]"
```

### Fade In (needs CSS keyframe)
```css
/* component.module.css */
@keyframes fadeIn {
    from { opacity: 0; transform: translateY(8px); }
    to { opacity: 1; transform: translateY(0); }
}
.fade-in { animation: fadeIn 0.3s ease-out; }
```

---

## Status / State Colors

```rust
// Success
class: "text-emerald-400 bg-emerald-500/10 border-emerald-500/20"

// Warning
class: "text-amber-400 bg-amber-500/10 border-amber-500/20"

// Error
class: "text-red-400 bg-red-500/10 border-red-500/20"

// Info
class: "text-blue-400 bg-blue-500/10 border-blue-500/20"
```

---

## Scrollable Container

```rust
div {
    class: "overflow-y-auto max-h-[60vh] \
            scrollbar-thin scrollbar-thumb-white/10",
}
```

---

## Loading States

### Skeleton Pulse
```rust
div { class: "animate-pulse bg-white/10 rounded-xl h-32" }
```

### Skeleton Text Lines
```rust
div { class: "flex flex-col gap-2",
    div { class: "animate-pulse bg-white/10 rounded h-4 w-3/4" }
    div { class: "animate-pulse bg-white/10 rounded h-4 w-1/2" }
}
```

---

## Spacing Reference

| Use case | Class | px |
|---|---|---|
| Tight | `gap-1` / `p-1` | 4px |
| Compact | `gap-2` / `p-2` | 8px |
| Default | `gap-4` / `p-4` | 16px |
| Comfortable | `gap-6` / `p-6` | 24px |
| Spacious | `gap-8` / `p-8` | 32px |
| Section | `gap-12` / `py-12` | 48px |
