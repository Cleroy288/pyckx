# Frontend Deployment Guide

This document explains how to build the frontend and serve it with the Rust backend.

## Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                    Production Mode                          │
│  ┌─────────────────────────────────────────────────────┐   │
│  │              Rust Backend (Actix-web)               │   │
│  │  ┌─────────────┐  ┌─────────────┐  ┌────────────┐  │   │
│  │  │  API Routes │  │Static Files │  │SPA Fallback│  │   │
│  │  │ /api/*      │  │ /_next/*    │  │ index.html │  │   │
│  │  └─────────────┘  └─────────────┘  └────────────┘  │   │
│  └─────────────────────────────────────────────────────┘   │
└─────────────────────────────────────────────────────────────┘
```

## Quick Start

### Development Mode (Standard)

Run frontend and backend separately:

```bash
# Terminal 1: Backend
cd backend && cargo run

# Terminal 2: Frontend (Next.js dev server with proxy)
cd Pyckx-frontend && npm run dev
```

### Development Mode (Production Parity) ⭐ Recommended

Single command that mirrors production - **backend serves frontend**:

```bash
cd Pyckx-frontend
npm run dev:prod
```

This runs:
- 🟡 **RUST**: `cargo watch` - auto-recompiles backend on changes
- 🔵 **NEXT**: Bun file watcher - rebuilds frontend to `backend/static/`

Access at `http://localhost:8080` (same as production).

### Production Deploy (Fly.io)

```bash
# 1. Build frontend
cd Pyckx-frontend && npm run build:static

# 2. Deploy backend (includes frontend in static/)
cd ../backend && fly deploy
```

## Detailed Instructions

### 1. Build Frontend

From the `Pyckx-frontend` directory:

```bash
# Set environment variables for production build
export BUILD_MODE=export
export SECURITY_MODE=production

# Build
bun run build
```

This creates an `out/` directory with static HTML files.

### 2. Copy to Backend

Copy the entire `out/` folder contents to `backend/static/`:

```bash
# From project root
rm -rf backend/static/*
cp -r Pyckx-frontend/out/* backend/static/
```

Expected structure:
```
backend/static/
├── index.html
├── login.html
├── register.html
├── collection.html
├── _next/
│   └── static/
│       ├── chunks/
│       ├── css/
│       └── media/
└── ... (other static assets)
```

### 3. Configure Backend

Update `backend/.env`:

```bash
# Enable frontend serving
SERVE_FRONTEND=true
STATIC_DIR=./static
```

### 4. Run Production Server

```bash
cd backend
cargo run --release
```

The server will:
- Serve API routes at `/auth/*`, `/user/*`, `/api/*`, `/app/*`
- Serve static assets from `/_next/*`
- Fallback to `index.html` for SPA client-side routing (also serves other static files like images)

## Environment Variables

### Backend (.env)

| Variable | Development | Production | Description |
|----------|-------------|------------|-------------|
| `SERVE_FRONTEND` | `false` | `true` | Enable static file serving |
| `STATIC_DIR` | `./static` | `./static` | Path to frontend build |

### Frontend (.env)

| Variable | Development | Production | Description |
|----------|-------------|------------|-------------|
| `BUILD_MODE` | `development` | `export` | Build output type |
| `SECURITY_MODE` | `development` | `production` | Security headers |
| `BACKEND_URL` | `http://localhost:8080` | N/A | Proxy target (dev only) |

## Troubleshooting

### 404 on page refresh

Ensure `SERVE_FRONTEND=true` is set. The SPA fallback serves `index.html` for all unmatched routes.

### Static assets not loading

Check that `_next/` folder exists in `backend/static/` and contains the built assets.

### API calls failing

Verify API routes are configured before static file serving in `main.rs`. API routes have higher priority.

### CORS errors in development

The frontend proxy in `next.config.mjs` handles CORS during development. In production, same-origin requests don't need CORS.

## Build Script (Optional)

Create `scripts/build-prod.sh` in project root:

```bash
#!/bin/bash
set -e

echo "🔨 Building frontend..."
cd Pyckx-frontend
BUILD_MODE=export SECURITY_MODE=production bun run build

echo "📦 Copying to backend..."
cd ..
rm -rf backend/static
mkdir -p backend/static
cp -r Pyckx-frontend/out/* backend/static/

echo "✅ Done! Run: cd backend && cargo run --release"
```
