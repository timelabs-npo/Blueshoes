# MODULE_INVENTORY.md

This inventory documents all module roots in the Blueshoes repository, their purpose, safety relevance, and test coverage.

## Module Roots

| Module Path | Purpose | Status | Safety Relevance | Known Risks | Tests |
| :--- | :--- | :--- | :--- | :--- | :--- |
| `runtime/bs-edge-agent/` | Core Rust implementation of the edge agent. | **ACTIVE** | **CRITICAL** | Raw shell escape, state drift, watchdog failure. | `hygiene_linter_test.rs`, `test_rollback.py`, semantic tests. |
| `runtime/bs-edge-agent/src/executor/` | Capability execution boundary. | **ACTIVE** | **CRITICAL** | Command injection, privilege escalation. | Part of core suite. |
| `runtime/bs-edge-agent/src/mutation/` | State transition and rollback logic. | **ACTIVE** | **CRITICAL** | Irreversible mutation, data loss. | `test_rollback.py`. |
| `scripts/` | Python-based operational utilities and audits. | **ACTIVE** | **HIGH** | Script injection, credential leakage in logs. | Healthchecks (`bs_healthcheck.py`). |
| `packaging/` | OS-specific (FreeBSD/OpenWrt) distribution. | **ACTIVE** | **MEDIUM** | Boot-time failure, RC script privilege. | N/A (requires target environment). |
| `docs/rfcs/` | Architecture decision records. | **ACTIVE** | **LOW** | Stale documentation leading to wrong implementation. | N/A |
| `spanner_demo/` | Go-based demonstration of GCP Spanner sync. | **EXPERIMENTAL** | **LOW** | Confusion with runtime source of truth. | N/A |
| `artifacts/devship/` | Local operational artifacts and tribunal requests. | **ACTIVE** | **MEDIUM** | Information leakage, stale requests. | N/A |

## Inventory Summary

- **Core Runtime**: Rust (Cargo workspace).
- **Automation**: Python 3.x.
- **Documentation**: Markdown (Doxygen-style / GitBook compatible).
- **Packaging**: Makefile / FreeBSD Ports / OpenWrt SDK.
- **Verification Coverage**: High on integration/hygiene, low on unit tests for certain modules.
