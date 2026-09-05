# Presentation source notes

Inspection date: 2026-09-05. Baseline inspected for repository claims: `timelabs-npo/Blueshoes@9ad954c31d72e4f8a3f49171f799cae140e6b2f1` (`main` at the time of inspection).

This document exists to stop the presentation layer from silently upgrading an ambition into a result.

## Repository evidence map

- [Previous README](https://github.com/timelabs-npo/Blueshoes/blob/9ad954c31d72e4f8a3f49171f799cae140e6b2f1/README.md): adaptive edge networking, rollback doctrine, no MITM, ECH preservation, no bundled commercial VPN endpoints, and a self-described B0 status. Status wording in that file is not treated here as fresh execution evidence.
- [Cargo manifest](https://github.com/timelabs-npo/Blueshoes/blob/9ad954c31d72e4f8a3f49171f799cae140e6b2f1/runtime/bs-edge-agent/Cargo.toml): `bs-edge-agent` version `0.1.0`, empty default feature set, opt-in `dangerous_execution`, and watchdog binary declaration.
- [Main entry point](https://github.com/timelabs-npo/Blueshoes/blob/9ad954c31d72e4f8a3f49171f799cae140e6b2f1/runtime/bs-edge-agent/src/main.rs): diagnostics, profile listing, JSON output, journal writes, planner, apply/confirm dispatch, semantic/provenance surfaces, and feature-dependent canary executor selection. The inspected `apply-confirmed` branch selects `DryRunExecutor` but also invokes snapshot/watchdog-related code; do not equate dry-run executor selection with proof of zero side effects.
- [Phase 1 scope](https://github.com/timelabs-npo/Blueshoes/blob/9ad954c31d72e4f8a3f49171f799cae140e6b2f1/docs/rfcs/0012-phase1-scope.md): GL.iNet MT-3000 / MediaTek MT7981B, FreeBSD-oriented daemon target, bounded transaction loop, and deferred global mesh / automated AI loop. This is a specification, not a board qualification report.
- [Module inventory](https://github.com/timelabs-npo/Blueshoes/blob/9ad954c31d72e4f8a3f49171f799cae140e6b2f1/MODULE_INVENTORY.md): module roles and claimed test coverage. Coverage labels were not independently re-run for this presentation task.
- [.agent_instructions.md](https://github.com/timelabs-npo/Blueshoes/blob/9ad954c31d72e4f8a3f49171f799cae140e6b2f1/.agent_instructions.md): capability-based execution doctrine, adversarial-network assumptions, Rhea/Rheknel-inspired invariant language, tribunal requirement, and receipt requirement.

## External standards / technology references

These references support the *existence and semantics of candidate capabilities*, not their integration into Blueshoes.

- [NIST FIPS 203 — ML-KEM](https://csrc.nist.gov/pubs/fips/203/final): standardized Module-Lattice-Based Key-Encapsulation Mechanism.
- [RFC 10024 — PQ/T hybrid key agreement for TLS 1.3](https://www.rfc-editor.org/rfc/rfc10024.html): defines X25519MLKEM768, SecP256r1MLKEM768, and SecP384r1MLKEM1024 hybrid TLS 1.3 groups.
- [RFC 10042 — PQ/T ML-KEM key exchange for SSH](https://www.rfc-editor.org/rfc/rfc10042.html): defines ML-KEM-based hybrid SSH key exchange methods.
- [SCION documentation](https://docs.scion.org/en/latest/control-plane.html): SCION endpoints obtain and combine authenticated path segments; SCION operation depends on SCION control-plane infrastructure / gateways and must not be presented as unilateral path control over arbitrary legacy BGP networks.
- [RFC 9498 — GNU Name System](https://www.rfc-editor.org/rfc/rfc9498.html): informational specification for decentralized, privacy-enhancing and censorship-resistant name resolution; it is not an IETF Standards Track document and does not imply that every deployment is unblockable.
- [Yggdrasil](https://yggdrasil-network.github.io/): experimental end-to-end encrypted IPv6 routing overlay / mesh technology. It requires peer connectivity over some underlying medium; it cannot manufacture a physical link that does not exist.

## Target architecture introduced by the presentation

The README intentionally presents the user-specified product direction:

- post-OpenWrt firmware architecture rather than an application layered over stock router firmware;
- post-VPN architecture in which tunnels are optional capabilities rather than the product and trust anchor;
- post-DPI architecture based on protocol/path/name/medium agility rather than a single static anti-signature trick;
- post-quantum crypto agility with PQ/traditional hybrid key establishment as an architectural requirement;
- OpenBSD-first security/network semantics;
- whole-OS RAM-only target operating model;
- AI-driven strategy synthesis without AI sovereignty;
- self-healing through bounded validation and rollback;
- self-evolution through locally proposed strategies that cannot self-promote;
- Rheknel as the intended deterministic local truth/promotion gate;
- asynchronous topological mutation across path, name, medium and cryptographic epoch;
- `omnia-playbook` as a versioned reference/evidence corpus, not an oracle or packet-for-packet impersonation database;
- independent topology, timing/spectral and entropy/distribution observers as candidate Tribunal roles.

These are **target-architecture statements** unless and until direct execution receipts establish them. The inspected source still contains FreeBSD-oriented implementation paths. The presentation does not rewrite that fact.

## Important terminology guardrails

- Stock OpenBSD is not a microkernel. If `mbsd` eventually implements a microkernel-style trusted computing base, hardware enclave, or other split architecture around OpenBSD-derived components, that architecture needs its own implementation description and evidence.
- “DPI detected” must not be inferred from a timing anomaly alone. Congestion, radio interference, overloaded middleboxes and route changes can produce similar signals.
- “Rheknel reacts in O(1) / sub-millisecond time” is not treated as an established performance property. A specific algorithm, input bound, target CPU and benchmark receipt are required.
- “GNS cannot be blocked”, “SCION can always route around a censor”, “mesh works after total physical isolation”, and “PQ protected” without negotiated-session evidence are intentionally rejected as unconditional claims.
- “Hide your traffic from yourself” is expressed as a **data-minimization / amnesia design goal**, not as perfect anonymity.

## What was not established

No runtime compilation, QEMU execution, physical-router bring-up, OpenBSD boot, `mbsd` boot, RAM-only root image, packet capture, pf integration, model-in-the-loop execution, independent security audit, watchdog survivability result, censorship-circumvention result, production self-healing result, production self-evolution result, SCION/GNS/Yggdrasil integration, post-quantum negotiation, target-hardware ML-KEM benchmark, or in-kernel Rheknel enforcement was established by this documentation task.

“The internet is free. Again!” is a manifesto about open access and operator control, not a promise of free ISP service or guaranteed circumvention.

## Presentation assets

`assets/hero.svg`, `assets/post-era.svg`, and `assets/route-story.svg` are original vector illustrations with CSS animation, accessible text alternatives, and reduced-motion rules. Their routes are illustrations, not measurements.

`playground.html` is a standalone offline visualization using local HTML, CSS, SVG, and JavaScript only. It has no API, telemetry, remote fonts, package dependencies, device access, model call, or network probe. All nodes, link costs, failures, policies, and receipts are synthetic. Its weighted shortest-path solver is **not** the Blueshoes runtime algorithm. The “toy truth gate” is only a deterministic stand-in for the target authority shape; it is not Rheknel. An optional tunnel edge illustrates policy, not a functioning VPN or transport.

GitHub strips scripts from rendered Markdown. The HTML companion must be downloaded and opened locally, or separately published by a maintainer. No GitHub Pages deployment is implied by the README.

## Presentation verification

A prior local Chromium/Playwright presentation QA pass exercised the recovered showcase at desktop and mobile sizes and covered route alternatives, rejection states, tunnel opt-in, isolation, reset, keyboard interaction, motion preferences, JSON export, geometry/overflow and browser errors. That evidence is useful only for the presentation implementation and does not prove current runtime, router, censorship, security, rollback, or cryptographic behavior.

Because the README and hero were subsequently rewritten for the post-OpenWrt / post-VPN / post-DPI / post-quantum framing, that older screenshot comparison should not be presented as pixel-level verification of the latest branch without a fresh render.

## Confidential source material

A private narrative supplied during design work used the title “Asynchronous Topological Mutation” and carried a `CONFIDENTIAL / IETF EXPERIMENTAL DRAFT` label. Its confidential header/contact block is **not republished here**. A public GitHub document is not confidential, and an IETF-like draft does not become an IETF Internet-Draft until it is actually submitted through the appropriate process.

The public README uses only the architecture concepts needed for the project presentation and applies the epistemic guardrails above.

## Review scope

The active documentation branch contains a tribunal request requiring visible separation between target architecture and current evidence. Independent reviewer verdicts were not generated in this task. The intended handoff remains human-review-only; no automatic merge or release is authorized.
