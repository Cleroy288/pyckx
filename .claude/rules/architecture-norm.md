# Project Architecture Norm — Leptos Frontend Architecture

This document defines a **strict** project structure standard adapted for Leptos frontend applications. It draws from Clean Architecture principles, applied to a reactive frontend context with Leptos signals, components, and server functions.

---

## I. The Dependency Rule

**Dependencies point inward. Always. No exceptions.**

```
[Pages / Routes] → [Components] → [Services / State] → [API / Infrastructure]
       ↓                 ↓                ↓
  [URL params]     [Props/Signals]   [Domain types]
```

- **Inner layers know nothing about outer layers.** Domain types don't import components. Services don't import pages.
- **Communication across boundaries happens through props, signals, and typed interfaces.**
- **The entry point (`main.rs`) is the only place that wires routes together.**

---

## II. The Four Layers

Every Leptos project has exactly four logical layers. They map to folders/modules in the source tree.

### 1. Domain / State (innermost)

The core of the application. Pure business logic and shared state definitions. **Zero framework dependencies.**

Contains:
- **Domain types / Models** — Business objects with their rules and invariants. No Leptos imports, no API-specific annotations. Pure data + validation.
- **Value Objects** — Immutable types that represent a concept (e.g., `Email`, `Score`, `DateRange`).
- **Domain Errors** — Business-specific error types (e.g., `InvalidInput`, `Unauthorized`).
- **Shared state types** — Types used for app-wide state (auth context, user preferences).
- **Validation logic** — Pure functions that validate domain rules.

Rules:
- **No imports from any other layer.**
- **No Leptos framework code.** No signals, no components, no views.
- **No I/O.** No HTTP calls, no local storage access.
- Types must be **testable in complete isolation** with zero setup.

### 2. Services / API (data & orchestration)

Handles communication with external systems and orchestrates data flow.

Contains:
- **API client modules** — HTTP calls to backend APIs. One module per domain area (e.g., `api/auth.rs`, `api/intello.rs`).
- **Server functions** — Leptos `#[server]` functions for SSR data loading.
- **Data transformation** — Convert API responses to domain types and vice versa.
- **Service logic** — Business workflows that coordinate multiple API calls or state updates.

Rules:
- **Imports domain only.** Never imports components or pages.
- **Returns domain types**, not raw API response types. Map at this layer.
- **Handles errors and maps them to domain errors.**
- **One module per domain area.** If a module grows beyond 5 public functions, split it.

### 3. Components (presentation)

Reusable UI building blocks. The visual layer of the application.

Contains:
- **UI components** — Buttons, inputs, modals, cards, etc. In `components/ui/`.
- **Feature components** — Domain-specific components that combine UI pieces. In `components/{feature}/`.
- **Layout components** — Page layouts, navigation, sidebars.

Rules:
- **Presentational components are pure.** They receive data through props and emit events through callbacks. No direct API calls.
- **Container/feature components may use signals and resources** to bridge state → presentation.
- **One component per file** when practical. Related sub-components can share a file if small.
- **Components import domain types** for their props. Never import page-level code.
- **Reuse existing UI components.** Don't rebuild what already exists in `components/ui/`.

### 4. Pages / Routes (outermost)

Route-bound page components that wire everything together.

Contains:
- **Page components** — One per route. In `pages/{feature}/`.
- **Route definitions** — URL-to-page mapping.
- **Page-level state** — `create_resource`, `create_signal`, `create_action` for page-specific data.
- **Data loading** — Fetch data needed for the page, pass it to components.

Rules:
- **Pages are composition, not logic.** A page wires services, state, and components together. Business logic lives in services.
- **One page file per route.** Complex pages may have a folder with sub-modules.
- **Pages handle URL parameters and query strings.**
- **Pages handle loading states and error states** for their data.
- **No reusable UI logic in pages.** If something could be reused, extract it to a component.

---

## III. Canonical Folder Structure

```
leptos-app/
│
├── src/
│   ├── main.rs                    # Entry point: mount app, configure routes
│   │
│   ├── domain/                    # Layer 1: Domain types & business logic
│   │   ├── mod.rs
│   │   ├── types.rs               # Shared domain types (User, Session, etc.)
│   │   ├── errors.rs              # Domain-specific error enum
│   │   └── validation.rs          # Pure validation functions
│   │
│   ├── api/                       # Layer 2: Services & API clients
│   │   ├── mod.rs
│   │   ├── auth.rs                # Auth API calls
│   │   ├── intello.rs             # Intello API calls
│   │   └── client.rs              # Shared HTTP client setup
│   │
│   ├── components/                # Layer 3: UI Components
│   │   ├── mod.rs
│   │   ├── ui/                    # Generic reusable UI components
│   │   │   ├── mod.rs
│   │   │   ├── button/
│   │   │   ├── icon/
│   │   │   ├── select/
│   │   │   └── textarea/
│   │   ├── dashboard/             # Feature: dashboard components
│   │   └── intello/               # Feature: intello components
│   │
│   ├── pages/                     # Layer 4: Route-bound pages
│   │   ├── mod.rs
│   │   ├── home/
│   │   │   ├── home.rs
│   │   │   └── home.module.css
│   │   ├── login/
│   │   │   └── login.rs
│   │   └── intello/
│   │       └── intello.rs
│   │
│   └── state/                     # App-wide reactive state (optional)
│       ├── mod.rs
│       └── auth.rs                # Auth context provider
│
├── styles/                        # Global CSS
│   └── bundle.css
│
├── tests/                         # Integration tests
│   └── ...
│
├── Cargo.toml
├── Trunk.toml
└── .claude/
```

---

## IV. File Composition Rules

Each file has a **single responsibility**. Here is what goes where:

| File type | Contains | Does NOT contain |
|---|---|---|
| `domain/types` | Business objects, validation rules | API calls, Leptos components, signals |
| `domain/errors` | Domain-specific error enum | HTTP status codes, framework errors |
| `api/{area}` | HTTP calls, response mapping | Business logic, UI rendering |
| `components/ui/*` | Reusable UI elements, props | Direct API calls, page-level state |
| `components/{feature}/*` | Feature-specific UI, may use signals | Route handling, URL parsing |
| `pages/{feature}/*` | Route composition, data loading | Reusable UI logic, raw API calls |
| `state/*` | Context providers, app-wide signals | UI rendering, API calls |
| `main.rs` | App mount, route definitions | Business logic, UI components |

---

## V. Component Design Rules

### A. Props

1. **Props are structs or individual parameters.** Use `#[component]` macro with explicit prop types.
2. **Optional props use `#[prop(optional)]` or `Option<T>`.** Never use magic default values.
3. **Callback props use `Callback<T>` or closures.** For child-to-parent communication.
4. **Props are the component's API.** Document them with `///` comments.

### B. Signals & State

1. **Prefer derived signals over redundant state.** If a value can be computed from other signals, derive it.
2. **Minimize signal scope.** Create signals at the lowest level that needs them.
3. **Use context for app-wide state** (auth, theme, locale). Provide at root, consume where needed.
4. **Never mutate parent state directly.** Use callbacks passed as props.

### C. Composition

1. **Small components.** If a component exceeds 40 lines of view logic, split it.
2. **Children via `children` prop** for wrapper/layout components.
3. **Slots via named props** for multi-slot layouts.
4. **Extract repeated patterns** into reusable components.

---

## VI. Cross-Cutting Concerns

| Concern | Where it lives |
|---|---|
| Routing | `main.rs` — route definitions with `<Router>` |
| Authentication state | `state/auth.rs` — context provider at app root |
| API client config | `api/client.rs` — base URL, headers, interceptors |
| Error display | `components/ui/` — generic error display component |
| Loading states | `components/ui/` — generic loading/skeleton component |
| CSS styles | Co-located `.module.css` files or `styles/` for global |
| Form validation | `domain/validation.rs` — pure functions, used by components |

---

## VII. Strict Rules

1. **No business logic in pages.** If a page has an `if` that checks a business rule, move it to a service or domain function.
2. **No API calls in presentational components.** Presentational components receive data through props.
3. **No domain types with Leptos imports.** Domain types are framework-agnostic.
4. **No circular dependencies.** If module A imports module B, module B must never import module A.
5. **No god files.** No `utils.rs` with 50 unrelated functions. If a utility is shared, it gets its own focused module.
6. **No global mutable state outside of Leptos signals/context.** Use the reactive system.
7. **One component per file** (exception: very small related sub-components).
8. **Reuse UI components.** Before creating a new button/input/card, check `components/ui/`.
9. **CSS modules for component styles.** Global CSS only in `styles/`. Component-specific CSS is co-located.
10. **Server functions live in `api/`.** Not in components or pages.

---

## VIII. Naming Conventions

| Element | Pattern | Example |
|---|---|---|
| Domain type file | descriptive name | `domain/types.rs` |
| Error file | `errors` | `domain/errors.rs` |
| API module | `{area}` | `api/auth.rs` |
| UI component dir | component name | `components/ui/button/` |
| Feature component dir | feature name | `components/dashboard/` |
| Page dir | feature name | `pages/home/` |
| Page file | feature name | `pages/home/home.rs` |
| CSS module | `{component}.module.css` | `home.module.css` |
| State file | `{concern}` | `state/auth.rs` |

---

## IX. When to Bend the Rules

These rules optimize for **medium-to-large Leptos projects**. For very small projects (prototypes, single-page tools):

- You may flatten the structure (skip `domain/`, use fewer subfolders).
- You may skip the service layer when API calls are simple and direct.
- You may put small related components in the same file.

**But never skip the fundamental separation:** pages don't contain business logic, presentational components don't make API calls, domain types don't import Leptos. This rule holds regardless of project size.
