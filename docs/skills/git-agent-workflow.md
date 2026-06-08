---
name: git-agent-workflow
description: >-
  Enforces a strict, step-by-step git commit, comment, and push protocol for autonomous AI agents. 
  Ensures that every distinct file mutation is cryptographically traceable, documented, 
  and pushed to the remote immediately to protect rollback invariants and allow human oversight.
---

# Git Agent Workflow: Commit, Comment & Push

## Overview

This skill defines the mandatory, atomic operational loop for all agents executing modifications in the Blueshoes repository. To preserve the core doctrine of **"Rollback is Sacred"**, agents must never bundle multiple tasks into a single commit or push. Every single step of progress must be independently committed, commented, and pushed.

---

## The Step-by-Step Commit & Push Protocol

Whenever you perform a file modification, run a build, or verify a milestone, you **MUST** execute the following loop:

```
┌──────────────────────────────────────────┐
│          1. Perform Code Mutation        │
└────────────────────┬─────────────────────┘
                     │
                     ▼
┌──────────────────────────────────────────┐
│          2. Run Verification Tests       │
└────────────────────┬─────────────────────┘
                     │
                     ▼
┌──────────────────────────────────────────┐
│         3. Git Add Modified Files        │
└────────────────────┬─────────────────────┘
                     │
                     ▼
┌──────────────────────────────────────────┐
│        4. Commit with Detailed Comment   │
└────────────────────┬─────────────────────┘
                     │
                     ▼
┌──────────────────────────────────────────┐
│        5. Push to Remote Immediately     │
└──────────────────────────────────────────┘
```

### 1. Perform Code Mutation
Keep edits strictly confined to a single logical task (e.g., adding a helper, fixing a syntax warning, updating a single test).

### 2. Run Verification Tests
Before committing, always run:
```bash
make test
```
If tests fail, do **not** commit. Revert or correct the code first.

### 3. Git Add Modified Files
Stage only the files relevant to the completed step:
```bash
git add <path/to/modified/file>
```

### 4. Commit with Detailed Comment
Write a semantic commit message containing:
- **Type/Scope**: e.g., `feat(probes)`, `fix(watchdog)`, `docs(constitution)`
- **Intent**: A one-line summary of what the change does.
- **Detailed Trace (Body)**: 1-2 lines detailing the *evidence* or *rationale* (e.g., "Verified that cargo test passed with 18 tests, non-snake-case warning resolved").

Example:
```bash
git commit -m "fix(doctor): resolve FreeBSD non-snake-case compiler warning

- Renamed FreeBSD_readable to free_bsd_readable in src/probes/doctor.rs.
- Confirmed that make test runs clean without warnings."
```

### 5. Push to Remote Immediately
Never hold unpushed commits locally. Push to the active tracking remote branch immediately:
```bash
git push timelabs main
```

---

## Why Every Single Step Matters

1. **Granular Rollbacks**: If a subsequent step introduces a regression or violates the [Cloud Constitution](file:///Users/sa/bs/docs/CLOUD_CONSTITUTION.md), we can roll back exactly to the last committed clean step in under 5 seconds.
2. **Multi-Agent Coordination**: Pushing every step prevents conflicts (git merge races) between concurrent agents or human operators working on the same repository.
3. **Observability**: Humans and auditor agents can monitor the commit stream in real-time on GitHub to verify alignment.

---

## Common Mistakes to Avoid

* ❌ **Bundling multiple changes**: Never modify a probe, fix a test, and update documentation in one commit. Split them into three separate commits and three separate pushes.
* ❌ **Skipping local tests before pushing**: Pushing broken code violates the Alignment Gate. Always run tests first.
* ❌ **Vague commit messages**: Do not write `git commit -m "fix code"`. Explain *what* was changed and *why* it matches the spec.
