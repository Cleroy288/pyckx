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
| [Architecture](architecture/overview.md) | Layers, request flow, folder structure |
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
| `/api/auth/*` | Login, register, logout |
| `/api/user/*` | User profile |
| `/api/apps/*` | App management |
| `/api/collection/dvds` | DVD CRUD |
| `/api/intello/*` | Games & AI generation |

## Environment

```bash
# Required
SUPABASE_URL=https://xxx.supabase.co
SUPABASE_ANON_KEY=xxx
SUPABASE_SERVICE_KEY=xxx
OPENROUTER_API_KEY=xxx
HOST=127.0.0.1
PORT=8080

# Optional (for higher Gemini rate limits)
GOOGLE_AI_KEY=xxx
```

