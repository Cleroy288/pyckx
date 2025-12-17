# Pyckx Backend

A Rust/Actix-web backend with Clean Architecture and Supabase.

## Quick Start

```bash
cd backend
cargo run          # Dev server at localhost:8080
cargo test         # Run tests
```

## Project Structure

```
src/
├── api/            # Handlers & DTOs
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
| [Architecture](architecture/overview.md) | Layers, request flow, folder structure |
| [Adding Features](architecture/adding-features.md) | Step-by-step templates |
| [End-to-End Feature](architecture/end-to-end-feature.md) | Complete backend + frontend implementation guide |
| [Intello App](apps/intello/README.md) | AI-powered games (QCM, flashcards, etc.) |
| [Collection App](apps/collection/README.md) | DVD collection management |
| [Database Schema](database/schema.md) | Supabase tables reference |
| [Frontend Deployment](deployment/frontend.md) | Build & serve with backend |

## API Routes

| Prefix | Description |
|--------|-------------|
| `/auth/*` | Login, register, logout |
| `/api/apps/*` | App management |
| `/app/collection/*` | DVD CRUD |
| `/app/intello/*` | Games & AI generation |

## Environment

```bash
SUPABASE_URL=https://xxx.supabase.co
SUPABASE_ANON_KEY=xxx
SUPABASE_SERVICE_KEY=xxx
OPENROUTER_API_KEY=xxx
HOST=127.0.0.1
PORT=8080
```
