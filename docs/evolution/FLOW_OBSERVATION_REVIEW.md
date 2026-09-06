# Read-only Flow Observation review record

## Authorized work and pre-execution request

Continue only `evolution/clashmac-flow-observation-v1` and the paired
`omnia-playbook/evolution/network-flow-semantic-redteam-v1`. Implement a standalone
Rust observation crate, four concrete adapter interfaces, Windows query-only TCP
collection, independent fixture normalization and a read-only topology projection.
No executor dependency, helper, network mutation, merge, or audit/Kudu branch edit.
ClashMac remains the closed-source behavioral reference at
`6bd4eee77ac3face93d6ba38fdc505e15a4e376e`; no implementation or binary import.

Starting heads: Blueshoes `9ad772479eaa8e1f715933815d094db9e519f731`;
omnia-playbook `2be243aa0e04a2666256c7d0d566fbed5bd26eac`.
The user explicitly authorized material-step commits, pushes and paired PR updates.

## Independent advisory verdicts (before implementation)

Three isolated reviewers read the proposal and repository context without reading
one another's verdicts. The orchestrator aggregated them only after submission.

- Security: ACCEPT WITH IMPLEMENTATION CONDITIONS. Require bounded native table
  reads, independent process birth queries bracketing the authoritative snapshot,
  typed authority, nested injection rejection, dependency/native import evidence,
  scoped IPv6, unknown counters, monotonic freshness and redacted receipts.
- Governance: CONDITIONAL ACCEPT. User authorization covers these commits/pushes.
  Disclose nullable counter compatibility and schema hashes, retain NetBSD schema
  acceptance, distinguish fixture tests from native execution and preserve exact
  cross-repository evidence. No new permission is required within scope.
- Architecture: APPROVE for bounded design with constraints. Keep the crate
  separate from the edge executor, preserve scoped identity and unknown listener
  peers, and never turn collection errors into successful empty snapshots.
  This wire/projection slice does not claim substrate canonicalization receipts.

## Release conditions

Required: independent fixture cases, strict parser and graph tests, actual Windows
query smoke, dependency/API inspection, explicit unsupported-platform markers and
post-implementation independent review. Old V1 consumers reject new null counters;
the widened schema accepts all old valid documents. Fixture PASS and native PASS
are separate gates. These advisory verdicts do not establish runtime qualification.

Stop conditions: any mutation/executor/helper path; fabricated counters or process
identity; stale data promoted to fresh; display data replacing evidence; unbounded
native reads; private topology in published evidence; unsupported native PASS;
ClashMac implementation import; edits outside the authorized branches.

## Post-implementation independent review

The three isolated reviewers inspected the production implementation and tests.
Security found no blocking parser/native/graph issue, then independently reproduced
the five dependency/source/corpus/CLI/PE gates and native wire/graph validation.
Architecture and Governance found issues before release: a rounded native birth
maximum, missing integer-token profile cases, an ignored duplicate-key fixture,
ambiguous direction wording and inaccurate collection interval wording. All were
corrected and independently rechecked. Final verdicts: Security ACCEPT;
Governance ACCEPT; Architecture APPROVE for this bounded read-only scope.

Reviewed implementation: `32307237d4c5c305b6cbb2509503a8cea9d5758e`;
independent corpus: `30002c67533258691203391b4f0c30a3125d8e23`.
The production fingerprints in `runtime/flow-observation/reviewed-source-lock.json`
bind the inspected source set. See [verification evidence](FLOW_OBSERVATION_EVIDENCE.md).
These are independent advisory/code-review and test receipts, not a universal
runtime safety claim, mutation qualification or authority grant.
