_list:
    @just --list

fmt:
    cargo fmt --all
    bun run format

lint:
    cargo clippy --all-targets --all-features -- -D warnings
    bun run lint

build:
    cargo build --release
    bun run build

dev:
    cargo run
    bun run dev

start:
    cargo run --release

deploy: