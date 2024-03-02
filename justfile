# just manual: https://github.com/casey/just/#readme

_default:
  @just --list

# Format source code
fmt:
  cargo fmt --all
  cd frontend && cargo fmt

fmt-check:
  cargo fmt --all -- --check
  cd frontend && cargo fmt -- --check

# Run the server in debug mode
run: frontend
  RUST_LOG=debug cargo run

# Build the frontend in debug mode
frontend: css
  cd frontend && trunk build

# Build the frontend in release mode
frontend-release: css-release
  cd frontend && trunk build --release

# Serve and watch frontend
frontend-watch:
  cd frontend && trunk serve

# Install NPM packages (required for tailwind)
frontend-install-npm-packages:
  cd frontend && npm install

# Build CSS file
css: frontend-install-npm-packages
  cd frontend && tailwindcss -i src/style.css -o target/style.css

# Build and minify CSS file
css-release: frontend-install-npm-packages
  cd frontend && tailwindcss -i src/style.css -o target/style.css --minify

# Build the server in debug mode
build: frontend
  cargo build

# Build the server in release mode (musl)
build-release: fmt-check test frontend-release
  cargo zigbuild --release --target x86_64-unknown-linux-musl

# Set up (and update) tooling
setup: frontend-install-npm-packages
  # Ignore rustup failures, because not everyone might use it
  rustup self update || true
  cargo install \
    wasm-pack \
    cargo-edit \
    trunk

# Upgrade (and update) dependencies and tools
upgrade:
  cargo upgrade --incompatible
  cargo update
  cd frontend && cargo upgrade --incompatible
  cd frontend && cargo update

# Run code checks
clippy:
  cargo clippy --workspace --locked --all-targets --all-features
  cd frontend && cargo clippy --locked --all-targets --all-features

# Fix lint warnings
fix:
  cargo fix --workspace --all-targets
  cargo clippy --workspace --all-targets --fix
  cd frontend && cargo fix --all-targets
  cd frontend && cargo clippy --all-targets --fix

# Run tests
test:
  cargo test --workspace
  cd frontend && cargo test
