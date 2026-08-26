# TPS AutoLink

TPS AutoLink is a Rust-first automotive infotainment and connectivity platform.

## Canonical project control

`TPS-AUTOLINK-PROJECT-CONTROL.md` is the operational authority for this repository.

Current canonical gate for this foundation package: **AUTO-00**.

## AUTO-00 scope

AUTO-00 establishes only:

- the Rust workspace;
- repository structure;
- pinned Rust toolchain;
- CI;
- formatting/lint/test/build commands;
- the initial simulator;
- the frozen **protocol v1 skeleton**;
- the initial evidence workflow.

Streaming, Android Auto, CarPlay, complete cache, final UI and physical vehicle testing
are explicitly outside AUTO-00.

## Toolchain

Pinned by `rust-toolchain.toml`:

- Rust 1.98.0
- rustfmt
- clippy

## Verify

```bash
rustc --version
cargo --version
cargo fmt --check
cargo check --workspace --all-targets
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo test --workspace --all-targets
cargo build --workspace --release
cargo run -p tps-auto-simulator --release -- --self-test
```

Expected simulator self-test markers:

```text
TPS_AUTOLINK_SIMULATOR=READY
PROTOCOL=TPS AutoLink
PROTOCOL_VERSION=1.0
SELF_TEST=PASS
```

## Status semantics

- `PASS`: supported by recorded evidence.
- `NOT_EXECUTED`: command has not been run.
- `NOT_PROVEN`: evidence is insufficient.
- `UNKNOWN`: the fact is not yet known.

Code being present is not equivalent to `DONE`.
