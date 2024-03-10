_list:
    @just --list

alias f := fmt
alias l := lint
alias bd := build-dev
alias br := build-release
alias s := start
alias u := update

fmt:
    @echo "Starting formatters..."
    cargo fmt --all
    bun run format
    @echo "All formatted! ✨"

lint:
    @echo "Starting linters..."
    cargo clippy --all-targets --all-features -- -D warnings
    bun run lint
    @echo "All linted! ✨"

dev:
    @echo "Starting development server..."
    cargo run

start:
    @echo "Starting production server..."
    cargo run --release

update:
    @echo "Updating dependencies..."
    cargo update
    bun update
    @echo "All up to date! ✨"

build-server-dev:
    @echo "Building for production..."
    cargo build
    @echo "Done!"

build-server-release:
    @echo "Building production server..."
    cargo build --release
    @echo "Done!"

build-web:
    @echo "Building web..."
    bun run build
    @echo "Done!"

build-dev:
    @echo "Building development server..."
    just build-server-dev
    just build-web
    @echo "All built! ✨"

build-release:
    @echo "Building for production..."
    just build-server-release
    just build-web
    @echo "All built! ✨"

deploy:
    @echo "Start deploying to production..."
    
    @echo "Formatting..."
    just f

    @echo "Linting..."
    just l

    @echo "Building..."
    just b

    @echo "Deployed! ✨"