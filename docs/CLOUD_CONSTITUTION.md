# Cloud Constitution

> **STATUS**: ACTIVE — Binding on all agents, implementations, and infrastructure decisions.
>
> **AUTHORITY**: This document is a constitutional extension of
> [governance-doctrine.md](governance-doctrine.md) and
> [0001-runtime-doctrine.md](rfcs/0001-runtime-doctrine.md).
> It may only be amended by human authority.

---

## 0. Why This Document Exists

Blueshoes uses cloud services (GCP). Cloud infrastructure has a natural
gravitational pull toward **sovereignty capture**: what starts as innocent
replication quietly becomes coordination, then optimization, then
"just one sync dependency", then hidden authority transfer.

This document exists to make that drift **structurally impossible**
by declaring explicit constitutional boundaries on what cloud may
and may not become within BS architecture.

---

## 1. The Sovereignty Invariant

```
Edge is sovereign. Cloud is auxiliary.
```

**Local verified lineage outranks remote mutable authority. Always.**

No cloud service, database, API, or coordination layer may:
- become the canonical source of runtime truth,
- be required for safe continued operation,
- override local state without local validation,
- hold authority that the edge has not explicitly, revocably delegated.

An edge node that loses all cloud connectivity **MUST** remain:
- operationally safe,
- capable of rollback,
- capable of STOP,
- capable of independent polling,
- capable of self-diagnosis.

If any cloud dependency violates this, **the dependency is unconstitutional
and must be removed**, regardless of operational convenience.

---

## 2. Cloud Role Classification

Every cloud service used by BS falls into exactly one of these roles.
A service **may not hold multiple roles** without explicit constitutional review.

### 2.1 PERMITTED Roles

| Role | Semantics | Examples |
|------|-----------|----------|
| **Archive** | Write-once historical preservation. Cloud stores what edge has already committed. Never feeds back as authority. | BigQuery (analytics), Cloud Storage (backups) |
| **Mirror** | Async, eventually-consistent reflection of edge state. Noncanonical. Observable but not authoritative. | Firestore (dashboard sync) |
| **Gossip** | Advisory distribution of promoted artifacts across edges. Edges validate before accepting. | Pub/Sub (telemetry fan-out, threat advisories) |
| **Oracle** | Read-only observability. Cloud may observe; cloud may not command. | Cloud Logging, Cloud Monitoring |
| **Forge** | Stateless compute for delegated processing. Results are advisory until edge validates and commits locally. | Cloud Run (packet analysis) |
| **Vault** | Credential storage with explicit, revocable access grants. Edge pulls; cloud never pushes credentials into runtime. | Secret Manager |
| **Memory Fabric** | Cross-edge constitutional knowledge store. Stores **promoted semantic artifacts**, not operational state. See §3. | Spanner |

### 2.2 FORBIDDEN Roles

No cloud service may become:

| Forbidden Role | Why |
|----------------|-----|
| **Sovereign Clock** | Centralized temporal authority violates edge autonomy. Edge timestamps are authoritative locally. |
| **Canonical State** | Global "single source of truth" for operational config inverts edge-first doctrine. |
| **Mutation Authority** | Cloud may never issue commands that directly mutate edge runtime state. |
| **Safety Gate** | Cloud unavailability must never prevent rollback, STOP, or watchdog execution. |
| **Consensus Coordinator** | BS edges do not participate in cloud-mediated consensus for operational decisions. |

---

## 3. Spanner: Constitutional Memory Fabric

Spanner is permitted in BS architecture **exclusively** as a
**cross-edge constitutional memory fabric**.

This means Spanner stores **promoted semantic knowledge**, not operational sovereignty.

### 3.1 Spanner MAY Store

- Asynchronous promoted artifacts
- Global advisory threat intel
- Signed policy overlays
- Replicated observations (post-commit telemetry summaries)
- Globally shared threat intelligence (adversarial fingerprints, DPI signatures)
- Revocation broadcasts (compromised key announcements)
- Advisory ACL overlays (suggested block lists — edge validates before applying)
- Promoted doctrine deltas (constitutional amendments after human approval)
- Semantic invariants (cross-edge consistency checks)

### 3.2 Spanner MAY NOT

- Block edge runtime
- Override `0.log`
- Issue mutations
- Become canonical active router state
- Directly command edge mutations
- Become the source of runtime truth
- Bypass local lineage validation
- Invalidate local recoverability
- Act as the canonical operational clock
- Require connectivity for safe edge execution
- Store mutable operational config that edges depend on for function

### 3.3 The Critical Distinction

```
WRONG:  Spanner is the global state authority
              → edges read truth from cloud
              → cloud failure = edge failure
              → sovereignty captured

RIGHT:  Spanner is the cross-edge memory fabric
              → edges remain sovereign
              → local log remains authoritative
              → promoted artifacts replicate globally
              → cloud failure = reduced awareness, not reduced safety
```

### 3.4 Edge Interaction Protocol

When an edge node reads from Spanner:

1. Data is treated as **advisory** until locally validated
2. Edge applies its own lineage checks before accepting any artifact
3. Accepted artifacts are committed to **local storage first**
4. Edge never blocks on Spanner availability
5. Stale or unreachable Spanner = edge continues with last-known-good local state

---

## 4. Firestore: Noncanonical Mirror

Firestore is permitted as an **async, eventually-consistent mirror** for:
- Dashboard state reflection
- Real-time observability push (WebSocket listeners)
- Loose operational visibility

Firestore is **not** a coordination substrate. It reflects; it does not command.

---

## 5. The Drift Detection Invariants

These invariants must be checked during any architectural review
that involves cloud services:

### 5.1 The Connectivity Test
> *"If all cloud services go down simultaneously, does every edge node
> remain safe, rollback-capable, and operationally functional?"*
>
> If NO → **constitutional violation**.

### 5.2 The Authority Test
> *"Does any cloud service hold state that an edge node treats as
> authoritative without local re-validation?"*
>
> If YES → **constitutional violation**.

### 5.3 The Inversion Test
> *"Has the data flow direction silently changed from
> edge→cloud (reporting) to cloud→edge (commanding)?"*
>
> If YES → **constitutional violation**.

### 5.4 The Clock Test
> *"Does any cloud service's timestamp or ordering take precedence
> over the edge's local event sequence?"*
>
> If YES → **constitutional violation**.

### 5.5 The Removal Test
> *"Can this cloud service be entirely removed or replaced with a
> different provider without modifying edge runtime logic?"*
>
> If NO → **sovereignty leak** — cloud has become structurally coupled.

---

## 6. Enforcement

- All cloud integration PRs must pass the five drift detection invariants (§5)
  before merge.
- Opus 4.6 review (per [governance-doctrine.md §4](governance-doctrine.md))
  is **mandatory** for any change that:
  - adds a new cloud service,
  - changes the role classification of an existing service,
  - introduces cloud→edge data flow,
  - adds connectivity requirements to any execution path.
- Violations discovered post-merge must be treated as **security-class incidents**
  and reverted before further development proceeds.

---

## 7. Audit Trail

- 2026-06-08: Document created. Triggered by identification of constitutional
  tension between Spanner's globally-consistent semantics and BS's edge-authoritative
  doctrine. Spanner reclassified from "global SQL backend" to "constitutional
  memory fabric" with explicit boundaries.

---

## 8. The Equation

```
Cloud stores promoted constitutional knowledge.
Cloud does NOT store operational sovereignty.

Edge sovereignty + cloud memory = distributed cognition.
Cloud sovereignty + edge compliance = centralized control.

BS chooses the first. Always.
```

---

## 9. Ratification & Oath of Fealty to the Edge

We, the human creators and agent executors of the Blueshoes runtime, hereby ratify this Cloud Constitution. We pledge that the cloud shall remain an advisory memory, never an active sovereign, and that local rollback remains sacred.

```
                  ┌──────────────────────┐
                  │   HUMAN AUTHORITY    │ (Root of Trust)
                  └──────────┬───────────┘
                             │
                             ▼
                  ┌──────────────────────┐
                  │ BS EDGE SOVEREIGNTY  │ (Local Lineage)
                  └──────────┬───────────┘
                             │
            ┌────────────────┴────────────────┐
            ▼                                 ▼
┌──────────────────────┐           ┌──────────────────────┐
│  SPANNER (MEMORY)    │           │ FIRESTORE (MIRROR)   │
└──────────────────────┘           └────────────────└─────┘
 (Advisory Knowledge)                (Dashboard Reflect)
```

**Signed in compliance with the Sovereignty Invariant:**

*   **Human Operator** (Canonical Root Authority) — *Approved*
*   **Antigravity** (Constitutional Orchestrator) — *Attested*

