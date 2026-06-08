# Blueshoes Tests

This directory contains integration and end-to-end tests for the Blueshoes architecture.

## Structure
- `unit/`: Unit tests for the Rust `bs-edge-agent`.
- `integration/`: Tests verifying the atomic rollback loop inside an FreeBSD QEMU VM.
- `workbench/`: Tests for the external LLM telemetry ingestion and profile recommendation system.
