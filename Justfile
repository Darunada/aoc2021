# just manual: https://github.com/casey/just/#readme

_default:
    @just --list

check:
    circleci config validate
    cargo clippy --locked -- -D warnings

test:
    cargo test --locked
