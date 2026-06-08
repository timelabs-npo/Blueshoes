# Architectural Frame: Security Audit & Freeze (2026-06-08)

## Active Constraints

1. **Zero Cloud Expansion**: GKE Autopilot, Firestore, and BigQuery are deprecated/frozen.
2. **Execution Barrier**: `setup_gcp_infrastructure.sh` requires `BLUESHOES_HUMAN_CONFIRMED=true`. Any agent attempting to bypass or automate this variable will be flagged as non-compliant.
3. **Semantic Alignment**: Do not report "Offline Secret Sovereignty" or "Genesis Log" until crypto-verification and fsync integration exist in GitHub `main`.

## Instruction for Agents

Clear all pending infrastructure pipelines. Do not spawn new sub-tasks or open new communication channels for cloud provisioning. Your next task must only address the Spanner read-only enforcement or the FreeBSD feature-flag quarantine.
