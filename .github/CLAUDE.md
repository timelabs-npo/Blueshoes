# Blueshoes — Agent Context

## Project
- **Name:** Blueshoes
- **Core Principle:** Operator Sovereignty, Invariant Safety & Capability-Based Execution
- **Environment:** Highly adversarial networks (state-level censorship, DPI, active probing)

## The Sovereignty Prosthetic Paradigm
- **Hermetic Focus & Zero Broadcast:** The edge agent operates fully autonomously offline. Cloud connections (e.g., GCP Firestore, Logging) are strictly asynchronous sync layers, not the control plane. Mute all broadcast telemetry during critical operations.
- **Edge-Authoritative (Local Neural Method):** Local processing always takes precedence over corporate cloud lag. If the external network goes dark, local execution and the `0.log` state remain 100% functional (The Right-to-Repair axiom).

## Architecture (The DTS / 0.log Vision)
- **The Genesis Log (0.log) is the Source of Truth:** All system state transitions, capabilities, and configurations must be event-sourced from the foundational journal log. The system does NOT implicitly trust the current mutable network state (which can be manipulated by DPI or state actors); it trusts the deterministic log of verified capabilities.
- **Deterministic Tribunal State (DTS):** Network transitions only become permanent if they survive adversarial probing (netchecks) and are explicitly confirmed. If they fail, the watchdog forces a rollback to the last verified log state.
- **Degrade Gracefully:** Components must poll independently and gracefully handle network blackouts.

## Conventions
- Rust strictness: use exact capabilities, no raw shell `ip route` execution outside `src/executor`.
- No grep-based security; enforce types.
- Commits must use conventional formats: `feat()`, `fix()`, `chore()`.
