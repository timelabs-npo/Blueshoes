<div align="center">

# BLUESHOES

## POWER HIDES AT THE CROSSING.

**Learn the path. Find the pressure. Build another way through.**

[Enter the Spaceport](https://blueshoes.space) · [Break a toy network](docs/readme/playground.html) · [Meet the family](https://blueshoes.space/rhea/) · [Inspect the source](runtime/bs-edge-agent)

</div>

A provider owns your exit. A resolver decides which name reaches which place. One tunnel becomes your entire idea of freedom. The connection works—until the crossing belongs to someone who says no.

**Blueshoes is network-flow research for people who intend to own their connection.** The ambition is a small routing machine that can observe its conditions, propose another admissible path, and account for every change it makes. Cheap hardware. Local control. A network with more than one possible future.

**Today:** this repository contains a Rust edge/runtime prototype, probes, planning and journaling code, and a synthetic Flow Lab. The integrated routing brick and automatic Flow Surgery remain the destination. [Current checks and limits](#put-your-hands-on-the-evidence) are below.

<p align="center"><img src="docs/readme/assets/flow-surgery.svg" alt="Concept illustration of Flow Surgery; not live telemetry or a completed routing system" width="100%" /></p>

## Two bridges. One very political map.

Imagine a town with two bridges to the place everyone works. One carries **4 vehicles per minute**, the other **3**. Five people arrive each minute. Send four one way and one the other: everyone crosses.

Now close the larger bridge.

```text
                 bridge A: 4 vehicles/minute
             ┌──────────────────────────────┐
      TOWN ──┤                              ├── WORK
             └──────────────────────────────┘
                 bridge B: 3 vehicles/minute

Demand: 5/minute     Both open: 4 + 1     A closed: 3 through, 2 waiting
```

This is an invented example. It assumes independent bridges and access roads with no tighter limit. It is not a benchmark or a demonstration of a Blueshoes solver.

The remaining bridge can carry three. At least two must wait, be refused, or find **another real crossing**. Rename the road, redraw the map, ask a thousand models: the missing capacity stays missing.

That is the doorway into three powerful ideas:

| Idea | The question | What it reveals |
|---|---|---|
| **Topology** | What can reach what? | Connections, alternate paths, and the cuts that isolate a place. The drawing can bend; the connections are what matter. |
| **Geometry** | How do we measure and compare those possibilities? | A chosen notion of distance, delay or cost can expose an expensive detour. Not every cost is a mathematical distance; capacity and policy impose additional constraints. |
| **Flow** | What actually moves, and how much gets through? | Traffic, queues and unmet demand. A path is a possibility; a flow spends real time and capacity. |

**The hidden power is the crossing everyone needs.** Whoever can limit, price, delay or refuse it can shape everybody else's options. This is why topology is worth understanding even if you have never opened a router configuration.

## Keep the intention. Question the path.

A video call needs to carry a conversation. It does not owe allegiance to one tunnel endpoint.

Blueshoes calls the traffic relation we want to preserve a **Flow**. Its supporting paths may change, provided the declared endpoint, capacity, policy and cryptographic constraints still hold. That is the target behind **Flow Surgery**: precise changes to connectivity, followed by checks of what actually happened.

| Proposed operation | Its job |
|---|---|
| **CUT** | Remove a failed or forbidden edge from the candidate graph. |
| **BYPASS** | Seek another feasible support for the same demand. |
| **SPLICE** | Join compatible path segments through a real gateway. |
| **BRAID** | Share demand across multiple paths within their joint constraints. |
| **GRAFT** | Add a genuinely reachable peer, tunnel or other capability. |
| **SEAL** | Record the result after the specified checks. |
| **ROLLBACK** | Restore prior admitted state when an implemented recovery path permits it. |

These are **design vocabulary**, not seven completed runtime commands. A graft needs a reachable substrate. A rollback needs material that actually restores the state. If no admissible path exists, the honest result is rejection.

<details>
<summary><strong>The mathematics beneath the picture</strong></summary>

On a fixed directed graph, let `f` be nonnegative edge flow, `c` edge capacities, `B` an oriented incidence matrix, and `b` the source/sink demand vector with the matching sign convention. Feasibility requires:

$$
Bf=b,\qquad 0\le f\le c.
$$

Two feasible flows with the same boundary have difference `Δf` satisfying `BΔf=0`. Internal carriage can change while net supply and demand stay fixed. Multiple traffic classes also share capacity: their flows on an edge must sum to no more than its capacity.

This is an abstract flow model. Packet ordering, latency, transport behavior, policy and negotiated cryptography need additional constraints and executable validation. No smooth-manifold or Ricci-flow routing implementation is implied.

</details>

## Intelligence gets a scalpel. Authority holds the handle.

The intended system separates observation, proposals, admission, execution and evidence:

```text
observe → propose a change → local admission → bounded executor → check the effect
                                                │                       │
                                                └──── recovery path ────┘
```

An observer may notice a timing change or a broken path. That does not prove censorship: congestion, radio conditions and ordinary failures can look similar. A model may propose surgery. Agreement among models does not grant a capability.

[Rheknel](https://github.com/timelabs-npo/rheknel) explores the deterministic boundary; its current main is still a dispatcher prototype. [Omnia Playbook](https://github.com/timelabs-npo/omnia-playbook) supplies operational knowledge. [MBSD](https://github.com/timelabs-npo/mbsd) investigates the operating substrate. This is the intended composition, not a claim that those pieces are already a qualified integrated appliance.

## The horizon is deliberately large.

**Beyond one VPN:** make tunnels one available tool among several, with explicit prerequisites and limits.

**Beyond a fixed network shape:** explore alternative paths, naming mechanisms and reachable peers as separate capabilities. Software does not manufacture radios, gateways or exits.

**Toward post-quantum transport:** make cryptographic requirements observable and testable at the negotiated-session boundary. No verified Blueshoes PQ transport or board benchmark is claimed here.

**Toward a small, forgetful machine:** an OpenBSD-oriented, memory-resident design with explicit durable-state exceptions. The source currently contains FreeBSD-oriented runtime paths; MBSD has OpenWrt and OpenBSD research lanes. Whole-system RAM-only operation and physical qualification still require evidence.

The ambition is freedom with mechanisms you can inspect. Universal invisibility, guaranteed bypass and perfect erasure would require evidence this project does not have.

## Put your hands on the evidence

Start with the [runtime source](runtime/bs-edge-agent), [architecture drafts](docs/rfcs/), [security policy](SECURITY.md), and [historical presentation source notes](docs/readme/SOURCE_NOTES.md). The source notes record an older baseline; they do not replace current checks.

**Verification on 2026-09-06, source baseline [`384abbd5`](https://github.com/timelabs-npo/Blueshoes/commit/384abbd5cae60f93cf29a5fc07af4f16854313e1):**

```bash
# From the repository root: full suite currently fails in the watchdog build.
make test

# A narrower, passing check:
cd runtime/bs-edge-agent
cargo test --locked --bin bs-edge-agent
```

The scoped command passed **20 tests**. The full command failed with `E0432` (`crate::executor`) and `E0411` (`Self`) in `src/watchdog.rs`. Neither result qualifies a router, rollback behavior or adversarial resilience. A command labeled `dry-run` is not a blanket promise of no side effects; selected paths also call snapshot/watchdog machinery.

[Flow Observation PR #9](https://github.com/timelabs-npo/Blueshoes/pull/9) is a separate experimental workstream, open at this snapshot. Its code and reported results are not part of this main-branch baseline.

For a hands-on first encounter, open the [synthetic Flow Lab](docs/readme/playground.html), remove a link and watch the available paths change. Its output is deliberately labeled synthetic.

## A public light. A local machine.

[blueshoes.space](https://blueshoes.space) hosts the public explanations, family map and experiments through Cloudflare Workers. It has no authority over your router. The domain is an entrance to the work, not a dependency the target runtime should need to keep routing.

[The family map](https://blueshoes.space/rhea/) connects Rhea/Tribunal, Rheknel, MBSD, both Omnia projects, Atlas, Play, iOS, Keyboard, CLI, Memory, Tutorials and Homebrew. Choose your point of entry: a packet, a proof obligation, a confusing sentence, a cheap board that deserves better software.

**Путь можно поменять. Право понимать, что происходит, — оставить себе.**

---

[MIT](LICENSE) · [Security](SECURITY.md) · [Public work map](https://github.com/timelabs-npo/.github/blob/main/docs/FACADE_WORK_MAP.md)
