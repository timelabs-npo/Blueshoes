# Blueshoes

![Blueshoes brand banner](docs/assets/brand/blueshoes-banner.svg)

Blueshoes is a rollback-safe, deterministic edge networking runtime for OpenWrt-class routers.

[![Security Policy](https://img.shields.io/badge/security-policy-0ea5e9)](SECURITY.md)
[![License: GPLv3](https://img.shields.io/badge/license-GPLv3-4f46e5)](LICENSE)
[![Runtime Doctrine](https://img.shields.io/badge/doctrine-RFC%200001-22d3ee)](docs/rfcs/0001-runtime-doctrine.md)

## Brand Statement (Revised)

**Resilient networking without hidden behavior:** Blueshoes keeps users online through deterministic rollback and human-gated execution, while preserving cryptographic integrity and avoiding covert monetization.

## Formal Promise Status (Audit Snapshot)

| Promise | Status | Evidence |
|---|---|---|
| No MITM / no synthetic root CA | ✅ Met | [SECURITY.md](SECURITY.md), [RFC 0005](docs/rfcs/0005-mitm-ban.md), [hygiene linter](runtime/bs-edge-agent/tests/hygiene_linter_test.rs) |
| Human-gated unsafe execution | ✅ Met | [CLI gate design](runtime/bs-edge-agent/src/main.rs), [safety tests](runtime/bs-edge-agent/tests/hygiene_linter_test.rs) |
| No covert monetization / no bundled commercial VPN defaults | ✅ Met | [SECURITY.md](SECURITY.md), [RFC 0001](docs/rfcs/0001-runtime-doctrine.md), [RFC 0018](docs/rfcs/0018-rejected-ideas.md) |
| Deterministic rollback-first transaction model | ⚠️ Partial | Doctrine and planner exist; mutation runtime intentionally gated and still beta ([Status](#status), [RFC 0002](docs/rfcs/0002-rollback-model.md)) |
| Non-destructive removability | ⚠️ Partial | Declared in doctrine; full live mutation lifecycle remains under staged rollout ([RFC 0001](docs/rfcs/0001-runtime-doctrine.md)) |

## Quick Start

### Dry-run canary (default safe mode)
```bash
bs-edge-agent canary
```

### Explicit unsafe execution (double gate)
```bash
bs-edge-agent --unsafe-execute --confirm unsafe:<request_id> canary
```

Execution path is additionally isolated by the `dangerous_execution` compile-time feature.

## Why Blueshoes

- **Rollback is sacred:** failed mutations are designed to fail closed and recover.
- **Human authority remains primary:** no autonomous LLM mutation authority.
- **Transparency over stealth:** no traffic decryption, no covert rerouting business logic.
- **Evidence-first operations:** journals, probes, and deterministic artifacts are first-class.

## Documentation

- [Core Runtime Doctrine (RFC 0001)](docs/rfcs/0001-runtime-doctrine.md)
- [Rollback Model (RFC 0002)](docs/rfcs/0002-rollback-model.md)
- [MITM Ban (RFC 0005)](docs/rfcs/0005-mitm-ban.md)
- [Architecture (RFC 0008)](docs/rfcs/0008-architecture.md)
- [Phase 1 Scope (RFC 0012)](docs/rfcs/0012-phase1-scope.md)
- [Security Policy](SECURITY.md)
- [Contributing](docs/contributing.md)

## Marketing + Legal Audit Deliverables

- [Comprehensive audit report](docs/audits/2026-06-07-marketing-legal-audit.md)
- [Promise register (machine-readable)](docs/audits/promise-register.json)
- [Benchmark scorecard](docs/assets/brand/brand-scorecard.svg)

## Status

Current status: **B0 Runtime Beta Pack**.

Current runtime capabilities:
- Read-only telemetry probes
- Structured transaction journaling
- Cross-compilation for OpenWrt targets
- Deterministic audit validation

The runtime currently does **not yet** mutate routing state by default and remains intentionally guarded behind explicit execution gates.
