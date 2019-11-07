build:
    cargo build
clean:
    cargo clean
test:
    cargo test --all-targets
    cargo test --all-targets --release
    cargo test --all-targets --release --features debug-only
update:
    cargo update
clippy:
    cargo clippy --all-targets --tests --all-features
bench:
    cargo bench
pedantic:
    cargo clippy --all-targets --tests --features pedantic
rustfmt:
    cargo fmt --all -- --check
fmt: rustfmt
check: fmt update test clippy
