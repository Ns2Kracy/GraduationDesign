_list:
    @just --list

alias f := fmt
alias l := lint
alias b := build
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

build:
    @echo "Building for production..."
    cargo build --release
    bun run build
    @echo "All built! ✨"

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

deploy:
    @echo "Start deploying to production..."
    
    @echo "Formatting..."
    just f

    @echo "Linting..."
    just l

    @echo "Building..."
    just b

    @echo "Deployed! ✨"