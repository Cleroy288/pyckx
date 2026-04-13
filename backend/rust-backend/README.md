# LAPP Backend

A Rust backend for a multi-app platform — a single server hosting multiple personal projects, all rewritten in Rust.

## Overview

LAPP (Login Application Platform) is a modular backend that serves as the foundation for multiple apps under one roof. Instead of maintaining separate backends for each project, this platform provides shared infrastructure (authentication, sessions, error handling) while allowing each app to live in its own isolated module.

## Architecture

```
src/
├── api/              # HTTP layer (handlers, DTOs)
├── apps/             # Feature apps (each app is self-contained)
│   └── collection/   # Example: Collection app
├── domain/           # Core business entities
├── services/         # Business logic
├── infrastructure/   # External integrations (Supabase, etc.)
├── error/            # Centralized error handling
├── config/           # Environment configuration
└── shared/           # Cross-cutting utilities
```

## Apps

Each app implements the `AppModule` trait and lives in `src/apps/`:

| App        | Description                               | Status         |
| ---------- | ----------------------------------------- | -------------- |
| Collection | Manage collections of items and resources | 🚧 In Progress |

## Tech Stack

- **Rust** + **Actix-web** — Fast, type-safe web framework
- **Supabase** — Authentication provider
- **Tracing** — Structured logging

## Getting Started

```bash
# Clone and navigate
cd backend

# Set up environment
cp .env.example .env
# Edit .env with your Supabase credentials

# Run
cargo run
```

## Environment Variables

```env
IP=127.0.0.1
PORT=8080
SP_ID=your-supabase-project-id
SP_URL=https://your-project.supabase.co
SP_ANON=your-anon-key
SP_SERVICE_ROLE=your-service-role-key
SECURE_HTTP=true or false
```

## API Endpoints

### Auth

- `POST /auth/login` — Login with email/password
- `POST /auth/register` — Register new user
- `POST /auth/logout` — Logout current session

### User

- `GET /user/me` — Get current user info

## Adding a New App

1. Create module in `src/apps/your_app/`
2. Implement `AppModule` trait
3. Add to `App` struct in `src/app.rs`
4. Register routes in `src/api/handlers/`

## License

MIT
