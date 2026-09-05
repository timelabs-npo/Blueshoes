# Presentation source notes

Inspection date: 2026-09-05. Baseline inspected for repository claims: `timelabs-npo/Blueshoes@9ad954c31d72e4f8a3f49171f799cae140e6b2f1` (`main` at the time of inspection).

This document exists to stop the presentation layer from silently upgrading an ambition into a result.

## Evidence map

- [Previous README](https://github.com/timelabs-npo/Blueshoes/blob/9ad954c31d72e4f8a3f49171f799cae140e6b2f1/README.md): adaptive edge networking, rollback doctrine, no MITM, ECH preservation, no bundled commercial VPN endpoints, and a self-described B0 status. Status wording in that file is not treated here as fresh execution evidence.
- [Cargo manifest](https://github.com/timelabs-npo/Blueshoes/blob/9ad954c31d72e4f8a3f49171f799cae140e6b2f1/runtime/bs-edge-agent/Cargo.toml): `bs-edge-agent` version `0.1.0`, empty default feature set, opt-in `dangerous_execution`, and watchdog binary declaration.
- [Main entry point](https://github.com/timelabs-npo/Blueshoes/blob/9ad954c31d72e4f8a3f49171f799cae140e6b2f1/runtime/bs-edge-agent/src/main.rs): diagnostics, profile listing, JSON output, journal writes, planner, apply/confirm dispatch, semantic/provenance surfaces, and feature-dependent canary executor selection. The inspected `apply-confirmed` branch selects `DryRunExecutor` but also invokes snapshot/watchdog-related code; do not equate dry-run executor selection with proof of zero side effects.
- [Phase 1 scope](https://github.com/timelabs-npo/Blueshoes/blob/9ad954c31d72e4f8a3f49171f799cae140e6b2f1/docs/rfcs/0012-phase1-scope.md): GL.iNet MT-3000 / MediaTek MT7981B, FreeBSD-oriented daemon target, bounded transaction loop, and deferred global mesh / automated AI loop. This is a specification, not a board qualification report.
- [Module inventory](https://github.com/timelabs-npo/Blueshoes/blob/9ad954c31d72e4f8a3f49171f799cae140e6b2f1/MODULE_INVENTORY.md): module roles and claimed test coverage. Coverage labels were not independently re-run for this presentation task.
- [.agent_instructions.md](https://github.com/timelabs-npo/Blueshoes/blob/9ad954c31d72e4f8a3f49171f799cae140e6b2f1/.agent_instructions.md): capability-based execution doctrine, adversarial-network assumptions, Rhea/Rheknel-inspired invariant language, tribunal requirement, and receipt requirement.

## Target architecture introduced by the presentation

The README intentionally presents the user-specified product direction:

- OpenBSD-first constitutional core;
- whole-OS RAM-only target operating model;
- AI-driven strategy synthesis without AI sovereignty;
- self-healing through bounded validation and rollback;
- self-evolution through locally proposed strategies that cannot self-promote;
- Rheknel as the intended local truth/promotion gate;
- OpenWrt reduced to a compatibility/bootstrap substrate during transition rather than the long-term constitutional authority;
- VPNs treated as optional routing capabilities rather than required architecture.

These are **target-architecture statements** unless and until direct execution receipts establish them. The inspected source still contains FreeBSD-oriented implementation paths. The presentation does not rewrite that fact.

## What was not established

No runtime compilation, QEMU execution, physical-router bring-up, OpenBSD boot, RAM-only root image, packet capture, pf integration, model-in-the-loop execution, independent security audit, watchdog survivability result, censorship-circumvention result, production self-healing result, production self-evolution result, or in-kernel Rheknel enforcement was established by this documentation task.

“Hide your traffic from yourself” is expressed as a **data-minimization / amnesia design goal**, not as a claim of perfect anonymity. “The internet is free. Again!” is a manifesto about open access and operator control, not a promise of free ISP service or guaranteed circumvention.

## Presentation assets

`assets/hero.svg` and `assets/route-story.svg` are original vector illustrations with CSS animation, accessible text alternatives, and reduced-motion rules. Their routes are illustrations, not measurements.

`playground.html` is a standalone offline visualization using inline HTML, CSS, SVG, and JavaScript only. It has no API, telemetry, remote fonts, package dependencies, device access, model call, or network probe. All nodes, link costs, failures, policies, and receipts are synthetic. Its weighted shortest-path solver is **not** the Blueshoes runtime algorithm. The “toy truth gate” is only a deterministic stand-in for the target authority shape; it is not Rheknel. An optional tunnel edge illustrates policy, not a functioning VPN or transport.

GitHub strips scripts from rendered Markdown. The HTML companion must be downloaded and opened locally, or separately published by a maintainer. No GitHub Pages deployment is implied by the README.

## Presentation verification

Chromium was exercised through Playwright using the self-contained page content. Thirty bounded checks passed: initial and alternate routes, over-policy rejection, tunnel opt-in, full isolation, reset, keyboard toggles and focus retention, pause/resume, JSON export and labeling, desktop/mobile geometry, overflow and target sizes, reduced-motion behavior, no automatic requests, and no browser JavaScript errors. JavaScript syntax was also checked with Node.

Desktop (1440 px) and mobile (390 px) full-page screenshots, the hero (1280 × 660), and route illustration (1200 × 360) were visually inspected. This is **presentation verification only**. It is not a runtime test suite, router qualification, censorship test, or independent tribunal review.

## Review scope

The active documentation branch already contains a tribunal request requiring visible separation between target architecture and current evidence. Independent reviewer verdicts were not generated in this task. The intended handoff remains human-review-only; no automatic merge or release is authorized.
