# https://just.systems/man/en/

# Show all tasks by default
_default:
    @just --list

# Run the unit tests
test *ARGS='':
    cargo nextest run {{ ARGS }}

# Run clippy linting
lint:
    cargo clippy --all-features --all-targets

# Run testing and linting in one go
check: test lint

# Show test coverage (requires cargo-llvm-cov)
cov:
    cargo llvm-cov nextest --open

# Show the documentation
doc:
    cargo doc --open

# Build the binary
build *FLAGS='':
    cargo build {{ FLAGS }}
