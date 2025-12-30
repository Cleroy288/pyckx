# Pyckx Frontend Architecture

## Overview

Pyckx is a Next.js 14+ application using the App Router, React 18, TypeScript, and Tailwind CSS with shadcn/ui components.

## Tech Stack

- **Framework**: Next.js 14+ (App Router)
- **Language**: TypeScript
- **Styling**: Tailwind CSS + CSS Variables
- **UI Components**: shadcn/ui (Radix primitives)
- **Package Manager**: Bun
- **State Management**: React Context + useState/useRef
- **Authentication**: HTTP-only session cookies

## Directory Structure

```
Pyckx-frontend/
├── app/                    # Next.js App Router pages
│   ├── layout.tsx          # Root layout with providers
│   ├── page.tsx            # Dashboard (main page)
│   ├── login/page.tsx      # Login page
│   ├── register/page.tsx   # Registration page
│   └── collection/page.tsx # DVD Collection app
├── components/             # React components
│   ├── ui/                 # shadcn/ui primitives (57 generic components)
│   ├── intello/            # Intello app components
│   │   ├── qcm-player.tsx
│   │   ├── flashcard-player.tsx
│   │   ├── fill-blank-player.tsx
│   │   ├── keywords-player.tsx
│   │   ├── open-question-player.tsx
│   │   ├── order-phrase-player.tsx
│   │   ├── true-false-player.tsx
│   │   ├── custom-question-form.tsx
│   │   ├── manual-qcm-form.tsx
│   │   └── mermaid-diagram.tsx
│   ├── dashboard/          # Dashboard components
│   │   ├── app-card.tsx
│   │   ├── app-details.tsx
│   │   └── app-grid.tsx
│   ├── auth/               # Auth components
│   │   └── auth-guard.tsx
│   ├── layout/             # Layout components
│   │   ├── top-navigation.tsx
│   │   ├── grid-background.tsx
│   │   ├── theme-provider.tsx
│   │   └── theme-toggle.tsx
│   └── settings/           # Settings components
│       └── palette-selector.tsx
├── lib/                    # Utilities and logic
│   ├── api/                # API layer
│   │   ├── config.ts       # API endpoints
│   │   ├── auth.ts         # Auth API functions
│   │   └── types.ts        # Shared API types
│   ├── classes/            # Class-based modules
│   │   └── collection/     # Collection app classes
│   ├── theme-context.tsx   # Light/dark theme
│   ├── palette-context.tsx # Color palette system
│   ├── color-palettes.ts   # Palette definitions
│   ├── auth-context.tsx    # Auth state
│   ├── user-apps-context.tsx # User apps state
│   └── apps-data.ts        # Available apps config
└── doc/                    # Documentation
```

## Class-Based Architecture

App-specific logic uses TypeScript classes for encapsulation:

```
lib/classes/collection/
├── index.ts                # Main exports
├── CollectionApp.ts        # App orchestrator
├── items/
│   ├── Dvd.ts              # DVD class + types
│   └── index.ts
└── lists/
    ├── DvdList.ts          # DVD list with filter/sort
    └── index.ts
```

### Benefits
- Types co-located with classes (no global types bloat)
- API calls encapsulated as class methods
- Immutable list operations (filter/sort return new instances)
- Easy to extend (add Book, Game, etc.)

### Usage
```typescript
import { CollectionApp, DvdList, Dvd } from "@/lib/classes/collection"

const collection = new CollectionApp()
await collection.loadDvds()
const filtered = collection.dvds.filter("matrix", "name").sort("year", "desc")
```

## Theming System

### Theme (Light/Dark)
- Stored in `localStorage` key: `pyckx-theme`
- Managed by `ThemeProvider` in `lib/theme-context.tsx`
- Applies `dark` class to `<html>` element

### Color Palettes
- 5 pre-configured palettes (Teal Tech, Purple Dream, etc.)
- Stored in `localStorage` key: `pyckx-palette`
- Managed by `PaletteProvider` in `lib/palette-context.tsx`

### Custom Colors
- Per-mode customization (light/dark separate)
- Stored in `localStorage` key: `pyckx-custom-colors`
- 8 customizable colors: primary, secondary, accent, muted, background, success, warning, destructive
- Import feature: copy colors from other mode (excluding background)

### CSS Variables
All colors use CSS variables that update dynamically:
```css
--primary, --secondary, --accent, --muted, --background
--success, --warning, --destructive
--foreground, --card, --border, --input, --ring
```

## Authentication

- Session-based auth with HTTP-only cookies
- Cookie name: `session_id`
- Backend: Rust/Actix-Web at `/api/auth/*` endpoints
- `AuthGuard` component protects routes
- `AuthProvider` manages auth state globally

## API Layer

### Endpoints (lib/api/config.ts)

All endpoints use the `/api/{service}/{feature}` pattern:

```typescript
endpoints.auth.login()       // POST /api/auth/login
endpoints.auth.logout()      // POST /api/auth/logout
endpoints.user.me()          // GET  /api/user/me
endpoints.collection.dvds()  // GET/POST /api/collection/dvds
endpoints.intello.*          // /api/intello/*
```

### Error Handling
Backend returns: `{ code: string, message: string, field?: string }`
Frontend extracts and displays user-friendly messages.

## Commands

```bash
bun install              # Install dependencies
npm run dev              # Standard dev server (Next.js at :3000, backend at :8080)
npm run dev:prod         # Production-parity dev (backend serves frontend at :8080)
npm run build:static     # Build for production (exports to backend/static/)
bun run lint             # Run ESLint
```

