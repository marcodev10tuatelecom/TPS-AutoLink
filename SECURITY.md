# Security Policy — TPS AutoLink v1.0.0

## Scope

TPS AutoLink v1.0.0 is infotainment software. Safety-critical vehicle functions are
out of scope under the canonical project-control document.

## Mandatory rules

- Never commit private keys, passwords, tokens, signing secrets or production credentials.
- Validate all untrusted protocol input.
- Do not invent cryptographic primitives.
- Keep `unsafe` and FFI minimal, isolated, documented and audited.
- Do not bypass OEM, DRM, platform or licensing protections.
- Do not use production secrets in development logs or test fixtures.

## AUTO-00

AUTO-00 intentionally contains no application secrets and no cryptographic implementation.
The protocol crate forbids `unsafe` code at crate level.

## Reporting

Until a repository-specific private reporting channel is configured, security findings must
not include secrets or exploitable production credentials in public issue text.
