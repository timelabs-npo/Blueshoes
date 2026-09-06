# blueshoes.space — lighthouse, not root

`blueshoes.space` is the public **presentation / experiment / receipt surface** for Blueshoes. It must never become a required control plane, trust anchor, DNS root, routing oracle, or bootstrap dependency for the router itself.

> The domain may explain the network. It must not become the network's sovereign.

## Production state — 2026-09-06

The durable apex is now served by the Cloudflare Worker **`blueshoes-spaceport`** using static assets from `docs/`.

- `https://blueshoes.space/` → `blueshoes-spaceport` Worker → static Spaceport assets.
- `demo.blueshoes.space` → optional Cloudflare Tunnel → physical lab/router demo when intentionally online.
- The obsolete apex Tunnel DNS record that previously produced Cloudflare error `1033` has been removed.
- The apex is no longer coupled to Tunnel health.
- The Tunnel is deliberately isolated to `demo.` so it may disappear without taking the public Spaceport with it.

This preserves the architectural rule:

> **THE WORMHOLE MAY DIE. THE FLOW DOES NOT.**

## Surface topology

```text
                        blueshoes.space
                              │
                    ┌─────────┴─────────┐
                    │  PUBLIC LIGHTHOUSE │
                    │  no runtime power  │
                    └─────────┬─────────┘
                              │
        ┌───────────────┬─────┼─────┬────────────────┐
        │               │     │     │                │
        ▼               ▼     ▼     ▼                ▼
      /repo            /lab  /map  /receipts        /rfc
    GitHub source      Flow   topology evidence     architecture
                      Surgery  theatre  artifacts    drafts
        │               │     │     │                │
        └───────────────┴─────┴─────┴────────────────┘
                              │
                       presentation only
                              │
                    ─ ─ ─ trust boundary ─ ─ ─
                              │
                              ▼
                      BLUESHOES ROUTER
                    local admission research

              demo.blueshoes.space
                        │
                 ephemeral Tunnel
                        │
                        ▼
                 physical lab/router
```

### `/`
High-art manifesto + project map. Static by default. It should still work with JavaScript disabled.

### `/lab`
The **Flow Surgery toolkit** playground: synthetic graph, CUT / BYPASS / SPLICE / BRAID / GRAFT / SEAL / ROLLBACK. Clearly labeled simulation; never imply live router mutation.

### `/map`
Redirects to `/rhea/`: the public family map, six core projects and nine client/library/learning/distribution/organization entrances. The map describes responsibilities and current source status. It is not live network telemetry.

### `/receipts`
Redirects to the checked-in `artifacts/devship/` directory. Inspect each artifact for what it actually records. Target-hardware, negotiated PQ/T and fault-injection evidence remain desired future categories unless a particular qualifying receipt exists; this path does not assert their completion.

### `/rfc`
Architecture and experimental drafts with immutable links to exact Git commits.

### `/repo`
Permanent redirect to `https://github.com/timelabs-npo/Blueshoes`.

## Fun subdomain scheme

These are presentation names, not required protocol names:

| Surface | Suggested hostname | Purpose |
|---|---|---|
| Spaceport | `blueshoes.space` | canonical public landing surface |
| Surgery lab | `lab.blueshoes.space` | interactive synthetic Flow Surgery |
| Observatory | `map.blueshoes.space` | topology / Flow visualization |
| Evidence vault | `receipts.blueshoes.space` | reproducible proofs and artifacts |
| RFC dock | `rfc.blueshoes.space` | specs / drafts / architecture |
| Ephemeral wormhole | `demo.blueshoes.space` | temporary live demo only; safe to disappear |

Do **not** use `control.blueshoes.space`, `authority.blueshoes.space`, or any other hostname that implies remote sovereignty over the router.

## Hosting doctrine

### Canonical apex — Cloudflare Worker + static assets

`blueshoes-spaceport` is the durable presentation origin for the apex. The repository remains the source of truth; the Worker is delivery infrastructure only.

The production Wrangler contract lives in `wrangler.spaceport.jsonc` and points to `./docs` as the static asset directory.

### Ephemeral demo — Cloudflare Tunnel

`demo.blueshoes.space` is the only intended Tunnel-facing public hostname. It may expose a physical router/lab experiment when deliberately enabled.

If `demo.` disappears, the router should continue functioning and the main site should remain up.

### Optional future surfaces

`lab.`, `map.`, `receipts.`, and `rfc.` may remain paths under the apex or become dedicated subdomains later. Their hosting choice must not expand the router's trust boundary.

## Deployment hygiene

The earlier bootstrap mechanisms have served their purpose and should not be treated as production paths:

- tokenless temporary Worker preview/claim workflow — retired after permanent Worker deployment;
- temporary `wrangler.preview.jsonc` — retired;
- GitHub Pages fallback workflow — retired after the Cloudflare Worker became the canonical origin.

Keep the production deployment path small: `docs/` + `wrangler.spaceport.jsonc` + the permanent Cloudflare deployment workflow/configuration.

## Verification contract

A production deployment is considered externally healthy when all of the following hold:

1. `https://blueshoes.space/` returns HTTP `200`.
2. TLS is valid.
3. The response contains the Flow Surgery Spaceport content.
4. referenced static assets load successfully.
5. the apex is not routed through a Cloudflare Tunnel.
6. `demo.blueshoes.space` remains isolated as the optional Tunnel surface.
7. loss of `demo.` cannot take down the apex.

## Constitution

**blueshoes.space is a lighthouse, not a root.**

It can host art, explanations, experiments, public topology, and receipts. The target Blueshoes runtime must not require this domain, GitHub, Cloudflare or a public dashboard to route. This is a design requirement; the public site does not establish a qualified routing runtime.
