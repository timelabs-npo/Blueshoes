# Constitution

## Governance Model

- **Tribunal Architecture**: Consensus‑oriented cognition with roles `verifier`, `critic`, `governor`, `observer`.
- **Deterministic Boundaries**: Hard execution constraints enforced by the **governor** layer (rollback, watchdog, immutable invariants).
- **Evidence Persistence**: All decisions are persisted as provenance records in the semantic substrate.

## Core Principles

1. **No Irreversible Mutation** – Every state transition must be verifiable and reversible.
2. **Governance‑First** – Protocol, language, and doctrine are defined before any UI.
3. **Thin Viewports** – Front‑ends are merely cognition viewports; they do not affect core logic.

## Roles

- **Verifier**: Low‑temperature, high‑consistency model that enforces schema adherence.
- **Critic**: High‑diversity model that seeks contradictions and adversarial reasoning.
- **Governor**: Deterministic local logic (Python/Rust) that enforces hard invariants.
- **Observer**: Lightweight local/open model for monitoring and telemetry.

## Enforcement

- `bs-watchdog` ensures timeout‑based rollback.
- Governor layer provides hard enforcement of invariants.
- Tribunal decisions are recorded in this constitution and the semantic substrate.
