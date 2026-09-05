# Tribunal Request — README Flows / Flow Surgery / blueshoes.space pass

Date: 2026-09-05
Scope: public presentation / documentation only
Target: `main`
Human authorization: explicit user request to edit the live GitHub README and add the requested presentation treatment.

## Proposed change

Upgrade the public Blueshoes presentation with:

- mathematically explicit **Flows** language based on time-varying directed multigraphs, conservation, capacity, cuts, cycle-space intuition and constrained multi-path transformations;
- a named **Flow Surgery toolkit** whose public verbs are presentation-level operations: `CUT`, `BYPASS`, `SPLICE`, `BRAID`, `GRAFT`, `SEAL`, `ROLLBACK`;
- a bright original animated SVG explaining those operations;
- clearly labeled parody text marks `FAKE SPONSOR // F1*` and `FAKE SPONSOR // X*`, with an explicit non-affiliation note and no copied logos or implied sponsorship;
- a playful `blueshoes.space` “spaceport” motif and preparatory static-site/domain wiring, without claiming DNS or GitHub Pages is currently active.

## Scientific / semantic guardrails

- In this README, “topology” means the connectivity structure of a graph / capability graph, not an assertion that the physical Internet is a differentiable manifold.
- A flow is not equated with a single path or a TCP five-tuple. The mathematical illustration uses a feasible graph flow `f` satisfying capacity constraints and flow conservation at transit vertices.
- “Flow surgery” means bounded transformations of the support/capability graph that preserve declared endpoint intent and invariants. It is not a claim that the current runtime already implements every named operation.
- Cycle-space / kernel language is used only where mathematically applicable (`B Δf = 0` for boundary-preserving re-routing intuition); no unsupported homology/manifold claim is made.
- Any added capability edge (overlay, tunnel, mesh, SCION gateway) still requires a real reachable substrate. The presentation may not invent connectivity.
- No claims of perfect invisibility, universal censorship resistance, production post-quantum negotiation, or deployed self-healing are added.

## Domain boundary

The repository currently reports GitHub Pages disabled. Creating a `docs/CNAME` and static `docs/index.html` is only preparatory wiring for `blueshoes.space`; actual service requires GitHub Pages activation plus DNS configuration outside this repository. The README must not state that the domain is live unless directly verified.

## Authority boundary

No runtime, executor, firewall, boot, kernel, secrets, cryptographic implementation, CI authority, or release logic is changed. This documentation update does not promote any runtime claim to verified status.
