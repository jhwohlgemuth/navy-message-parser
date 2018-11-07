lint:
    cargo fmt
    cargo clippy

test: lint
    cargo test
