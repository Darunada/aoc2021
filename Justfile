# just manual: https://github.com/casey/just/#readme

_default:
    @just --list
run:
    cargo run

check:
    circleci config validate
    cargo clippy --locked -- -D warnings

check_ci:
    cargo clippy --locked -- -D warnings

test:
    cargo test --locked
