# System Architecture

The Blueshoes architecture is bifurcated into two distinct environments to protect the router's stability while allowing for advanced analysis.

## The Edge Runtime (`bs-edge-agent`)
This is the core daemon running directly on the FreeBSD router.
- **Role**: It monitors connection health, switches routing profiles, and enforces atomic rollbacks.
- **Constraints**: It must be extremely lightweight. We prefer memory-safe languages (Rust or Go), pending final compilation footprint tests.
- **Determinism**: The agent only executes pre-defined, static profiles. It does not guess or generate rules dynamically.

## The Analytics Workbench (`bs-workbench`)
This is an external environment (e.g., a local VM or a developer laptop) used for heavy lifting.
- **Role**: It processes telemetry databases and raw packet captures exported from the router.
- **LLM Integration**: It hosts the LLM logic that analyzes network pathologies to suggest new routing strategies.
- **Isolation**: Because it lives off-router, it cannot crash the network stack if it runs out of memory or hallucinates.

## State Mutation Flow
1. The Edge Agent detects a connection failure (e.g., TCP Resets).
2. The Agent snapshots the current working routing table.
3. The Agent applies a new fallback profile (e.g., DNS-over-HTTPS).
4. The Agent tests connectivity (Validation).
5. If the test fails, the Agent immediately reverts to the Snapshot.
