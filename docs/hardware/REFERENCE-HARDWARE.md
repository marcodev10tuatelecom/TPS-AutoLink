# TPS AutoLink — Reference Hardware Matrix

**Gate:** AUTO-01  
**Status:** BLOCKED_PENDING_USER_HARDWARE  
**Authority:** TPS-AUTOLINK-PROJECT-CONTROL.md §27

AUTO-01 may not assume unproven device capabilities. The exact physical reference
phone and head unit must be recorded before discovery implementation.

## Canonical matrix

```text
PHONE_MODEL: UNKNOWN
PHONE_OS: UNKNOWN
HEAD_UNIT_MODEL: UNKNOWN
HEAD_UNIT_OS: UNKNOWN
CPU_ARCH: UNKNOWN
RAM: UNKNOWN
STORAGE: UNKNOWN
DISPLAY: UNKNOWN
AUDIO: UNKNOWN
WIFI: UNKNOWN
BLUETOOTH: UNKNOWN
USB: UNKNOWN
POWER: UNKNOWN
```

## Evidence rule

A field may move from `UNKNOWN` only when supported by at least one of:

- device settings / system information from the physical unit;
- manufacturer or platform primary documentation for the exact model/version;
- direct measurement or command output from the exact reference hardware.

Retail listings, assumptions based on similar models, and generic family specifications
do not prove the capability of the selected reference unit.

## Approval

The matrix is not `APPROVED` until the user explicitly identifies/approves the exact
reference phone and exact reference head unit.

## Blocking record

```text
BLOCKER-ID: AUTO01-B001
GATE: AUTO-01
DESCRIPTION: Exact approved reference phone/head-unit hardware matrix is not yet defined.
IMPACT: AUTO-01 physical discovery cannot be implemented or certified against an approved target.
MINIMUM_FIX: Provide the exact phone model + OS and exact head-unit model + OS; then complete the remaining matrix fields from primary/device evidence.
VERIFICATION: Matrix contains no required UNKNOWN field for discovery-relevant capabilities and is explicitly approved.
OWNER: USER / physical hardware selection
STATUS: OPEN
```
