# M1: Read-Only Telemetry Agent

The `bs-edge-agent` M1 milestone serves as a read-only telemetry probe. It executes local diagnostics and bounded network requests without mutating any state on the router (no UCI, nftables, or iproute2 mutations).

## Core Capabilities
- Sub-second system diagnostics (load, memory, OS)
- Basic routing awareness
- End-to-end DNS, ICMP, and HTTPS latency probes
- Strict offline journaling

## Execution & Journaling
All telemetry outputs are formatted as strictly defined JSON.
The agent appends events directly to the local journal path.
- **Primary Path**: `/var/lib/blueshoes/events.jsonl`
- **Fallback Path**: `./target/blueshoes-dev/events.jsonl`

### Netcheck Pipeline
The `netcheck` command triggers the orchestration of all probes sequentially:

```bash
cargo run -- netcheck
```

Example JSON Output:
```json
[
  {
    "timestamp_utc": 1780798729,
    "event_type": "telemetry_probe",
    "probe": "system",
    "status": "warn",
    "duration_ms": 2,
    "evidence": {
      "loadavg": null,
      "mem_free_kb": null,
      "mem_total_kb": null,
      "os": "Darwin MacBookPro.lan 25.5.0",
      "uptime": null
    },
    "mutation_performed": false
  },
  {
    "timestamp_utc": 1780798729,
    "event_type": "telemetry_probe",
    "probe": "dns",
    "status": "ok",
    "duration_ms": 54,
    "evidence": {
      "resolved_ips": [
        "8.6.112.10",
        "8.47.69.10"
      ],
      "target": "example.com"
    },
    "mutation_performed": false
  },
  {
    "timestamp_utc": 1780798729,
    "event_type": "telemetry_probe",
    "probe": "icmp",
    "status": "ok",
    "duration_ms": 5,
    "evidence": {
      "latency_line": "64 bytes from 1.1.1.1: icmp_seq=0 ttl=64 time=0.309 ms",
      "stdout": "PING 1.1.1.1 (1.1.1.1): 56 data bytes...",
      "target": "1.1.1.1"
    },
    "mutation_performed": false
  },
  {
    "timestamp_utc": 1780798730,
    "event_type": "telemetry_probe",
    "probe": "https",
    "status": "ok",
    "duration_ms": 704,
    "evidence": {
      "http_status": 200,
      "target": "https://example.com"
    },
    "mutation_performed": false
  }
]
```

*Notice `mutation_performed: false` is enforced natively in the Rust struct.*

### Tailing the Journal
To read the last `N` events from the active journal:
```bash
cargo run -- journal --tail 10
```

## Security Guarantees
1. **No Target Mutations**: The `TelemetryEvent` struct natively enforces `mutation_performed = false`. 
2. **Automated Audit**: During `cargo test`, `tests/audit_test.rs` rigorously scans the entire `src/` codebase to assert that forbidden strings (like `uci set` or `nft add`) do not exist. Any accidental inclusion of mutating commands will break the CI build.
## FreeBSD Deployment (M1.5 Cross-Compilation)
The GL-MT3000 router uses a MediaTek MT7981B chip (Dual-core ARM Cortex-A53). The correct Rust target for this architecture is `aarch64-unknown-linux-musl`.

To build the static binary for the router, we use the `cross` toolchain (which utilizes Docker to handle the C cross-compilation dependencies).

1. Install cross:
   ```bash
   make setup-cross
   ```
2. Build the router binary:
   ```bash
   make build-FreeBSD
   ```
3. The resulting statically linked executable will be located at:
   `runtime/bs-edge-agent/target/aarch64-unknown-linux-musl/release/bs-edge-agent`

This binary can be safely `scp`'d directly to the router's `/tmp/` or `/usr/bin/` directory and executed without any external dependencies.
