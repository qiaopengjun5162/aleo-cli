# aleo-cli — CLI tool for Aleo blockchain
# see https://github.com/casey/just

set positional-arguments := true

# ── Build ────────────────────────────────────────────

# Build the project
build:
    cargo build

# Release build
build-release:
    cargo build --release

# Clean build artifacts
clean:
    cargo clean

# ── Test ────────────────────────────────────────────

# Run all tests (nextest preferred, fallback to cargo test)
test:
    cargo nextest run || cargo test

# ── Lint / Check ────────────────────────────────────

# Quick code check
check:
    cargo check

# Lint with Clippy
clippy:
    cargo clippy -- -D warnings

# Format code check (CI)
format:
    cargo fmt --all -- --check

# Fix code formatting
format-fix:
    cargo fmt --all

# Spell check with typos
typos:
    typos

# ── Docs ────────────────────────────────────────────

# Generate documentation
docs:
    cargo doc --no-deps

# Open docs in browser
docs-open:
    cargo doc --no-deps --open

# ── Quality ─────────────────────────────────────────

# Security audit (requires cargo-deny)
audit:
    cargo deny check

# Update dependencies
update:
    cargo update

# ── Run all ─────────────────────────────────────────

# Run all checks: format + check + clippy + test
all: format check clippy test

# Full CI suite: format + check + clippy + test + audit + docs
ci-full: format check clippy test audit docs

# ── Publish ─────────────────────────────────────────

# Publish to crates.io (dry-run first)
publish-dry-run:
    cargo publish --dry-run

# Publish to crates.io
publish:
    cargo publish
