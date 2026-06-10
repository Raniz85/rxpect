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

feature_tests:
    #!/usr/bin/env sh
    cargo test --no-default-features
    for f in diff iterables diff,iterables; do
         cargo test --no-default-features -F $f
    done

ci: checkFormat lint test

prep_commit: format fix_lints test

