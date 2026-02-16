# Default recipe to display help
default:
  @just --list

# Format all code
format:
  rumdl fmt .
  taplo fmt
  cargo +nightly fmt --all

# Auto-fix linting issues
fix:
  rumdl check --fix .

# Run all lints
lint:
  rumdl check .
  taplo fmt --check
  cargo +nightly fmt --all -- --check
  cargo +nightly clippy --all -- -D warnings
  cargo machete

# Run tests
test:
  cargo test --all-features

# Run tests with coverage
test-coverage:
  cargo tarpaulin --all-features --workspace --timeout 300

# Build entire workspace
build:
  cargo build --workspace

# Check all targets compile
check:
  cargo check --all-targets --all-features

# Check for Chinese characters
check-cn:
  rg --line-number --column "\p{Han}"

# Full CI check
ci: lint test build

# ============================================================
# Maintenance & Tools
# ============================================================

# Clean build artifacts
clean:
  cargo clean

# Install all required development tools
setup:
  cargo install cargo-machete
  cargo install taplo-cli
  cargo install typos-cli

# Generate documentation for the workspace
docs:
  cargo doc --no-deps --open

# ============================================================
# SDK Commands
# ============================================================

# Run SDK unit tests
sdk-test:
  cargo test -p elevenlabs-sdk

# Run SDK clippy lints
sdk-lint:
  cargo clippy -p elevenlabs-sdk -- -D warnings

# Generate SDK documentation
sdk-doc:
  cargo doc -p elevenlabs-sdk --no-deps

# Build all SDK examples
sdk-build-examples:
  cargo build -p elevenlabs-sdk --examples

# ============================================================
# SDK Integration Testing (Mock Server)
# ============================================================

# Start mock server on port 4010 (background)
sdk-mock-start:
  @echo "Starting mock server on http://127.0.0.1:4010 ..."
  python3 scripts/mock-server.py --port 4010 &
  @sleep 1
  @echo "Mock server started."

# Stop mock server
sdk-mock-stop:
  @echo "Stopping mock server ..."
  -pkill -f "mock-server.py" || true
  @echo "Mock server stopped."

# Run SDK integration tests (starts server, runs tests, stops server)
sdk-test-integration:
  ./scripts/prism-test.sh

# Check SDK endpoint coverage against OpenAPI spec
sdk-check-coverage:
  python3 scripts/check_coverage.py

# ============================================================
# CLI Commands
# ============================================================

# Build the CLI binary
cli-build:
  cargo build -p elevenlabs-cli

# Run CLI clippy lints
cli-lint:
  cargo clippy -p elevenlabs-cli -- -D warnings

# Run the CLI (pass arguments after --)
cli-run *ARGS:
  cargo run -p elevenlabs-cli -- {{ARGS}}

# Install the CLI locally
cli-install:
  cargo install --path bin/elevenlabs-cli

# ============================================================
# Publishing
# ============================================================

# Publish all crates to crates.io
publish:
  cargo publish --workspace
