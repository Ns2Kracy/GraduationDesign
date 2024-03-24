_list:
    @just --list

# Set shell for non-Windows OSs:
set shell := ["sh", "-c"]

# Set shell for Windows OSs:
set windows-shell := ["powershell.exe", "-NoLogo", "-Command"]

# Aliases
alias u := update
alias f := format
alias l := lint
alias b := build
alias ps := panda-studio
alias s := server
alias w := web

# Update dependencies
update:
    echo "Updating dependencies..."
    cargo update
    cd apps/web ; bun update
    echo "All up to date! ✨"

# Format the codebase
format:
    echo "Starting formatters..."
    cargo fmt --all
    cd apps/web ; bun run format
    echo "All formatted! ✨"

# Lint commands
lint:
    echo "Starting linters..."
    cargo clippy --all-targets --all-features -- -D warnings
    cd apps/web ; bun run lint
    echo "All linted! ✨"

# Panda Studio
panda-studio:
    echo "Starting Panda Studio..."
    cd apps/web ; bun run studio

# Start server
_dev-server:
    echo "Starting development server..."
    cargo run

_prod-server:
    echo "Starting production server..."
    cargo run --release

server target:
    echo "Starting server for {{target}}..."
    just _{{target}}-server
    echo "{{target}} server started! ✨"

# Start web
_dev-web:
    echo "Starting web..."
    cd apps/web ; bun run dev

_prod-web:
    echo "Starting web..."
    cd apps/web ; bun run preview

web target:
    echo "Starting web for {{target}}..."
    just _{{target}}-web
    echo "{{target}} web started! ✨"

# Build commands
_build-dev-server:
    echo "Building development server..."
    cargo build
    echo "Development server built! ✨"

_build-release-server:
    echo "Building production server..."
    cargo build --release
    echo "Production server built! ✨"

build-web:
    echo "Building web..."
    cd apps/web ; bun run build
    echo "Web built! ✨"

build target:
    echo "Building for {{target}}..."
    just _build-{{target}}-server
    just build-web
    echo "{{target}} built! ✨"