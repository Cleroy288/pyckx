# LAPP - Leptos + Actix Full-Stack App

## Development Setup

### Prerequisites

```bash
# Install development tools
cargo install just            # Command runner
cargo install cargo-watch     # Backend hot reload
cargo install trunk           # Frontend builder

# Add WASM target
rustup target add wasm32-unknown-unknown
```

### Quick Start

```bash
# Install tools (one-time)
just install-tools

# Start development with hot reload
just dev-all
```

This runs both:
- **Frontend** (Trunk): Watches `leptos-app/`, builds to `backend/static/`
- **Backend** (cargo-watch): Restarts on Rust file changes

Open http://localhost:8080 in your browser.

Press `Ctrl+C` to stop both servers.

---

## Project Structure

```
lapp-pyckx/
├── Cargo.toml          # Workspace root
├── .cargo/config.toml  # Build config
├── Justfile            # Development commands
├── backend/            # Actix-web server
│   ├── static/         # Leptos dist (auto-generated)
│   └── src/
└── leptos-app/         # Leptos frontend
    ├── Trunk.toml      # Build config
    └── src/
```

---

## Available Commands

| Command | Description |
|---------|-------------|
| `just dev-all` | Start both frontend + backend with hot reload |
| `just watch-front` | Frontend only (Trunk) |
| `just watch-back` | Backend only (cargo-watch) |
| `just build` | Production build |
| `just clean` | Clean all artifacts |

---

## Environment Variables

For development, create `.env` in `backend/`:

```bash
SERVE_FRONTEND=true   # Enable static file serving
STATIC_DIR=./static   # Path to Leptos dist
```

---

## Architecture

```
┌─────────────────────────────────────────────────┐
│                    Browser                       │
│                 localhost:8080                   │
└─────────────────────┬───────────────────────────┘
                      │
┌─────────────────────▼───────────────────────────┐
│              Actix-web Backend                   │
│  • API routes: /api/*                           │
│  • Static files: /* → backend/static/           │
│  • SPA fallback: index.html                     │
└─────────────────────┬───────────────────────────┘
                      │
         ┌────────────┴────────────┐
         │                         │
    /api/* routes          Static assets
    (Rust handlers)        (Leptos WASM)
```

---

## Hot Reload Flow

1. **Edit frontend** (`leptos-app/src/*.rs`)
   - Trunk detects change
   - Rebuilds WASM (~1-2s)
   - Outputs to `backend/static/`
   - Refresh browser to see changes

2. **Edit backend** (`backend/src/*.rs`)
   - cargo-watch detects change
   - Recompiles backend (~3-5s)
   - Restarts server automatically

---

## Troubleshooting

### Slow compilation
Enable incremental builds (already configured in `.cargo/config.toml`).

### Frontend not loading
1. Check `SERVE_FRONTEND=true` in `.env`
2. Verify `backend/static/` contains `index.html`
3. Run `just watch-front` to rebuild

### just command not found
Make sure `~/.cargo/bin` is in your PATH:
```bash
export PATH="$HOME/.cargo/bin:$PATH"
```
