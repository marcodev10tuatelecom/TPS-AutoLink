# AUTO-00 — Primary Source Register

This file records primary sources used to freeze foundation decisions.

## Rust stable toolchain

- Rust Release Team, **Announcing Rust 1.98.0**, 2026-08-20:
  https://blog.rust-lang.org/releases/latest/
- Rust Forge, current release versions:
  https://forge.rust-lang.org/

Decision derived from those sources:

```text
AUTO-00_RUST_TOOLCHAIN = 1.98.0
```

## rust-toolchain.toml

- The rustup book — Overrides / toolchain file:
  https://rust-lang.github.io/rustup/overrides.html
- The rustup book — Profiles:
  https://rust-lang.github.io/rustup/concepts/profiles.html

The project pins `1.98.0`, uses the `minimal` profile, and explicitly adds
`rustfmt` and `clippy`.

## GitHub Actions checkout

- Official repository:
  https://github.com/actions/checkout
- Official tag:
  `v7.0.1`
- Tag commit resolved through the GitHub API on 2026-08-26:

```text
3d3c42e5aac5ba805825da76410c181273ba90b1
```

The workflow pins the full commit SHA rather than a moving major tag.

## GitHub-hosted Ubuntu 24.04 runner

- Official runner image documentation:
  https://github.com/actions/runner-images/blob/main/images/ubuntu/Ubuntu2404-Readme.md

The image documents Rustup as preinstalled. The workflow still installs the pinned
Rust 1.98.0 toolchain explicitly rather than accepting the image's preinstalled Rust version.

## Evidence boundary

These sources establish toolchain/CI facts only. They do not prove that the TPS AutoLink
workspace compiles. Compilation remains `NOT_EXECUTED` until actual Rust CI or a verified local
Rust 1.98.0 environment runs the gate commands.
