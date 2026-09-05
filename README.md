<p align="center">
  <img src="docs/readme/assets/hero.svg" width="100%" alt="Blueshoes. The internet is free. Again! Electric-blue shoes escaping a chrome tollgate. Stop renting the way out." />
</p>

<p align="center">
  <strong>A love letter to the open internet.<br>A breakup letter to the reconnect button.</strong>
</p>

<p align="center">
  <a href="#the-big-idea">The big idea</a> ·
  <a href="#take-it-for-a-walk">Take it for a walk</a> ·
  <a href="#under-the-laces">Under the laces</a> ·
  <a href="#what-is-real-today">What is real today</a> ·
  <a href="#build-the-way-out">Build the way out</a>
</p>

---

## The big idea

**You wanted a website. Somehow, that became a subscription, a server picker, and a little spinning circle.**

Blueshoes is an open-source experiment in **adaptive, local-first network access**: put more intelligence at your edge, observe what actually works, and make changes with an explicit way back.

The ambition is deliciously unreasonable: **make the paid-tunnel workaround unnecessary for ordinary access.** Not by reselling the same tunnel in a nicer coat. By rethinking how your network responds when the obvious path stops working.

**Keep encryption. Lose dependence.**

> **Research prototype, not a released VPN replacement.** The slogan is the destination; the code is the journey. This repository contains a Rust edge-agent reference implementation and its design corpus. Universal unblocking, anonymity, production rollback safety, and hardware readiness are not established by this README. [See the source notes.](docs/readme/SOURCE_NOTES.md)

### Not another VPN. Not a war on encryption.

| The experience we want to retire | The direction we are building toward |
| :--- | :--- |
| “Pick another server. Try again.” | Observe the failure, then evaluate a permitted alternative. |
| A subscription becoming the default answer to access | Operator-owned decisions at the network edge. |
| One broken change taking the whole connection down | Bounded transactions, validation, and a tested way back. |
| A glowing “connected” badge explaining nothing | Inspectable plans, local observations, and explicit failure. |

This is a **product ambition**, not a claim that every VPN behaves this way or that Blueshoes already replaces one. Encrypted tunnels remain useful. The project includes an explicit **operator-configured tunnel** profile; it does not bundle commercial VPN endpoints. A tunnel can be a tool. It should not have to be your entire relationship with the internet.

## Take it for a walk

<img src="docs/readme/assets/route-story.svg" width="100%" alt="Concept animation: an alternate path curves around a blocked link. Observe, plan, validate, keep or roll back. Not live telemetry." />

**Break a link. Tighten the budget. Watch the options disappear—or another route light up.**

The [interactive route playground](docs/readme/playground.html) is a tiny, dependency-free HTML companion. Download that file and open it in a browser; GitHub displays HTML source rather than running the page inside a README. No install, account, API key, or network probe is needed.

It uses a **synthetic graph and invented link costs**, not the Rust engine. It demonstrates route selection, policy constraints, and honest “no route” outcomes—not censorship resistance or real rollback. The README illustrations animate without JavaScript; reduced-motion preferences are respected.

<details>
<summary><strong>🧱 The obvious route breaks. Now what?</strong></summary>

The design goal is to inspect available, permitted alternatives rather than blindly repeat the same attempt. In the playground, click **Break direct path**. The highlighted route and its synthetic cost change. If no route satisfies your policy, the answer is **NO PERMITTED ROUTE**—not a green badge and wishful thinking.

</details>

<details>
<summary><strong>🪂 The clever change makes things worse. Now what?</strong></summary>

**Rollback is sacred.** The intended transaction model is observe → prepare → apply within bounds → validate → retain or revert. Recovery must survive the failure it is supposed to repair. A lovely diagram, a journal entry, and a hash are not substitutes for fault-injection tests on the target system.

The playground does not exercise the runtime watchdog or router recovery. [Read the Phase 1 scope.](docs/rfcs/0012-phase1-scope.md)

</details>

<details>
<summary><strong>👟 So… can I uninstall my VPN?</strong></summary>

**Not on the strength of this repository.** Blueshoes is experimental. It is not an anonymity network, an independently audited privacy product, or a promise that every blocked service becomes reachable. Do not replace a working security setup with a manifesto.

“Free” means the ambition of open access and operator control—not free ISP service, free infrastructure, or a guarantee against censorship.

</details>

## Under the laces

**A Rust edge agent. A transaction model. An unusually serious relationship with the undo button.**

The core lives in [`runtime/bs-edge-agent/`](runtime/bs-edge-agent/). Its source includes diagnostic commands, JSON output and journaling, profile planning, execution boundaries, and recovery-related modules. The architecture is documented in [`docs/rfcs/`](docs/rfcs/); the [module inventory](MODULE_INVENTORY.md) is the map, not a test certificate.

```text
                 intended control loop

  observe ──→ propose ──→ constrain ──→ validate
                             │              │
                     operator authority    ├── keep
                                            └── roll back

       the network changes; ownership stays local
```

**Non-negotiables in the project design:** no MITM or TLS interception; preserve ECH rather than pretend to force it; no autonomous LLM shell mutation; no bundled commercial VPN endpoints; explicit human authority for high-risk operations. See the [security policy](SECURITY.md) and [RFC corpus](docs/rfcs/).

## What is real today

| Layer | Evidence and boundary |
| :--- | :--- |
| **Rust reference source** | `bs-edge-agent` is version `0.1.0` in Cargo. CLI diagnostics, profiles, journaling, planning, and recovery-related code are present. Source presence is not an execution result. |
| **Default build configuration** | Cargo declares `default = []`; `dangerous_execution` is an opt-in feature. Do not infer that every command is side-effect-free. |
| **Execution wiring** | The inspected `apply-confirmed` path selects `DryRunExecutor`, but also calls snapshot/watchdog machinery. Canary has feature-dependent executor selection. Neither is a casual installation step. |
| **Hardware / OS target** | Phase 1 names GL.iNet GL-MT3000 and a FreeBSD-oriented runtime. That is a development target, not proof of a supported FreeBSD firmware image or validated board bring-up. |
| **Future network-wide behavior** | Shared mesh telemetry and an automated AI control loop are deferred in Phase 1. Do not read these as shipping features. |
| **This visual playground** | A standalone educational simulation. No real traffic, router access, measurements, or runtime safety verification. |

Source snapshot: [`9ad954c`](https://github.com/timelabs-npo/Blueshoes/commit/9ad954c31d72e4f8a3f49171f799cae140e6b2f1). [Exact files and interpretation limits →](docs/readme/SOURCE_NOTES.md)

## Build the way out

**Bring evidence. Bring taste. Bring a very unreasonable dislike of “reconnecting…”**

Start with the source—not a `curl | sudo` leap of faith:

```bash
git clone https://github.com/timelabs-npo/Blueshoes.git
cd Blueshoes

# Developer entry point; requires a compatible Rust toolchain.
# This command is not a firmware installer or a tested release recipe.
cargo run --manifest-path runtime/bs-edge-agent/Cargo.toml \
  --bin bs-edge-agent -- --help
```

Do not enable `dangerous_execution`, run mutation commands, or install onto your only gateway just to explore the project. Compilation and target execution were not established by this presentation change.

The most valuable contributions are **reproducible failure cases, rollback fault injection, explicit threat models, verified platform bring-up, and interfaces that tell the truth**. Open an [issue](https://github.com/timelabs-npo/Blueshoes/issues) with the exact commit, environment, reproduction, and observed result. Keep private network details and credentials out of public reports; consult [SECURITY.md](SECURITY.md) for security reporting.

<details>
<summary><strong>🎨 Open the visual kit</strong></summary>

[Animated hero](docs/readme/assets/hero.svg) · [Route illustration](docs/readme/assets/route-story.svg) · [Interactive HTML](docs/readme/playground.html) · [Presentation notes](docs/readme/SOURCE_NOTES.md)

The checked-in SVGs and playground are self-contained; no tracking pixels, remote fonts, or analytics are included. An additional generated [chrome-and-cobalt concept illustration](https://d8j0ntlcm91z4.cloudfront.net/user_34VUArl0ObNryw2EGhUaoDEmHaR/hf_20260905_180711_f38b19bf-6c45-4082-84b3-d2a430dc80db.png) is hosted externally and is optional. It is artwork, not a product photograph.

</details>

---

<p align="center">
  <strong>The internet is free. Again!</strong><br>
  <sub>That is the future we are here to build. Lace up.</sub><br><br>
  <a href="LICENSE">MIT licensed</a> · <a href="https://github.com/timelabs-npo/Blueshoes">Made in the open</a>
</p>
