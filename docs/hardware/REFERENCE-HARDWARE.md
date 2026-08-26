# TPS AutoLink — Reference Hardware Matrix

**Gate:** AUTO-01  
**Status:** BLOCKED_PENDING_REFERENCE_DETAILS  
**Authority:** TPS-AUTOLINK-PROJECT-CONTROL.md §27

AUTO-01 may not assume unproven device capabilities. The exact physical reference
phone and head unit must be recorded before discovery implementation.

## Canonical matrix

```text
PHONE_MODEL: Apple iPhone 8 Plus (exact model number A1864/A1897/A1898 = UNKNOWN)
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

## Proven phone facts from Apple primary documentation

The following facts are proven for the iPhone 8 Plus family and may be used only where
family-wide capability is sufficient:

```text
PHONE_FAMILY: Apple iPhone 8 Plus
PHONE_CHIP: A11 Bionic
PHONE_DISPLAY: 5.5-inch Retina HD LCD, 1920x1080
PHONE_WIFI: 802.11ac with MIMO
PHONE_BLUETOOTH: Bluetooth 5.0
PHONE_CELLULAR: LTE family support; exact band/model variant remains UNKNOWN
PHONE_CONNECTOR: Lightning
PHONE_STORAGE_OPTIONS: 64 GB / 128 GB / 256 GB; exact unit capacity UNKNOWN
```

Primary sources:

- Apple Support BR — iPhone 8 Plus technical specifications:
  https://support.apple.com/pt-br/111950
- Apple Support BR — Identify your iPhone model:
  https://support.apple.com/pt-br/108044

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
DESCRIPTION: Exact approved reference hardware matrix is incomplete.
EVIDENCE: Phone family identified as Apple iPhone 8 Plus; exact iOS version and exact head-unit identity/OS remain UNKNOWN.
IMPACT: AUTO-01 physical discovery cannot be implemented or certified against an approved target.
MINIMUM_FIX: Read the iPhone Software Version from Settings > General > About and capture the head-unit system/about screens visible on the display.
VERIFICATION: Required discovery-relevant fields are supported by device/primary evidence and explicitly approved.
OWNER: USER / physical hardware evidence
STATUS: OPEN
```
