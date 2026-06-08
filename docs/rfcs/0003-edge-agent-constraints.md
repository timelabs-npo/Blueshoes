# Hardware Budgets: MT-3000 Target

For Phase 1, we are targeting the GL.iNet GL-MT3000 (MediaTek MT7981B). The router has 512MB RAM and 256MB Flash, but we must leave the vast majority of these resources free for base FreeBSD routing and system upgrades.

## Hypothesized Footprint Targets
To ensure Blueshoes operates invisibly without causing Out-of-Memory (OOM) events or filling the flash drive, we are aiming for the following budgets for the `bs-edge-agent`:

1. **RAM Target**: $< 15\text{MB}$ during idle and observation.
2. **Flash Target**: $< 5\text{MB}$ to ensure `sysupgrade` doesn't fail.
3. **Database Limit**: Local SQLite telemetry logs should be capped at $2\text{MB}$ via FIFO truncation.

*Note: These are operational hypotheses. Final language choice (Rust vs Go) and compilation flags will be determined once we begin footprint profiling on the physical hardware.*
