# Presentation source notes

Source inspection date: 2026-09-05. Baseline: `timelabs-npo/Blueshoes@9ad954c31d72e4f8a3f49171f799cae140e6b2f1` (default branch observed as `main`). This presentation does not incorporate or change other Rhea, OMNIA, MBSD, or Blueshoes repositories.

## Evidence map

- [Previous README](https://github.com/timelabs-npo/Blueshoes/blob/9ad954c31d72e4f8a3f49171f799cae140e6b2f1/README.md): adaptive edge networking, rollback doctrine, no MITM, no autonomous LLM shell mutation, ECH preservation, no bundled commercial VPN endpoints; self-described B0 status. Its blanket runtime-status language is not a fresh validation result.
- [Cargo manifest](https://github.com/timelabs-npo/Blueshoes/blob/9ad954c31d72e4f8a3f49171f799cae140e6b2f1/runtime/bs-edge-agent/Cargo.toml): package 0.1.0; empty default features; opt-in `dangerous_execution`; watchdog binary.
- [Main entry point](https://github.com/timelabs-npo/Blueshoes/blob/9ad954c31d72e4f8a3f49171f799cae140e6b2f1/runtime/bs-edge-agent/src/main.rs): diagnostics, profile listing, JSON output, journal writes, planner, apply/confirm dispatch, and feature-dependent canary executor selection. The apply-confirmed branch selects DryRunExecutor but also invokes snapshot and watchdog code. Do not equate dry-run execution selection with absence of all side effects.
- [Phase 1 scope](https://github.com/timelabs-npo/Blueshoes/blob/9ad954c31d72e4f8a3f49171f799cae140e6b2f1/docs/rfcs/0012-phase1-scope.md): MT3000/FreeBSD development target; bounded transaction loop; global mesh telemetry and automated AI loop deferred. This is a specification, not a hardware qualification report.
- [Module inventory](https://github.com/timelabs-npo/Blueshoes/blob/9ad954c31d72e4f8a3f49171f799cae140e6b2f1/MODULE_INVENTORY.md): module roles; its coverage labels were not independently verified here.

## What was not established

No runtime compilation, QEMU test, physical router test, packet capture, independent security audit, watchdog survivability result, censorship-circumvention result, or production readiness is asserted. Existing security and operational authority are unchanged. Marketing comparisons are ambitions, not competitor benchmarks. “Free” is not a promise of free connectivity or absolute protection.

## Presentation assets

`assets/hero.svg` and `assets/route-story.svg` are original vector illustrations with CSS animation, accessible text alternatives, and reduced-motion rules. Their paths are illustrative, not measurements. Browser and GitHub image-proxy behavior can vary; still frames preserve the message.

`playground.html` is a standalone offline visualization, using only inline HTML, CSS, SVG, and JavaScript. It has no API, telemetry, remote fonts, package dependencies, or device access. All nodes, link costs, failures and receipts are synthetic. Its weighted shortest-path model is not the Blueshoes runtime algorithm. Costs are abstract units, not RTT, throughput, reliability, or savings estimates. An optional tunnel edge illustrates policy, not a functioning transport. Pausing affects animation only. Export saves a clearly labeled local simulation JSON receipt.

An additional image was generated through Higgsfield (job `f38b19bf-6c45-4082-84b3-d2a430dc80db`). It is linked as an optional external concept asset, not loaded automatically. The image could not be imported into the local verification environment; its offline availability and visual fidelity were not verified. No claim is made that the vector assets reproduce it exactly.

GitHub strips scripts from rendered Markdown, while supporting details/summary disclosures. The HTML companion must be downloaded and opened locally, or separately published by a maintainer; no Pages site was enabled by this change.

## Review scope

See [the review request](../../artifacts/devship/readme-showcase-request-20260905.json). Independent tribunal reviews were not performed. The intended handoff is a draft for human review, not an auto-approved or automatically merged release. Browser presentation checks do not establish runtime safety.


## Presentation verification

Chromium was exercised through Playwright using the self-contained page content. The agent-browser CLI was not installed; local file navigation was restricted, so the browser was given the HTML directly. Thirty bounded presentation checks passed: initial and alternate routes, over-budget rejection, tunnel opt-in, full isolation, reset, keyboard toggles and focus retention, pause/resume, JSON export and labeling, desktop/mobile geometry, overflow and target sizes, reduced-motion behavior, no automatic requests, and no browser JavaScript errors. JavaScript syntax was also checked with Node.

Desktop (1440 px) and mobile (390 px) full-page screenshots, the hero (1280 × 660), and the route illustration (1200 × 360) were visually inspected. A clipped decorative caption was corrected. Typography hierarchy, palette, spacing, responsive stacking, route labels, and figure boundaries were inspected. GitHub's own rendered README and image-proxy animation behavior have not been browser-verified. This is presentation verification only, not a runtime test suite or independent tribunal review.
