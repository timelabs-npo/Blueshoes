# Phase 1 Scope

The initial development phase of Blueshoes focuses strictly on validating the transactional routing engine on constrained hardware. 

## In Scope for Phase 1
- **Target Hardware**: GL.iNet MT-3000 (MediaTek MT7981B).
- **Core Engine**: A bare-metal daemon (`bs-edge-agent`) running natively on FreeBSD.
- **Rollback Safety**: Implementing the Snapshot $\to$ Apply $\to$ Validate $\to$ Rollback loop.
- **Static Profiles**: Basic fallback profiles (e.g., encrypted DNS upstreams, ECH preservation behaviors, and optionally an explicit operator-configured tunnel profile with no bundled commercial endpoints).
- **Basic Telemetry**: Logging connection failures (TCP RST, DNS timeouts) to a local SQLite database.

## Out of Scope for Phase 1 (Deferred)
- **Global Mesh Telemetry**: Exchanging telemetry between router nodes globally to map censorship topographies. This is deferred to avoid premature privacy/legal risks while we stabilize the core engine.
- **Automated AI Loop**: Connecting the LLM directly to the router's profile switching logic. (Currently requires manual approval).
- **Universal Hardware Support**: Deploying to x86 or Broadcom-based routers.
