@_list:
	just --list --unsorted

fmt:
   cargo fmt

web:
    trunk serve --open

check-all:
    cargo fmt --all --check
    cargo clippy -- -D warnings
    cargo check --all-features --lib --target wasm32-unknown-unknown