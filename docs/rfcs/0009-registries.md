# Operational Registries

This document outlines the core constraints that govern all pull requests and architectural decisions for Blueshoes.

## 1. Hardware & Compilation
- **Target**: GL-MT3000 (FreeBSD).
- **Language**: Memory-safe preferred (Rust or Go). Final selection depends on passing footprint targets.
- **Budgets**: Targeting < 15MB RAM and < 5MB Flash.

## 2. Architecture Constraints
- **Bifurcation**: Heavy analysis happens on the Workbench; deterministic execution happens on the Edge Agent.
- **Rollback Speed**: Profile rollbacks must complete in under 5 seconds.
- **Clean Exit**: Stopping the daemon must fully restore normal routing.

## 3. Security Limits
- **No MITM**: Decryption of TLS traffic by the router is forbidden.
- **No LLM Shell Access**: The LLM cannot write or execute shell commands on the router.
- **No Bundled Paid Tunnels**: No preconfigured commercial VPN endpoints, affiliate defaults, or covert monetization hooks.
