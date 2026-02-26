# Lapp-Pyckx Development Commands

# 1. Build CSS + WASM  2. Delete old  3. Replace  4. Serve
build-front:
    #!/usr/bin/env bash
    set -e
    cd dioxus-app
    npx @tailwindcss/cli -i input.css -o assets/tailwind.css
    rm -rf ../target/dx/dioxus-app/release
    dx build --release
    DX="../target/dx/dioxus-app/release/web/public"
    cp -r assets/* "$DX/assets/"
    rm -rf ../backend/static
    cp -r "$DX" ../backend/static
    echo "✓ frontend ready in backend/static"

# Run both: frontend watch + backend watch
dev-all:
    #!/usr/bin/env bash
    trap 'kill 0' EXIT
    (cd dioxus-app && cargo watch -w src -w assets -w input.css -s "\
        npx @tailwindcss/cli -i input.css -o assets/tailwind.css \
        && rm -rf ../target/dx/dioxus-app/release \
        && dx build --release \
        && DX=../target/dx/dioxus-app/release/web/public \
        && cp -r assets/* \$DX/assets/ \
        && rm -rf ../backend/static \
        && cp -r \$DX ../backend/static \
        && echo '✓ frontend ready'" 2>&1 | sed 's/^/[front] /') &
    (cd backend && cargo watch -x run 2>&1 | sed 's/^/[back]  /') &
    wait

# Backend: auto-restart on changes
watch-back:
    cd backend && cargo watch -x run

# Production build (minified CSS + backend binary)
build:
    #!/usr/bin/env bash
    set -e
    cd dioxus-app
    npx @tailwindcss/cli -i input.css -o assets/tailwind.css --minify
    rm -rf ../target/dx/dioxus-app/release
    dx build --release
    DX="../target/dx/dioxus-app/release/web/public"
    cp -r assets/* "$DX/assets/"
    rm -rf ../backend/static
    cp -r "$DX" ../backend/static
    cd ../backend && cargo build --release

# Clean everything
clean:
    cargo clean
    rm -rf backend/static
