# Blueshoes Roadmap

## Phase 1: Core Engine Stabilization (Current)
- **[DONE] M1: Read-only Telemetry Probes**. `bs-edge-agent` safely reads Linux state and network endpoints.
- **[DONE] M1.5: Cross-Compilation Scaffold**. Established the `aarch64-unknown-linux-musl` target.
> [!IMPORTANT]  
> **M1.6: AGY-supervised Router Smoke Test** (CURRENT BLOCKER)
> *Task:* Antigravity may execute the smoke-test commands, but only from an approved task envelope, only against the explicitly configured router IP, and only with read-only runtime commands after copying the binary to `/tmp`. M2 cannot begin until verified.

## Phase 2: Transaction Engine & Edge Intelligence
- **M2: Local Dummy Journal**.
- **M3: Profile Schema Parser**. JSON validation of routing intents (e.g., "DNS_PRIVACY").
- **M4: Dry-Run Transaction Planner**. Calculate `ip route` and `nftables` diffs strictly in memory.
- **M5: The Fake Rollback Loop**. Test atomic rollback triggers without touching the system.
- **M6: FreeBSD Adapter**. Bind the dry-run planner to actual `uci` and `netifd` execution APIs.
- **M7: Canary Validation**. Perform a 30-second mutation and auto-rollback to guarantee failsafes.
- **M8: First Controlled Mutation**. Execute the first permanent, deterministic routing update.

## Phase 3: Advanced Obfuscation
- Implement fine-grained DNS-over-HTTPS/DoT fallbacks with strict privacy controls and clear user-visible configuration.
- Add support for optional, explicitly configured tunnel transports where the operator supplies and controls the egress endpoint. The project must not ship bundled commercial VPN endpoints, paid defaults, or covert monetization hooks.

## Phase 4: Deferred Complexity (The Global Mesh)
- **Opt-in Telemetry Exchange**: Evaluate peer-to-peer sharing of anonymized capability data between nodes. The goal is to crowdsource routing paths to defeat censorship at scale, structurally disrupting legacy VPN monetization models.
- **Cooperative SOCKS**: Allow explicit client opt-in proxies for advanced tracing, with strong abuse-resistance and no transparent interception.

## Deferred / Rejected Complexity
- **Enterprise Orchestration**: No active CI/CD platforms, external webhooks, or automated queueing systems. The AI toolchain governance is strictly enforced via passive, local file locks in `.tasks/` (see `docs/dev-workflow.md`).
