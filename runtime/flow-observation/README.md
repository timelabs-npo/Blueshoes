# Read-only Flow Observation

Standalone Rust crate. It has no dependency on `bs-edge-agent`, its executor,
mutation modules, helpers, network clients or subprocess control. `FlowAdapter`
offers only `collect()` and `fixture(input, evaluation)`. The wire parser consumes
`FlowObservationV1` data; fixture adapters consume `NativeFlowSampleV1` DTOs.

From the repository root:

```text
cargo test --manifest-path runtime/flow-observation/Cargo.toml --locked
cargo run --manifest-path runtime/flow-observation/Cargo.toml --locked -- smoke
cargo run --manifest-path runtime/flow-observation/Cargo.toml --locked -- snapshot
cargo run --manifest-path runtime/flow-observation/Cargo.toml --locked -- fixture runtime/flow-observation/fixtures/omnia/equivalent-win32.json 2026-09-06T00:00:10Z
```

`smoke` prints aggregate query evidence without addresses, PIDs, paths or topology.
`snapshot` explicitly prints local observations and their provenance to stdout;
its output contains private topology and is not published as CI evidence. Neither
command installs helpers, probes endpoints, starts a server or writes network state.

`fixture FILE RFC3339_NOW` and `observation FILE RFC3339_NOW` project bounded input
without native collection. The former consumes a native DTO fixture; the latter
consumes one V1 observation. Both report native execution as NOT_EXECUTED.

## Flow Graph projection

`graph::project(evidence, evaluation)` is pure and stateless. It emits endpoint and
process nodes plus directed flow edges that retain complete supporting observations,
origin, declared freshness, prior freshness and freshness at projection time.
Every graph and observation has typed `observation_only` authority.
Display location never participates in IDs, endpoint evidence or route inference.
Unknown processes have no association node. Imported process references are opaque
assertions scoped to their observation, so a supplied `pid:42` cannot merge owners.
Native/fixture references share a process node only within their birth/collector scope.
Endpoint nodes are scoped to supporting observations; later UI grouping must preserve
that evidence boundary. Duplicate identical observations are idempotent; conflicting
IDs reject. BTree ordering gives deterministic output independent of input order.

Reprojection evaluates freshness again and never upgrades stale/unknown evidence.
This projection is suitable for a connections/topology UI, with no execution API.

## Platform interfaces

| Adapter | Process birth token contract | Live collector |
| --- | --- | --- |
| DarwinAdapter | darwin_start_us, kernel start microseconds | NOT_EXECUTED; fixture adapter |
| Win32Adapter | win32_creation_100ns, exact FILETIME | TCP IPv4/IPv6 query |
| LinuxAdapter | linux_start_ticks, boot-scoped start ticks | NOT_EXECUTED; fixture adapter |
| OpenBsdAdapter | openbsd_start_us, kernel start microseconds | NOT_EXECUTED; fixture adapter |

These are DTO contracts, not claims that binary kernel table formats are identical.
NetBSD remains accepted by the V1 wire parser; no NetBSD native adapter is provided.
Every fixture batch marks its native gate NOT_EXECUTED, regardless of host OS.
An unsupported live collector yields NOT_EXECUTED with explicit gaps; a Windows
query error returns an error, never a successful empty snapshot.

## Windows query scope

The only host calls introduced are `GetExtendedTcpTable`,
`OpenProcess(PROCESS_QUERY_LIMITED_INFORMATION)`, `GetProcessTimes` and `CloseHandle`.
TCP owner-PID tables support both IPv4 and IPv6. Tables are bounded at 16 MiB,
three resize attempts and 10,000 combined rows. Checked buffer extents precede
unaligned struct copies; process handles close through RAII.

A discovery sample finds PIDs. Independent process creation queries then bracket
the actual socket sample. Missing pre/post birth, PID zero or changed birth produces
no process binding. Each post-sample read opens the current PID anew. Flow IDs include
collection scope, snapshot and row ID; they do not claim socket continuity between
samples. Process references include platform, collection scope, PID and exact birth.
This is bracketed observation evidence, not atomic socket lifetime proof.

Listener remote addresses/ports have no peer meaning and normalize to unknown.
Scoped IPv6 addresses retain their scope suffix. Wildcard addresses remain partial.
UDP, counters/rates, policy, route and atomic socket identity are NOT_EXECUTED.
Unavailable bytes/rates are null; measured fixture zero values stay zero.

Primary API contracts: [TCP tables](https://learn.microsoft.com/en-us/windows/win32/api/iphlpapi/nf-iphlpapi-getextendedtcptable),
[process times](https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-getprocesstimes),
[IPv4 rows](https://learn.microsoft.com/en-us/windows/win32/api/tcpmib/ns-tcpmib-mib_tcprow_owner_pid),
[IPv6 rows](https://learn.microsoft.com/en-us/windows/win32/api/tcpmib/ns-tcpmib-mib_tcp6row_owner_pid).

## Wire compatibility, freshness and trust

The pinned, unreleased 2026-09-06 V1 revision permits required byte counters to be
null. Its SHA-256 is `857de52ca0b7bb6ba6edbabccf753cbca59b5b1e884b819a2d57bbc4a7e81c2c`.
Old integer-only consumers reject new null-bearing documents; revised readers
accept old valid documents. Unsigned integers above u64 are outside this crate's
application profile, though JSON Schema permits arbitrary-sized nonnegative integers.
All required properties, enum values, nested unknown properties and duplicate keys
are checked by direct strict deserialization. Ingestion is capped at 8 MiB.

Freshness uses an explicit evaluation clock and age bound (30 seconds for native
collection). Old/future observations downgrade; stale/unknown never upgrade to
fresh. Native timestamps cover collection before/after identity checks. Wire imports
retain their supplied provenance as an `imported_assertion`; a forged native source
string cannot obtain `native_local_query` trust. Fixtures always have fixture origin.

The wire/projection objects are not substrate entities or canonical substrate
receipts. A future substrate bridge requires an explicit transformation and review.
ClashMac remains solely the closed-source behavioral reference pinned at
`6bd4eee77ac3face93d6ba38fdc505e15a4e376e`; no code, assets or binaries were imported.
