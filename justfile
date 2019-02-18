build:
    cargo build
clean:
    cargo clean
test:
    cargo test --all
update:
    cargo update
clippy:
    cargo clippy --all --tests --all-features
bench:
    cargo bench
pedantic:
    cargo clippy --all --tests --features pedantic
rustfmt:
    cargo fmt --all -- --check
fmt: rustfmt
check: fmt update test clippy
