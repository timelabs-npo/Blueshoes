# Blueshoes Tribunal Protocol

**Version:** 1.0  
**Status:** ACTIVE  
**Scope:** Bounded Constitutional Review Protocol  

---

## 1. Verdict Semantics

Every review performed by the Tribunal must resolve to exactly one of the following verdicts:

*   **`APPROVE`**: 
    *   No constitutional invariant violations detected.
    *   Risk class is acceptable.
    *   Change is bounded, explicit, and recoverable.
*   **`REVISE`**: 
    *   The proposed architecture is conceptually acceptable.
    *   Implementation details, risk mitigation, or provided evidence are insufficient to justify approval.
    *   Requires specific fixes before re-evaluation.
*   **`FLAG`**: 
    *   Potential invariant pressure or structural hazard detected.
    *   Review pauses and escalates to mandatory human review before further execution.
*   **`REJECT`**: 
    *   Direct violation of a constitutional invariant.
    *   The proposed change must not proceed.

---

## 2. Evidence Hierarchy

Decisions and assertions made by agents must be backed by evidence. The following classification defines the weight of evidence (from weakest to strongest):

*   **`E0` — Assertion Only**: Declarations made in prompts or comments without execution data.
*   **`E1` — Compile Success**: The codebase builds successfully without compiler-locking or fatal warnings.
*   **`E2` — Unit Tests**: Code compiles and passes all unit and regression test suites.
*   **`E3` — Runtime Local Test**: Statically checked execution in a local sandbox or simulator environment.
*   **`E4` — Adversarial Test**: Intentional failure injection (e.g., watchdog trigger, network drop) successfully validates the fallback state.
*   **`E5` — Reproducible Independent Verification**: Multiple independent processes or actors reproduce the exact same execution logs or hashes.
*   **`E6` — Long-Duration Operational Stability**: Telemetry confirms zero-drift execution over an extended monitoring period on hardware.

*Rule:* Lower-class evidence (e.g., `E1`) cannot refute or override the requirements of a higher-class test plan (e.g., `E4`).

---

## 3. Constitutional Violations

Violations are classified by severity to determine escalation paths:

### 3.1 CRITICAL
*   Irreversible local state mutations.
*   Introduction of cloud runtime dependencies in the critical path.
*   Hidden persistence mechanisms or untracked state.
*   Unbounded command or script execution.
*   Unsigned or unvalidated authority transfer.
*   Bypassing the rollback loop, watchdog daemon, or `STOP` signals.
*   Exposure of private credentials or secrets.

### 3.2 HIGH
*   Unverifiable event lineage or missing provenance hashes.
*   Opaque or unmonitored agent orchestration.
*   Unlogged network calls by local runtimes.
*   Mutable authority semantics that shift bounds dynamically.

### 3.3 MEDIUM
*   Semantic ambiguity in task specifications.
*   Weak evidence presented for safety assertions.
*   Excessive abstraction layers that obscure execution paths.

---

## 4. Conflict Resolution (Constitutional Priority Graph)

When design goals conflict, the following priority tree must be applied:

```
            Security ──> (outranks) ──> Convenience
         Recoverability ──> (outranks) ──> Performance
      Local Survivability ──> (outranks) ──> Cloud Coordination
       Explicit Semantics ──> (outranks) ──> Implicit Optimization
        Human Override ──> (outranks) ──> Autonomous Convergence
```

---

## 5. Human Override Semantics

A human operator may override a Tribunal decision or an invariant alarm under the following strict conditions:

1.  **Explicit Log Entry**: The override must be recorded in the local event journal.
2.  **Invariant Acknowledgment**: The specific invariant being pressured or bypassed must be declared.
3.  **Active Rollback Path**: A verifiable, local recovery mechanism must remain active.
4.  **Recorded Rationale**: The emergency context or engineering rationale must be written to the local log.

Without these four criteria, the override is invalid and must be treated as a **CRITICAL** violation.

---

## 6. The Role of the Tribunal

*   The Tribunal **does not** determine absolute truth.
*   The Tribunal exists solely to:
    *   Surface architectural and invariant pressure.
    *   Expose hidden contradictions in code and design.
    *   Evaluate bounded operational risk.
    *   Verify constitutional alignment before deployment.
    *   Identify unverifiable or assertion-only claims.
*   Final authority remains with the human operator and deterministic local runtime invariants.

---

## 7. Defining "Good" vs "Bad"

### 7.1 GOOD
*   **Recoverable**: Always capable of returning to a known safe state.
*   **Inspectable**: State transitions and execution paths are readable and auditable.
*   **Bounded**: Explicit limits are set on execution time, memory footprint, and network access.
*   **Explicit**: No implicit side effects or hidden assumptions.
*   **Local-Survivable**: Continues safe execution when disconnected from all cloud resources.
*   **Minimally Sovereign**: Authority is delegated only for specified sub-tasks and is revocable.
*   **Semantically Stable**: Concepts and data schemas remain consistent across versions.
*   **Adversarially Testable**: System limits can be verified by injecting errors.

### 7.2 BAD
*   **Opaque**: State or execution logic is hidden behind unchecked wrappers.
*   **Irreversible**: State changes that cannot be undone in under 5 seconds.
*   **Silently Stateful**: Hidden or untracked local persistence.
*   **Cloud-Authoritative**: Edge behavior is dictated dynamically by remote endpoints.
*   **Unverifiable**: Actions that produce no cryptographically verifiable receipts.
*   **Self-Modifying**: Code or runtime paths that mutate without a traceable lineage.
*   **Persuasive without Evidence**: System claims backed only by natural language assertions (`E0`).

---

## 8. Anti-Consensus Invariant

**Consensus does not increase truth automatically.**

Multi-agent systems quickly generate convergence loops that can stabilize hallucinations or validate structural errors.
*   **Agreement is weak evidence (`E0`).**
*   **Independent reproducibility and test execution are strong evidence (`E2`–`E5`).**

---

## 9. Epistemic Hierarchy

When resolving conflicting claims between actors:
1.  **Deterministic Test Verification (`E3`–`E4`)** outranks **Agent Consensus**.
2.  **Local Provenance Lineage** outranks **Remote Cloud Configuration**.
3.  **Human Command** outranks **Agent Recommendation**.
4.  **Static Constitutional Rules** outrank **Task Specifications**.
