# Blueshoes

Blueshoes is a rollback-safe adaptive networking runtime doctrine and reference implementation for constrained edge devices.

It operates primarily on FreeBSD-based routers to provide resilient routing, bounded recovery, and deterministic rollback without risking total loss of internet connectivity.

All `bs-edge-agent` execution capabilities are locked behind an explicit double-gate runtime acknowledgement and isolated by a `dangerous_execution` compile-time feature. 

**Default Behavior (Dry Run)**:
```bash
bs-edge-agent canary
```
*Outputs a hashed deterministic dry-run plan containing `plan_sha256`. Execution is aborted safely.*

**Execution Override (Double Gate)**:
```bash
bs-edge-agent --unsafe-execute --confirm unsafe:<request_id> canary
```
Read more in [Phase 1 Scope](docs/rfcs/0012-phase1-scope.md).

The project architecture and doctrine are maintained as an RFC corpus. See the [docs/rfcs](docs/rfcs/) directory for the complete doctrine surface.

- [Runtime Doctrine](docs/rfcs/0001-runtime-doctrine.md)
- [Rollback Model](docs/rfcs/0002-rollback-model.md)
- **Topics**: FreeBSD, router, networking, rollback, reliability, rust, dns, ech, edge-computing, observability, censorship-resilience

## Core Philosophy: Rollback is Sacred

Programmatic routing mutation is dangerous. A broken firewall or routing rule can permanently disconnect the user from the network.

Blueshoes treats every routing mutation as a bounded transaction:

1. Observe the current state.
2. Apply a constrained profile.
3. Validate connectivity.
4. Roll back automatically on failure.

The runtime must fail safely, deterministically, and recoverably.

## Scope

Phase 1 targets the GL.iNet GL-MT3000 (FreeBSD) with a deterministic edge agent written in Rust.

## Explicit Constraints

- No MITM/TLS interception.
- No autonomous shell mutation by LLMs.
- ECH is observed and preserved, not forced.
- Blueshoes does not ship with bundled commercial VPN endpoints or “one-click paid tunnel” defaults.
- No opaque orchestration layers in the runtime path.
- Human override remains mandatory for high-risk operations.

Read more in [Phase 1 Scope](docs/phase1-scope.md).

- [Core Doctrine](docs/doctrine.md)
- [System Architecture](docs/architecture.md)
- [MITM Ban](docs/mitm-ban.md)
- [ECH Position](docs/ech-position.md)
- [Profiles](docs/profiles.md)
- [Transaction Model](docs/transaction-model.md)

- [Security Policy](SECURITY.md)

## Status

Current status: B0 Runtime Beta Pack.

The runtime currently supports:
- Read-only telemetry probes
- Structured transaction journaling
- Cross-compilation for FreeBSD targets
- Deterministic audit validation

The runtime does NOT yet mutate routing state.
