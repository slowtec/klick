# just manual: https://github.com/casey/just/#readme

_default:
  @just --list

# Format source code
fmt:
  cargo fmt --all
  cd frontend && cargo fmt

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

# Build CSS file
css:
  cd frontend && tailwind -i src/style.css -o target/style.css

# Build and minify CSS file
css-release:
  cd frontend && tailwind -i src/style.css -o target/style.css --minify

# Build the server in debug mode
build: frontend
  cargo build

# Build the server in release mode (musl)
build-release: frontend-release
  cargo build --release --target x86_64-unknown-linux-musl
