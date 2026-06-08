# SYSTEM_PROMPT_RECOVERY_MAP.md

This map identifies all instruction and system-prompt files within the Blueshoes repository, their purpose, status, and priority layers.

## Instruction Layer Hierarchy

| Priority | File Path | Purpose | Status | Loaded By |
| :--- | :--- | :--- | :--- | :--- |
| **0 (Root)** | `.agent_instructions.md` | Root operational directives and authority definitions. | **ACTIVE (STALE RULE FOUND)** | AGY / Agent Platform |
| **0 (Root)** | `docs/UNIFIED_AGENT_EXECUTION_CONTRACT.md` | Binding execution contract and constitutional invariants. | **ACTIVE** | Governance Layer |
| **1 (Secondary)** | `.github/CLAUDE.md` | Environment context, conventions, and project principles. | **ACTIVE** | Claude/Agent IDEs |
| **1 (Secondary)** | `docs/governance-doctrine.md` | Governance and authority rules. | **ACTIVE** | Governance Layer |
| **2 (Protocol)** | `docs/TRIBUNAL_PROTOCOL.md` | Procedures and verdicts for the Tribunal advisory system. | **ACTIVE** | Tribunal Orchestrator |
| **2 (Adapter)** | `docs/rhea-tribunal-adapter.md` | Architecture for Passive Tribunal Gate (T0). | **ACTIVE** | Integration Layer |
| **3 (Context)** | `docs/CLOUD_CONSTITUTION.md` | Rules for cloud/local lineage boundaries. | **ACTIVE** | GCP Integration |
| **3 (Context)** | `SECURITY.md` | Security policy and vulnerability disclosure. | **ACTIVE** | Public/Audit |

## Conflicts and Stale Rules

| File | Conflict / Issue | Severity | Resolution |
| :--- | :--- | :--- | :--- |
| `.agent_instructions.md` | Contains `git add .` instruction which contradicts explicit-file staging discipline. | **CRITICAL** | **RESOLVED**: Patched to use explicit staging only. |
| `docs/UNIFIED_AGENT_EXECUTION_CONTRACT.md` | Mentions "No tribunal theatre unless explicitly requested" but `.agent_instructions.md` makes it mandatory. | **LOW** | Contextual: Advisory by default, mandatory for autonomous state-modifying work. |

## Verified Prompt Fragments

- **Sovereignty Receipt Requirement**: Rule XV in `.agent_instructions.md` (Must provide direct observable receipt).
- **Capability-Based Execution**: Rule 2 in `.agent_instructions.md` (No raw shell outside `src/executor`).
- **Forbidden Language**: Section III in `docs/UNIFIED_AGENT_EXECUTION_CONTRACT.md`.
- **Evidence-First Reporting**: Section IV in `docs/UNIFIED_AGENT_EXECUTION_CONTRACT.md`.
