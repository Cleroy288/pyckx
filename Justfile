# Lapp-Pyckx Development Commands

# Build CodeMirror editor bundle
build-editor:
    cd dioxus-app/editor && npm run build

# Watch CodeMirror editor for changes
watch-editor:
    cd dioxus-app/editor && npm run watch

# Watch Tailwind CSS (rebuild on class changes)
watch-css:
    cd dioxus-app && npx @tailwindcss/cli -i input.css -o assets/tailwind.css --watch

# Fast frontend dev: dx serve + Tailwind watcher
# Hot reload RSX < 1s, full recompile ~3-5s (debug)
dev: _kill-ports
    #!/usr/bin/env bash
    trap 'kill 0' EXIT
    (cd dioxus-app && npx @tailwindcss/cli -i input.css -o assets/tailwind.css --watch 2>&1 | sed 's/^/[css]   /') &
    (cd dioxus-app && dx serve --port 8090 2>&1 | sed 's/^/[dx]    /') &
    wait

# 1. Build editor  2. Build CSS + WASM  3. Replace  4. Serve
build-front:
    #!/usr/bin/env bash
    set -e
    cd dioxus-app/editor && npm run build && cd ../..
    cd dioxus-app
    npx @tailwindcss/cli -i input.css -o assets/tailwind.css
    rm -rf ../target/dx/dioxus-app/release
    dx build --release
    DX="../target/dx/dioxus-app/release/web/public"
    cp -r assets/* "$DX/assets/"
    rm -rf ../backend/rust-backend/static
    cp -r "$DX" ../backend/rust-backend/static
    echo "✓ frontend ready in backend/rust-backend/static"

# Kill stale processes on dev ports
_kill-ports:
    #!/usr/bin/env bash
    for p in 8083 8090 8001 3100; do
        lsof -ti :$p 2>/dev/null | xargs kill -9 2>/dev/null || true
    done
    echo "✓ ports 8080 8083 8001 3100 freed"

# Run all services (dx serve + backends)
dev-all: _kill-ports
    #!/usr/bin/env bash
    trap 'kill 0' EXIT
    redis-server --daemonize yes 2>/dev/null || true
    (cd auth-service && bun run --watch src/index.ts 2>&1 | sed 's/^/[auth]  /') &
    (cd dioxus-app/editor && npm run watch 2>&1 | sed 's/^/[edit]  /') &
    (cd dioxus-app && npx @tailwindcss/cli -i input.css -o assets/tailwind.css --watch 2>&1 | sed 's/^/[css]   /') &
    (cd dioxus-app && dx serve --port 8090 2>&1 | sed 's/^/[dx]    /') &
    (cd backend/rust-backend && cargo watch -x run 2>&1 | sed 's/^/[back]  /') &
    (cd backend/python-backend && python -m uvicorn app.main:app --reload --host 127.0.0.1 --port 8001 2>&1 | sed 's/^/[py]    /') &
    wait

# Rust backend: auto-restart on changes
watch-back:
    cd backend/rust-backend && cargo watch -x run

# Python backend: auto-restart on changes
watch-py:
    cd backend/python-backend && python -m uvicorn app.main:app --reload --host 127.0.0.1 --port 8001

# Auth service: auto-restart on changes
watch-auth:
    cd auth-service && bun run --watch src/index.ts

# Production build (minified CSS + backend binary)
build:
    #!/usr/bin/env bash
    set -e
    cd dioxus-app/editor && npm run build && cd ../..
    cd dioxus-app
    npx @tailwindcss/cli -i input.css -o assets/tailwind.css --minify
    rm -rf ../target/dx/dioxus-app/release
    dx build --release
    DX="../target/dx/dioxus-app/release/web/public"
    cp -r assets/* "$DX/assets/"
    rm -rf ../backend/rust-backend/static
    cp -r "$DX" ../backend/rust-backend/static
    cd ../backend/rust-backend && cargo build --release

# Clean everything
clean:
    cargo clean
    rm -rf backend/rust-backend/static dioxus-app/assets/js/codemirror.bundle.js
