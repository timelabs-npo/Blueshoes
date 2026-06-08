# bs-edge-agent

This is the primary runtime daemon for the Blueshoes architecture. It is designed to run natively on FreeBSD routers (specifically targeting the GL.iNet MT-3000 in Phase 1).

## Constraints
- **Language**: Rust (currently scaffolded).
- **Architecture**: Aarch64 (Cortex-A53).
- **Memory Budget**: < 15MB RSS.
- **Flash Budget**: < 5MB (stripped binary).

## Responsibilities
1. Monitor network health (netcheck).
2. Execute the atomic transaction loop (Snapshot $\to$ Apply $\to$ Validate $\to$ Rollback).
3. Log failures to local SQLite telemetry.

This agent operates strictly under the [Runtime Doctrine](../docs/rfcs/0001-runtime-doctrine.md). It does not guess, it does not use AI internally, and it does not perform transparent MITM interception.
