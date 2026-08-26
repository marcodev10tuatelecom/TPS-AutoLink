# State Transition — AUTO-00 Start

```text
DATE: 2026-08-26
PROJECT: TPS AutoLink
FROM: AUTO-00 / READY
TO: AUTO-00 / IN_PROGRESS
AUTHORIZATION: USER ("vamos comecar com o AUTO-00")
ROADMAP_CHANGE: NO
GATE_CHANGE: NO
FROZEN_DECISION_REOPENED: NO
```

The transition is permitted by the canonical state machine:

```text
READY -> IN_PROGRESS
```

## Repository authorization follow-up

```text
AUTO00-B001
Initial state: BLOCKED — canonical GitHub repository not writable by the integration.
Resolution: GitHub App installation authorized for marcodev10tuatelecom/TPS-AutoLink.
Verification: first write commit e13ac60834fb75b1cce80cdc312467ffac7d6fcc succeeded.
Status: RESOLVED
```

AUTO-00 remains `IN_PROGRESS` because build, lint, test, release-build and simulator evidence are still pending.
