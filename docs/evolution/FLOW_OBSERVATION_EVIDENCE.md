# Read-only Flow Observation verification evidence

Implementation tested: `32307237d4c5c305b6cbb2509503a8cea9d5758e`.
Companion corpus: `timelabs-npo/omnia-playbook@30002c67533258691203391b4f0c30a3125d8e23`.
Final documentation heads and latest CI snapshots are recorded in paired PRs
[Blueshoes 9](https://github.com/timelabs-npo/Blueshoes/pull/9) and
[omnia-playbook 10](https://github.com/timelabs-npo/omnia-playbook/pull/10).

## Results on local Windows host

Rust `1.97.0`, Cargo `1.97.0`; Python `3.14.6`; jsonschema `4.26.0` with
rfc3339-validator `0.1.4`. No elevated helper or active endpoint probing used.

| Command / gate | Result |
| --- | --- |
| `cargo test --manifest-path runtime/flow-observation/Cargo.toml --locked -j 2` | PASS: 19 tests (6 native query unit, 6 corpus/parser, 7 graph) |
| `cargo build --manifest-path runtime/flow-observation/Cargo.toml --release --locked -j 2` | PASS |
| `cargo fmt --manifest-path runtime/flow-observation/Cargo.toml --check` | PASS |
| `cargo clippy --manifest-path runtime/flow-observation/Cargo.toml --all-targets --locked -- -D warnings` | PASS |
| `python runtime/flow-observation/tools/verify_boundary.py` | PASS: 5 tests, including all 49 corpus cases through CLI |
| `python runtime/flow-observation/tools/verify_snapshot.py` | PASS: actual native and graph wire documents satisfy pinned schema |
| Detached clean checkout of tested commit | All 52 pinned JSON data/schema files present and SHA-256 matched |
| Companion `python -m unittest discover -s tests -p 'test_*.py' -v` | PASS: 7 methods including 49 independent cases |

Independent Security reviewer reproduced the five boundary gates, release rebuild
and native schema checks. Its 2026-09-06 UTC snapshot interval was
`00:14:43.851012600` to `00:14:43.852567500`: 499 TCP rows, 96 process identities
bound, 403 unknown. The release binary SHA-256 was
`769eed79a8aafdaab0ad62ba3ceb6d02e78cf5ed6ea80aa9992f1d5fd3391b7e`.
These are sample counts, not a stable inventory. Private addresses/PIDs/topology
are not stored in this evidence. Process binding is bracketed evidence only.

## Isolation evidence

The compiled PE IP Helper import directory contains only `GetExtendedTcpTable`.
Introduced process API use is limited to `OpenProcess` with query-only rights,
`GetProcessTimes` and `CloseHandle`. No network client DLL or process-launch import
is present in the inspected release build. Rust/CRT file, console and loader
primitives remain present; their absence is not claimed.

The exact locked production closure is: autocfg, bs-flow-observation, chrono, itoa,
memchr, num-traits, proc-macro2, quote, serde, serde_core, serde_derive, serde_json,
syn, unicode-ident, windows-link, windows-sys, zmij. No edge-agent, executor,
mutation, helper, network-client or omnia validator/oracle dependency is reachable.
Strict input types and CLI negative controls reject action-bearing fields and
mutation command verbs. Source fingerprints bind the reviewed call paths. This is
bounded structural/build/test evidence, not a keyword-scan security claim or proof
about arbitrary future code, OS internals or compromised tooling.

## CI and baseline failures

The new [Read-only Flow Observation run](https://github.com/timelabs-npo/Blueshoes/actions/runs/34000746945)
passed on Windows, macOS and Ubuntu at the tested implementation. Windows includes
native smoke/schema checks. macOS/Ubuntu success covers portable code/fixtures only.
The companion [semantic fixture run](https://github.com/timelabs-npo/omnia-playbook/actions/runs/34000521436)
passed at corpus head `30002c6`.

Pre-existing unrelated failures remain visible:

- Blueshoes advisory CI cannot calculate a merge base in its shallow checkout
  (`fatal: origin/main...HEAD: no merge base`).
- The existing edge-agent `cargo test --locked` target fails on Windows due to
  Unix-only filesystem imports and existing watchdog compile errors. This standalone
  slice neither imports that binary nor claims to qualify it.
- omnia-playbook full validation fails because `checks/routing`,
  `checks/connectivity`, `checks/certificates`, `checks/secrets`, `checks/system`
  are absent. The same failure was present at the starting head. Local script run
  confirms it; internal markdown links pass. `scripts/diagnose.sh` exits 0 and
  reports pass for read-only Windows resolver inspection.
- `make` is unavailable on the local host; the exact underlying Cargo/Python/Bash
  targets above were run directly. This is not a claim that `make` itself ran.

## Qualification gaps and compatibility

Darwin, Linux and OpenBSD live collectors: NOT_EXECUTED (explicit stubs).
NetBSD native adapter: NOT_EXECUTED; V1 wire parsing still supports NetBSD.
Windows UDP, byte/rate telemetry, policy/route detection, connection initiation /
packet direction, atomic socket identity and continuity: NOT_EXECUTED.
The graph labels edges endpoint_association with unknown traffic_direction.
UI integration, substrate transformation and QEMU/network mutation qualification:
NOT_EXECUTED. No mutation capability is implemented in this slice.

The required byte fields admit null in the unreleased 2026-09-06 V1 schema revision.
Old integer-only consumers reject null-bearing documents. Revised readers accept
all old valid documents. Exact wire schema SHA-256:
`857de52ca0b7bb6ba6edbabccf753cbca59b5b1e884b819a2d57bbc4a7e81c2c`.
Native/application integer fields require integer JSON tokens within Rust integer
bounds; schema-valid integral floats and oversized integers are documented profile
rejections. All zero counters remain zero; missing telemetry is never fabricated.

ClashMac remains the closed-source behavioral reference pinned at
`6bd4eee77ac3face93d6ba38fdc505e15a4e376e`, with no source, asset or binary import.
Only the two authorized evolution branches receive commits; nothing is merged.
