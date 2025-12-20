# WARP.md

This file provides guidance to WARP (warp.dev) when working with code in this repository.

## Repo layout (big picture)
- `backend/`: Rust (Actix-web) backend for a multi-app platform (auth + shared infra + multiple “apps”).
- `Pyckx-frontend/`: Next.js app (Bun-managed) that talks to the backend via `/api/*`.
- `backend/doc/`: Most of the authoritative architecture + feature-implementation docs.
- Root `README.md` is minimal; treat `backend/README.md`, `backend/doc/*`, and `Pyckx-frontend/doc/*` as the primary guides.

## Common commands

### Backend (Rust / Actix)
From repo root:
```bash
cd backend

# First-time env setup
cp .env.example .env
# Fill in required Supabase vars (SP_*) + SECURE_HTTP

# Build
cargo build

# Run dev server (uses IP/PORT from .env)
cargo run

# Run tests
cargo test

# Run a single test (filter by substring)
cargo test qcm_crud
cargo test auth::login

# See logs (tracing-subscriber reads RUST_LOG)
RUST_LOG=debug cargo run

# Formatting / linting (if toolchain components are installed)
cargo fmt
cargo clippy
```
Notes:
- Tests live under `backend/src/tests/` (a `#[cfg(test)] mod tests;` in `backend/src/main.rs`), and many are integration-style tests against Supabase.
- Backend static frontend serving is controlled by `SERVE_FRONTEND` + `STATIC_DIR` in `backend/.env`.

### Frontend (Next.js / Bun)
From repo root:
```bash
cd Pyckx-frontend

# Install deps
bun install

# Local env
cp .env.local.example .env.local

# Dev server
bun run dev

# Lint
bun run lint

# Production build (standard Next build)
bun run build
```

### Full-stack development mode (two terminals)
- Terminal 1:
```bash
cd backend
cargo run
```
- Terminal 2 (Next dev server proxies `/api/*` to backend; see `Pyckx-frontend/next.config.mjs`):
```bash
cd Pyckx-frontend
bun run dev
```

### Full-stack “single binary” production mode (serve frontend from Rust)
This repo supports exporting the frontend as static HTML and serving it from the Rust backend (see `backend/doc/deployment/frontend.md`).

From repo root:
```bash
# 1) Build a static export
cd Pyckx-frontend
BUILD_MODE=export SECURITY_MODE=production bun run build

# 2) Copy exported files into backend/static/
cd ..
rm -rf backend/static/*
cp -r Pyckx-frontend/out/* backend/static/

# 3) Enable frontend serving in backend/.env
#    SERVE_FRONTEND=true
#    STATIC_DIR=./static

# 4) Run backend in release mode
cd backend
cargo run --release
```

## Backend architecture (where to look)

### Layering and request flow
The backend follows a layered/clean architecture (documented in `backend/doc/architecture/overview.md`):
- `backend/src/api/`: HTTP layer.
  - `api/handlers/`: Actix handlers; keep these thin (parse/extract, call service, map to DTO).
  - `api/dto/`: request/response DTOs and conversions.
- `backend/src/services/`: business logic / orchestration.
- `backend/src/domain/`: pure domain types (no I/O).
- `backend/src/infrastructure/`: external integrations.
  - `infrastructure/repository/`: repository traits (abstractions).
  - `infrastructure/supabase/`: Supabase-backed implementations.
  - `infrastructure/json_storage/`: JSON-backed implementations (mostly tests/legacy).
- `backend/src/error/`: error types per domain + mapping to a shared HTTP error.

Typical flow:
Handler → Service → Repository implementation → (Supabase/other I/O)

### Dependency wiring (DI)
- `backend/src/app.rs` is the composition root: loads `Config`, constructs repositories, then constructs services. Keep state in `web::Data<App>` (internally `Arc`), so Actix worker cloning stays O(1).
- `backend/src/main.rs` configures Actix middleware (CORS, rate limiting, logging), registers routes via `api::init`, and (optionally) serves the static frontend.

### “Multi-app” platform structure
- App-level metadata/availability lives in `backend/src/apps/registry.rs` (`AVAILABLE_APPS`).
- Feature areas are grouped by app and domain in both handlers and services (e.g. `api/handlers/intello/*`, `services/intello/*`).

### Implementing new features / new Intello games
Start with these docs (they encode the project’s conventions):
- `backend/doc/architecture/adding-features.md`
- `backend/doc/architecture/end-to-end-feature.md` (backend + frontend workflow)

## Backend coding norms (idiomatic Rust + Actix-web)

### Handler shape (Actix extractors first)
Prefer Actix extractors (`web::Json`, `web::Path`, `web::Query`, `Multipart`, etc.) over manually parsing from `HttpRequest` where possible.
- Handler responsibilities: auth/session extraction, request validation, service call, DTO mapping.
- Avoid putting business rules, persistence details, or OpenRouter prompt-building in handlers.

A good signature usually looks like:
- `app: web::Data<App>` (shared state)
- `payload: web::Json<T>` / `web::Path<T>` / `web::Query<T>`
- `req: HttpRequest` only if you truly need request metadata (cookies, headers, peer addr)

### App state (`web::Data`) and cloning
Actix’s `web::Data<T>` is the idiomatic way to share state; it is `Arc`-backed. Construct heavy state once (outside `HttpServer::new`) and pass clones of `web::Data` into the factory closure.

### Error handling: typed, layered, and convertible to HTTP
Aim for “domain errors” and “HTTP errors” to be separate concepts:
- Domain/service/repository code returns typed errors (use enums; `thiserror` is already in use).
- HTTP layer converts errors into a consistent JSON error response.

In Actix-web, the idiomatic bridge is implementing `actix_web::ResponseError` for your top-level HTTP error type, providing `status_code()` and `error_response()`.

Practices that work well here:
- Use `thiserror` for `#[derive(Error)]` and `#[from]` conversions.
- Don’t return `String`/sentinel values for failures; carry context on error variants.
- Prefer `?` propagation + `map_err` at boundaries; keep `unwrap/expect` to tests only.

### Observability (tracing)
- Prefer `tracing` spans/events over `println!`.
- For non-trivial handlers and service methods, consider `#[tracing::instrument]` (often `skip(app, payload)` / `skip(self)`), and log identifiers as fields (e.g. `user_id = %user_id`) rather than building big strings.

### Service layer boundaries
Services should:
- validate inputs (including “authorization-like” checks that don’t require HTTP context),
- orchestrate repositories and external clients,
- return domain types (or DTO-friendly domain types),
- be testable with trait-based repositories (see `backend/src/infrastructure/repository/*`).

Repositories should:
- express intent in trait methods (e.g., `find_by_user`, `insert`, `delete`),
- hide Supabase/PostgREST HTTP details inside `infrastructure/supabase/*`.

### Config: parse early, type strongly
Prefer parsing env vars once in `Config::from_env()` into the types you actually want (e.g. `u16` for port, `bool` for flags) so parsing errors happen at startup rather than at request time.
- Backend env template: `backend/.env.example`.
- Config entry point: `backend/src/config/config.rs`.

### Optional evolution: “use cases” layer
If handlers begin to accumulate orchestration logic across multiple services, consider introducing an explicit `use_cases/` layer as a stable boundary:
`handler -> use_case -> service(s) -> repository`.
This is optional, but it can help keep handlers trivial while keeping services focused (especially for multi-step flows like AI generation + persistence).

## Frontend architecture (where to look)

- Next.js App Router pages live under `Pyckx-frontend/app/`.
- UI components live under `Pyckx-frontend/components/` (shadcn/ui primitives in `components/ui/`).
- API layer lives under `Pyckx-frontend/lib/api/`:
  - Endpoint configuration is driven by env vars (see `.env.local.example` and `Pyckx-frontend/doc/environment-variables.md`).
- Development proxying:
  - `Pyckx-frontend/next.config.mjs` rewrites `/api/:path*` to `BACKEND_URL` during dev.
  - For static export mode, `BUILD_MODE=export` enables `output: "export"` and `trailingSlash`.

Relevant frontend docs:
- `Pyckx-frontend/doc/architecture.md`
- `Pyckx-frontend/doc/security-configuration.md`
- `Pyckx-frontend/doc/environment-variables.md`
