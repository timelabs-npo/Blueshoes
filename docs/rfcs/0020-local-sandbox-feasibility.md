# Local Sandbox Feasibility (GL-MT3000)

**Verdict: Phase 1 Sandbox (LXC/Docker) is INFEASIBLE.**

The GL.iNet MT3000 provides 512MB RAM and 256MB Flash.
- **Docker**: Requires significant RAM overhead. Running it on 512MB RAM alongside standard FreeBSD routing processes risks Memory Exhaustion and OOM Killer events, which breaks standard router functionality.
- **LXC / Debian proot**: Requires significant flash storage to house the Debian userland. The MT3000's 256MB flash is largely consumed by the base OS. Without an external USB drive, a containerized sandbox will fail to install or will fill the flash, violating the "non-destructive removability" constraint.

**Conclusion**: All advanced diagnostic tools must live externally on the `bs-workbench`. The router itself runs only the bare-metal FreeBSD native `bs-edge-agent`.
