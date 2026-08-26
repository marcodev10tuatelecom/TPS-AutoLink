# AUTO-00 — Decision Register

## AD-0001 — Rust toolchain pin

```text
DATE: 2026-08-26
GATE: AUTO-00
QUESTION: Which Rust release defines the foundation toolchain?
EVIDENCE: Rust stable 1.98.0 was released on 2026-08-20.
DECISION: Pin Rust 1.98.0 in rust-toolchain.toml.
REASON: Reproducible gate foundation; avoid a floating stable channel.
STATUS: FROZEN
REOPEN_CONDITIONS: FAILURE_EVIDENCE | SECURITY_EVIDENCE | REQUIREMENT_CHANGE | USER_AUTHORIZATION
```

## AD-0002 — AUTO-00 dependency policy

```text
DATE: 2026-08-26
GATE: AUTO-00
QUESTION: Which third-party Rust crates are required by the foundation?
DECISION: None.
REASON: The AUTO-00 protocol skeleton, core identity and simulator do not require external crates.
STATUS: FROZEN
REOPEN_CONDITIONS: ACTIVE_GATE_REQUIREMENT | FAILURE_EVIDENCE | USER_AUTHORIZATION
```

## AD-0003 — Host CI reference

```text
DATE: 2026-08-26
GATE: AUTO-00
QUESTION: Which host validates the foundation before physical head-unit hardware is selected?
DECISION: GitHub Actions Ubuntu 24.04 x86_64 with Rust 1.98.0.
REASON: AUTO-00 requires a reproducible software foundation; physical head-unit hardware is not in scope.
STATUS: FROZEN
REOPEN_CONDITIONS: FAILURE_EVIDENCE | REQUIREMENT_CHANGE | USER_AUTHORIZATION
```

The physical head-unit CPU/board remains `UNKNOWN` until the required reference-hardware matrix
is established before AUTO-01.
