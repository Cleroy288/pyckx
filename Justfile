# Lapp-Pyckx Development Commands
# Run with: just <recipe>

# Default: run both frontend and backend (use two terminals or &)
dev:
    @echo "Starting hot reload dev servers..."
    @echo "Run these in separate terminals:"
    @echo "  just watch-front"
    @echo "  just watch-back"
    @echo ""
    @echo "Or run both with:"
    @echo "  just dev-all"

# Run both in background (simpler approach)
dev-all:
    #!/usr/bin/env bash
    trap 'kill 0' EXIT
    (cd leptos-app && trunk watch) &
    (cd backend && cargo watch -x run) &
    wait

# Frontend: Trunk watches and builds to backend/static
watch-front:
    cd leptos-app && trunk watch

# Backend: cargo-watch restarts on changes
watch-back:
    cd backend && cargo watch -x run

# Production build
build:
    cd leptos-app && trunk build --release
    cd backend && cargo build --release

# Clean all build artifacts
clean:
    cargo clean
    rm -rf backend/static

# Install development tools
install-tools:
    cargo install just cargo-watch trunk
