# Contributing to Blueshoes

Blueshoes accepts contributions that preserve deterministic safety and doctrinal constraints.

## What Good Contributions Look Like

- Preserve rollback-first behavior and explicit safety gates.
- Keep runtime behavior transparent and operator-controlled.
- Avoid adding opaque orchestration or hidden monetization paths.
- Produce deterministic evidence (tests, artifacts, reproducible outputs).

## Contribution Guardrails

1. **No MITM behavior**: no TLS interception or synthetic root certificate flows.
2. **No autonomous unsafe mutation**: state mutation must remain explicitly human-gated.
3. **No covert monetization paths**: no bundled paid endpoints, affiliate defaults, or hidden rerouting logic.
4. **Keep doctrine aligned**: RFC corpus is the architecture source of truth.

## Recommended Workflow

1. Read [Runtime Doctrine (RFC 0001)](rfcs/0001-runtime-doctrine.md) and [Phase 1 Scope (RFC 0012)](rfcs/0012-phase1-scope.md).
2. Build and run tests locally before proposing changes.
3. Include documentation updates when behavior, policy, or boundaries change.
4. For risky architectural changes, include explicit rationale and rollback impact.

## Public-Facing Quality Standard

All externally visible docs should optimize for:
- clear value proposition,
- verifiable claims,
- working links,
- concise onboarding,
- legal/security clarity.

See the latest audit in [docs/audits/2026-06-07-marketing-legal-audit.md](../docs/audits/2026-06-07-marketing-legal-audit.md).
