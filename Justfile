checkFormat:
    cargo fmt --all --check
format:
    cargo fmt --all

lint:
    cargo clippy --all-targets --tests --all-features

fix_lints:
    cargo clippy --all-targets --tests --all-features --fix --allow-dirty

test:
    cargo test

ci: checkFormat lint test
