# blueshoes.space — lighthouse, not root

`blueshoes.space` is the public **presentation / experiment / receipt surface** for Blueshoes. It must never become a required control plane, trust anchor, DNS root, routing oracle, or bootstrap dependency for the router itself.

> The domain may explain the network. It must not become the network's sovereign.

## Observed external state

Checked 2026-09-05: `https://blueshoes.space` returned Cloudflare **error 1033**, which Cloudflare documents as a Tunnel error where no healthy `cloudflared` connector is available for the configured hostname. Treat this as an operational observation, not a permanent fact.

The repository already carries `docs/CNAME` with `blueshoes.space`, but a CNAME file alone does not activate GitHub Pages or reconfigure external DNS.

## Recommended surface topology

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
                    local truth = Rheknel
```

### `/`
High-art manifesto + project map. Static by default. It should still work with JavaScript disabled.

### `/lab`
The **Flow Surgery toolkit** playground: synthetic graph, CUT / BYPASS / SPLICE / BRAID / GRAFT / SEAL / ROLLBACK. Clearly labeled simulation; never imply live router mutation.

### `/map`
A visual topology theatre. Initially synthetic or replayed data. Later it may display privacy-preserving, explicitly opted-in measurements. Do not expose browsing histories, stable device identities, raw packet payloads, or secrets.

### `/receipts`
Machine-readable evidence: build hashes, target-hardware test receipts, negotiated PQ/T session examples, fault-injection reports, reproducibility records. This is the strongest long-term connection between the public site and Rheknel's epistemic philosophy.

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

## Hosting modes

### A — GitHub Pages as canonical static spaceport

Best for the manifesto, docs, SVG art, and static/synthetic lab. Publish `/docs` from `main`, configure `blueshoes.space` as the Pages custom domain, and point DNS to the Pages site. Simple, auditable, cheap.

### B — Cloudflare as presentation edge

Keep Cloudflare only as the public delivery edge / redirector while the canonical source remains Git. Useful for path routing, redirects, caching, headers, and temporary demo surfaces. Do not make Cloudflare a Blueshoes runtime dependency.

### C — Hybrid: static apex + ephemeral tunnel

Recommended fun mode:

- `blueshoes.space` → static Pages spaceport;
- `lab.` / `map.` → static or serverless visualization;
- `demo.` → an **ephemeral** Cloudflare Tunnel when a physical router demo is intentionally online.

If `demo.` disappears, the router should continue functioning and the main site should remain up. This is the architectural joke made real: **the wormhole may die; the Flow does not.**

## Current Cloudflare 1033 is useful information

The present `1033` state suggests the apex is configured through a Cloudflare Tunnel with no healthy connector. That is a poor default for the canonical landing page because the homepage dies whenever the tunnel dies.

Better split the responsibilities:

```text
STATIC, BORING, ALWAYS THERE       EPHEMERAL, FUN, ALLOWED TO VANISH
blueshoes.space                    demo.blueshoes.space
GitHub Pages / static origin       Cloudflare Tunnel → lab/router demo
```

## DNS / Pages activation checklist

1. Enable GitHub Pages for this repository using `main` + `/docs` or a Pages Actions workflow.
2. Configure the Pages custom domain as `blueshoes.space` in repository settings / Pages API.
3. Replace the current apex Tunnel route with Pages-compatible DNS records (temporarily DNS-only while certificates validate is the least surprising path).
4. Keep any Cloudflare Tunnel on a dedicated ephemeral hostname such as `demo.blueshoes.space`.
5. Add `www.blueshoes.space` as an optional redirect to the apex.
6. Verify HTTPS after DNS propagation and Pages certificate issuance.
7. Only then remove the **PRE-LAUNCH** label from the README/site.

## Constitution

**blueshoes.space is a lighthouse, not a root.**

It can host art, explanations, experiments, public topology, and receipts. Blueshoes must remain capable of routing when the entire domain, GitHub, Cloudflare, and every public dashboard are gone.
