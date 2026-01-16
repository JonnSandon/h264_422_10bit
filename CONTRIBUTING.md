# Contributing

## Toolchain

- Rust 1.92.0
- Edition 2024

The project declares its MSRV in `Cargo.toml` and `.rust-toolchain.toml`.

## Workflow

- `cargo fmt`
- `cargo clippy --all-targets -- -D warnings`
- `cargo test --all`
- `cargo bench` (optional)

## Scope

This crate is **decode-only** and **analysis-focused**:

- No encoding
- No realtime guarantees
- No unsafe FFI to FFmpeg
