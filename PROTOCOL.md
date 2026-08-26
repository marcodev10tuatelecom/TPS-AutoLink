# TPS AutoLink Protocol v1 — Frozen Skeleton

**Gate:** AUTO-00  
**Skeleton status:** FROZEN for v1 foundation  
**Complete wire protocol status:** NOT_DEFINED in AUTO-00

## 1. Purpose

This document freezes only the minimum protocol architecture needed to prevent incompatible
foundations while avoiding premature implementation of later gates.

## 2. Frozen identifiers

```text
PROTOCOL_NAME  = TPS AutoLink
PROTOCOL_MAJOR = 1
PROTOCOL_MINOR = 0
```

Major-version compatibility rule:

```text
compatible := local.major == remote.major
```

A minor-version difference alone does not make peers incompatible at the skeleton level.

## 3. Frozen logical envelope fields

Every future TPS AutoLink application message must be representable by a logical envelope
containing these fields:

```text
protocol_version
message_kind
request_id
payload_length
payload
```

AUTO-00 freezes the **existence and meaning** of these fields, not their final byte-level encoding.

## 4. Initial message kinds reserved by AUTO-00

```text
HELLO         = 1
CAPABILITIES  = 2
ERROR         = 255
```

These values exist only to exercise and test the foundation simulator.

## 5. Payload limit used by the foundation

The AUTO-00 protocol crate enforces:

```text
MAX_PAYLOAD_LEN = 1,048,576 bytes
```

This is a defensive foundation limit. A later gate may lower it for specific message classes.
Increasing it requires an approved protocol decision because it changes an accepted resource bound.

## 6. Explicitly NOT defined or frozen by AUTO-00

The following are intentionally deferred because they belong to later gates:

- transport selection;
- byte order / final binary wire encoding;
- discovery packets;
- pairing messages;
- authentication messages;
- cryptographic primitives;
- session resumption;
- media payloads;
- cache protocol;
- network handover protocol;
- Android/Apple adapters.

No later gate may silently alter the frozen skeleton. Incompatible changes require explicit
protocol versioning and approved change control.

## 7. Security invariant

Untrusted payload lengths must be checked before allocation or processing.

The Rust foundation crate contains no `unsafe` code.
