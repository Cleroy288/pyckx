# Pyckx Backend

A Rust/Actix-web backend with Clean Architecture and Supabase.

## Quick Start

```bash
cd backend
cargo run          # Dev server at localhost:8080
cargo test         # Run tests
```

## Development Modes

| Mode | Command | Description |
|------|---------|-------------|
| **API Only** | `cargo run` | Backend at :8080, frontend dev server separate |
| **Production Parity** | `npm run dev:prod` (from frontend) | Backend serves frontend with hot reload |
| **Production** | `cargo run --release` | Release build with static frontend |

## Project Structure

```
src/
├── api/            # Handlers & DTOs
├── use_cases/      # Application orchestration (Handler → UseCase → Service)
├── services/       # Business logic
├── domain/         # Pure entities
├── infrastructure/ # Repository traits + Supabase/JSON implementations
├── error/          # Domain error types
├── shared/         # Utilities (document extraction, prompts)
└── config/         # Environment config
```

## Documentation

| Doc | Description |
|-----|-------------|
| [Architecture Overview](./architecture/overview.md) | Layers, request flow, folder structure |
| [API Documentation (Generated)](./rustdoc/LAPP/index.html) | Requires local server or opening raw file |
| [Use Cases](architecture/use-cases.md) | Use Cases layer pattern |
| [Adding Features](architecture/adding-features.md) | Step-by-step templates |
| [End-to-End Feature](architecture/end-to-end-feature.md) | Complete backend + frontend implementation guide |
| [Intello App](apps/intello/README.md) | AI-powered games (QCM, flashcards, etc.) |
| [Collection App](apps/collection/README.md) | DVD collection management |
| [Database Schema](database/schema.md) | Supabase tables reference |
| [Frontend Deployment](deployment/frontend.md) | Build & serve with backend |

## API Routes

All routes follow pattern: `/api/{service}/{feature}/{id}`

| Prefix | Description |
|--------|-------------|
| `/api/auth/*` | Login, logout |
| `/api/user/*` | User profile |
| `/api/apps/*` | App management |
| `/api/collection/dvds` | DVD CRUD |
| `/api/intello/*` | Games & AI generation |

## Environment

```bash
# Required - Supabase
SP_URL=https://xxx.supabase.co
SP_ANON=xxx
SP_SERVICE_ROLE=xxx
SP_ID=xxx

# Required - Server
IP=127.0.0.1
PORT=8080
SECURE_HTTP=true

# Frontend Serving (production)
SERVE_FRONTEND=true
STATIC_DIR=./static

# AI (for Intello app)
OPENROUTER_API_KEY=xxx
GOOGLE_AI_KEY=xxx  # Optional, for higher rate limits
```
