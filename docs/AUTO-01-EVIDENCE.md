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
| Reference phone | UNKNOWN | Not yet supplied/approved |
| Reference head unit | UNKNOWN | Not yet supplied/approved |
| Hardware matrix | BLOCKED | `docs/hardware/REFERENCE-HARDWARE.md` |
| Discovery implementation | NOT_STARTED | Blocked by AUTO01-B001 |
| Positive discovery test | NOT_EXECUTED | Requires approved physical targets |
| Negative discovery test | NOT_EXECUTED | Requires approved physical targets |
| Physical PASS | NOT_PROVEN | Simulator cannot substitute for physical test |

## Blocker

`AUTO01-B001` is OPEN. No discovery implementation is authorized to assume a transport
or device capability until the required reference hardware matrix is supported by evidence.
