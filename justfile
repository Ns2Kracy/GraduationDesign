_list:
    @just --list

# Cross platform shebang:
shebang := if os() == 'windows' {
  'powershell.exe'
} else {
  '/usr/bin/env pwsh'
}

# Set shell for non-Windows OSs:
set shell := ["sh", "-c"]

# Set shell for Windows OSs:
set windows-shell := ["powershell.exe", "-NoLogo", "-Command"]

# If you have PowerShell Core installed and want to use it,
# use `pwsh.exe` instead of `powershell.exe`

shebang:
	#!{{shebang}}
	$PSV = $PSVersionTable.PSVersion | % {"$_" -split "\." }
	$psver = $PSV[0] + "." + $PSV[1]
	if ($PSV[2].Length -lt 4) {
		$psver += "." + $PSV[2] + " Core"
	} else {
		$psver += " Desktop"
	}
	echo "PowerShell $psver"

alias f := fmt
alias l := lint
alias ps := panda-studio
alias bw := build-web
alias bds := build-dev-server
alias bd := build-dev
alias brs := build-release-server
alias br := build-release
alias u := update
alias dw := dev-web
alias ds := dev-server
alias sw := start-web
alias ss := start-server

fmt:
    @echo "Starting formatters..."
    cargo fmt --all
    cd apps/web ; bun run format
    @echo "All formatted! ✨"

lint:
    @echo "Starting linters..."
    cargo clippy --all-targets --all-features -- -D warnings
    cd apps/web ; bun run lint
    @echo "All linted! ✨"

panda-studio:
    @echo "Starting Panda Studio..."
    cd apps/web ; bun run studio

dev-server:
    @echo "Starting development server..."
    cargo run

start-server:
    @echo "Starting production server..."
    cargo run --release

dev-web:
    @echo "Starting web..."
    cd apps/web ; bun run dev

start-web:
    @echo "Starting web..."
    cd apps/web ; bun run preview

update:
    @echo "Updating dependencies..."
    cargo update
    cd apps/web ; bun update
    just bds ; just bw
    @echo "All up to date! ✨"

build-dev-server:
    @echo "Building for production..."
    cargo build
    @echo "Done!"

build-release-server:
    @echo "Building production server..."
    cargo build --release
    @echo "Done!"

build-web:
    @echo "Building web..."
    cd apps/web ; bun run build
    @echo "Done!"

build-dev:
    @echo "Building development server..."
    just build-dev-server
    just build-web
    @echo "All built! ✨"

build-release:
    @echo "Building for production..."
    just build-release-server
    just build-web
    @echo "All built! ✨"