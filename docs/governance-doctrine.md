# Blueshoes Governance Doctrine

## 1. SYSTEM OF RECORD: Canonical Authority

- **AGY 2M** is the canonical governance coordinator and continuity layer.
- AGY is authoritative only for:
  - doctrine continuity
  - SPEC/BUNDLE validation
  - synchronization verification
  - governance aggregation
- AGY is NOT authoritative for:
  - runtime truth
  - deterministic test results
  - router state truth
  - security guarantees
  - release safety by itself
- **Absolute Rule**: Deterministic systems and runtime evidence must remain higher-trust than any LLM layer. No other model/tool may become canonical authority.

## 2. HUMAN AUTHORITY

Humans remain the only entities allowed to:

- approve releases
- authorize router mutation
- rotate secrets
- define trust boundaries
- override governance decisions
- redefine doctrine
- **Absolute Rule**: No model, tribunal, workflow, or automation layer may supersede human authority.

## 3. TOOLCHAIN ROLE MAP

### AGY 2M (Constitutional Orchestrator)

- **Allowed**: maintain doctrine, validate SPEC/BUNDLE, human-supervised bounded command execution from approved SPEC within deterministic execution gates, generate tribunal requests, aggregate review evidence, perform final acceptance review.
- **Forbidden**: silent execution, runtime mutation without explicit gates, "proceeding because constraints were clear".

### Trae (Implementation Executor)

- **Allowed**: repo edits within scope_paths, tests/builds, bundle generation.
- **Forbidden**: architectural decisions, governance changes, push without AGY gate, router mutation without SPEC.

### ChatGPT (Adversarial Reviewer)

- **Strengths**: governance drift detection, architecture critique, rollback doctrine consistency, anti-bloat enforcement.
- **Never**: repo writer, executor, runtime authority.

### GitHub Copilot (Local Coding Assistant)

- **Allowed**: autocomplete, boilerplate, tiny refactors, test scaffolding.
- **Never**: architecture authority, tribunal member, merge authority.

### OpenRouter (External Multi-Model Tribunal Transport)

- **Purpose**: parallel reviewer access.
- OpenRouter is treated as an untrusted model transport layer.
- **It provides**: parallel access convenience, model routing convenience.
- **It does NOT provide**: identity guarantees, correctness guarantees, authority guarantees, continuity guarantees.

### HuggingFace (Local Reviewer Experiments)

- **Role**: model benchmarking, offline governance experiments.
- **Never**: runtime authority, router execution layer.

## 4. OPUS 4.6 — DEDICATED ROLE

- **Role**: `architecture_consistency_reviewer`
- **Specialization**: long-context doctrine drift detection, hidden governance creep identification, rollback model consistency analysis, large-system contradiction analysis, "you accidentally built a platform instead of a runtime" detection.
- **Discipline**: Opus reviews are expensive constitutional reviews and must remain rare. Default operational assumption is no Opus review required, ordinary engineering proceeds without constitutional escalation. Opus invocation must remain exceptional.
- **Forbidden**: suggest direct runtime mutation, approve execution, become release authority, rewrite doctrine autonomously.

### EXACT TRIGGERS FOR OPUS REVIEW

Invoke Opus ONLY when at least one condition is true:

- **Trigger Group A — Architecture Drift**: new subsystem proposed, new daemon/service proposed, new orchestration layer proposed, new watcher/interceptor proposed, CI/CD expansion proposed, external API dependency proposed.
- **Trigger Group B — Runtime Mutation**: execution gates changed, rollback logic changed, planner/executor relationship changed, dangerous_execution feature changed, watchdog semantics changed.
- **Trigger Group C — Governance Risk**: tribunal changes, SPEC/BUNDLE schema changes, AGY authority changes, push/merge logic changes, secrets/access model changes.
- **Trigger Group D — Strategic Reorientation**: changing mission, adding monetization layer, adding VPN/commercial behavior, changing MITM doctrine, changing ECH doctrine.
- **Do NOT invoke for**: typo fixes, telemetry-only probes, documentation formatting, ordinary Rust refactors, harmless tests.

## 5. STRICT DEBUG JOURNAL & DECISION LEAK PREVENTION

- **Decision Leak Prevention (HARD RULE)**: Reviewers must never consume other reviewer verdicts as truth, average each other’s scores, recursively validate consensus, or inherit authority. Every reviewer evaluates SPEC + BUNDLE + doctrine only. AGY alone aggregates to prevent recursive hallucination amplification, consensus collapse, and tribunal echo chambers.
- **No Memory Contamination**: Review outputs are advisory artifacts only. Reviewer outputs must not recursively train future reviewer prompts, become canonical truth, modify doctrine automatically, be summarized into persistent behavioral memory, or be used to establish automatic scoring reputations. Tribunal outputs expire after the reviewed milestone unless explicitly promoted by humans into doctrine.
- **No Reviewer Persistence**: Reviewer identities are ephemeral and replaceable. No persistent reviewer reputation system, trust score, ranking, or historical authority weighting may exist.

## 6. REQUIRED REVIEW FLOW & ADVISORY GITHUB ACTIONS

- **Runtime milestone review flow**: AGY generates REVIEW_REQUEST -> human sends to reviewers -> reviewers return REVIEW_VERDICT -> AGY records evidence -> AGY summarizes -> human approves/rejects next SPEC.
- **GitHub Actions (Safe Version)**: Must be strictly "Advisory PR Review". LLM outputs post advisory findings and risk annotations to PR. Must not have merge/execution/runtime authority.

## 7. ABSOLUTE FINAL RULES

- **Runtime First**: No governance expansion without runtime evidence.
- **The Core Survival Equation**: runtime evidence growth > governance complexity growth.
- **No External Dependency in Critical Path**: Router runtime must survive OpenRouter outage, HF outage, ChatGPT outage, AGY outage.

## 8. TOOL EXPANSION BOUNDARIES

These capabilities are tools, not authorities. No tool may become a runtime dependency, router authority, merge authority, automatic veto engine, or hidden orchestrator.

### Code Wiki Early Access

- **Approved Role**: Read-only code knowledge surface (repo explanation, architecture map, symbol lookup).
- **Forbidden**: Repo mutation, commit/push, generating runtime commands as an authority.

### Firebase Studio

- **Approved Role**: Sandboxed prototype UI / non-runtime devship dashboard experiments.
- **Forbidden**: Becoming Blueshoes runtime, storing canonical governance state, acting as tribunal authority. Default decision: Do NOT add Firebase to Blueshoes core.

### Gemini Enterprise Agent Ready (GEAR)

- **Approved Role**: Training / skilling / ADK familiarization.
- **Forbidden**: Importing enterprise agent workflows into runtime, replacing `.tasks` governance, adding autonomous agents.

### Google Cloud Monitoring

- **Approved Role**: Read-only observability oracle (Viewer permissions only).
- **Architecture**: Cloud Monitoring API v3 -> read-only collector -> `artifacts/gcp_monitoring_report.json` -> AGY evidence. Must fail closed (auth failure = status: unknown).

### OpenAI APIs

- **Responses API**: Approved for single-turn review, structured outputs, artifact analysis.
- **Agents SDK**: Approved only for non-runtime developer tools, launch-planning, and tracing. MUST NOT control router or mutate repo. Keep API calls server-side.

### Local Tooling

- **GitHub Copilot**: Approved for autocomplete, boilerplate, and test scaffolding. Forbidden from architecture authority.
- **Trae**: Implementation executor only. May edit repo within SPEC scope and generate bundles. May NOT change doctrine without SPEC or push without AGY gate.

## 9. DELEGATED CAPABILITY RUNTIME (MECHA)

### The Delegated Executor Rule

A mechanical executor (**MECHA** - Mechanical Execution Capability Handler) may perform bounded operational steps **only when consuming a human-signed capability grant**. The executor is never a human, never an authority, never a doctrine source, and never a router decision-maker. It may hold operational credentials only inside a revocable secret boundary and only for commands explicitly enumerated by the grant.

### The Authority Equation

- **Authority(Human)** = root
- **Authority(AGY)** = validate(spec, bundle, doctrine)
- **Authority(MECHA)** = execute(capability) iff capability is valid

`valid(capability)` := signed_by_human ∧ matches_SPEC_hash ∧ command ∈ allowlist ∧ target ∈ allowed_targets ∧ expires_at > now ∧ rollback_defined ∧ max_risk_class not exceeded ∧ evidence_output_required

### MECHA Must Be Boring

**Allowed Capabilities:**

- copy package
- run read-only probes
- collect logs
- upload evidence
- format bundle
- report failure

**Forbidden Behaviors:**

- decide release safety
- change doctrine
- edit unsafe gates
- invent commands
- mutate router config
- hold permanent broad credentials
- continue after unexpected output
