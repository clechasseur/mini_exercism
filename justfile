set windows-shell := ["powershell.exe", "-NoLogo", "-Command"]

default: test

tidy: clippy fmt

clippy:
    cargo clippy --workspace --all-targets --all-features -- -D warnings

fmt:
    cargo +nightly fmt --all

test:
    cargo test --workspace --all-features

tarpaulin:
    cargo tarpaulin --target-dir target-tarpaulin

doc:
    cargo +nightly doc --workspace --no-deps --all-features --open

msrv:
    mv Cargo.toml Cargo.toml.bak
    mv Cargo.lock Cargo.lock.bak
    mv Cargo.toml.msrv Cargo.toml
    mv Cargo.lock.msrv Cargo.lock
    cargo msrv -- cargo check --workspace --lib --all-features
    rm Cargo.toml
    rm Cargo.lock
    mv Cargo.toml.bak Cargo.toml
    mv Cargo.lock.bak Cargo.lock
