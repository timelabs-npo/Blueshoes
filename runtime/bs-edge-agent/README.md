# bs-edge-agent

`bs-edge-agent` is the deterministic runtime daemon for Blueshoes, designed for OpenWrt routers (Phase 1 target: GL.iNet MT-3000).

## Runtime Constraints

- **Language**: Rust
- **Architecture**: Aarch64 (Cortex-A53)
- **Memory Budget**: `< 15MB` RSS target
- **Flash Budget**: `< 5MB` stripped binary target

## Core Responsibilities

1. Collect read-only network telemetry probes.
2. Run transaction planning and journal evidence output.
3. Enforce explicit execution gates for any unsafe path.

## Safety Model

- Default mode is dry-run.
- Unsafe execution requires explicit double confirmation.
- `dangerous_execution` feature gate isolates mutation-capable build paths.

## Doctrine Link

This runtime follows the [Core Runtime Doctrine (RFC 0001)](../../docs/rfcs/0001-runtime-doctrine.md) and [Rollback Model (RFC 0002)](../../docs/rfcs/0002-rollback-model.md).
