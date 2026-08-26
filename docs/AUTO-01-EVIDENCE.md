# AUTO-01 Evidence Record

**Gate:** AUTO-01  
**Status:** IN_PROGRESS  
**Started:** 2026-08-26

## Objective

Controlled discovery between the approved reference phone and approved reference head unit.

## Required deliverables

- device identity;
- presence announcement;
- discovery;
- timeout;
- retry;
- peer selection;
- logs;
- positive discovery test;
- negative discovery test.

## Exit criterion

The approved phone detects the approved head unit and the head unit identifies the expected phone.

## Current evidence

| Item | State | Evidence |
|---|---|---|
| Reference phone family | PROVEN | User identified Apple iPhone 8 Plus; Apple primary specifications recorded in `docs/hardware/REFERENCE-HARDWARE.md` |
| Exact iPhone variant | UNKNOWN | A1864/A1897/A1898 not yet identified |
| iPhone OS version | UNKNOWN | Must be read from Settings > General > About |
| Reference head unit | UNKNOWN | No model/system information captured yet |
| Hardware matrix | BLOCKED | `docs/hardware/REFERENCE-HARDWARE.md` |
| Discovery implementation | NOT_STARTED | Blocked by AUTO01-B001 |
| Positive discovery test | NOT_EXECUTED | Requires approved physical targets |
| Negative discovery test | NOT_EXECUTED | Requires approved physical targets |
| Physical PASS | NOT_PROVEN | Simulator cannot substitute for physical test |

## Blocker

`AUTO01-B001` is OPEN. The phone family is now known, but AUTO-01 still cannot select or
certify a discovery transport until the exact iOS version and discovery-relevant head-unit
capabilities are evidenced from the physical displays or primary documentation.
