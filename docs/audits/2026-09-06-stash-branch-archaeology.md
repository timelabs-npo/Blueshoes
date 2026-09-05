# Stash propagation and branch archaeology: Blueshoes

This documentation-only record routes archival evidence to its relevant repository. Snapshot: **2026-09-06, Europe/Moscow**, before the documentation branches created by this pass. Every comparison below uses fixed commit IDs.

The canonical archive is [rhea-project/stash](https://github.com/timelabs-npo/rhea-project/blob/3316bae0770744238099c25ae34e76e7ad4af8b4/stash/README.md). It is a normal Git branch named `stash`, separate from local `refs/stash`. Its 37 archive files total **361,824 bytes**; this pass reconstructed their UTF-8 bytes locally, verified each Git blob SHA-1 and size, and verified SHA-256 after disk readback. The four content-addressed original reports also match the SHA-256 encoded in their paths and total **115,053 bytes**.

The [original collection manifest](https://github.com/timelabs-npo/rhea-project/blob/3316bae0770744238099c25ae34e76e7ad4af8b4/stash/runs/2026-09-06-cloud-001/manifest.json) still records **41 pending items/groups** and `PARTIAL_WD_UNAVAILABLE`. That is the original cloud capture's state, not a statement that this Windows host lacks filesystem access. Mirroring the published archive does not collect the binaries, source trees, VM disks or histories merely named in those reports. Those pending artifacts were not captured in this pass.

At the six inspected main tips, no blob matches any of the 37 `stash/` archive blobs. This is exact-content evidence, not proof that no paraphrases, links or equivalent implementation exist. The propagation proposed here is a pinned documentation pointer and repository-specific findings; implementation adoption remains a separate change.

## Repository findings and routing

[PR #8](https://github.com/timelabs-npo/Blueshoes/pull/8), `codex/spaceport-live-redesign@e8f6dd15506e12c76d1622fbe951f46c72a15830`, is **2 ahead / 5 behind** main. The main-side commits add a Rhea page, remove temporary Cloudflare/GitHub Pages fallback configuration and record production topology; the draft changes the permanent deployment workflow and Spaceport assets. Reconcile those surfaces on a pinned candidate before merging; this audit did not deploy or validate the live site.

[PR #6](https://github.com/timelabs-npo/Blueshoes/pull/6) is recorded as merged, although `readme/blue-death-to-vpn` is **15 ahead / 23 behind** main by commit ancestry. Of its 12 changed paths, **10 currently have identical Git blobs on main**. Only `README.md` and `docs/readme/assets/hero.svg` differ in this path set. Therefore the ancestry count does not mean all 15 commits' content is missing, and blindly reapplying the branch would duplicate or reverse existing presentation work.

[Draft PR #9](https://github.com/timelabs-npo/Blueshoes/pull/9), `evolution/clashmac-flow-observation-v1@0851fa29f3b4b175f462ef196c81f4cec052df54`, is **4 ahead / 1 behind** main and changes four adoption/reference/link/review documents. Its companion is [Omnia Playbook #10](https://github.com/timelabs-npo/omnia-playbook/pull/10), not evidence of a deployed Flow implementation.

The original [stash collection queue](https://github.com/timelabs-npo/rhea-project/blob/3316bae0770744238099c25ae34e76e7ad4af8b4/stash/runs/2026-09-06-cloud-001/pending.json) retains unresolved Blueshoes B0 binary locators. The [binary-provenance derivative](https://github.com/timelabs-npo/rhea-project/blob/3316bae0770744238099c25ae34e76e7ad4af8b4/stash/runs/2026-09-06-cloud-001/supplied-binary-provenance.json) records supplied findings; it is not a new executable audit. Preserve the six binary identities and scope limits when propagating those findings. Actual binary recovery and execution were not performed in this pass.

Route the relevant provenance observations through this documentation record; review the deployment and Flow PRs independently. A green documentation or advisory workflow would not establish live forwarding, rollback correctness or v2 acceptance.

## Branch ledger

Pinned main: `384abbd5cae60f93cf29a5fc07af4f16854313e1`. Ahead/behind counts measure commit ancestry relative to that main. They do not measure missing patches, successful tests or merge readiness. Historical merged PRs can refer to older heads, or contain content integrated without the original ancestry.

| Branch | Pinned head | Ahead / behind main | PR evidence |
| --- | --- | --- | --- |
| `codex/spaceport-live-redesign` | [`e8f6dd15506e`](https://github.com/timelabs-npo/Blueshoes/commit/e8f6dd15506e12c76d1622fbe951f46c72a15830) | 2 / 5 | [#8](https://github.com/timelabs-npo/Blueshoes/pull/8) open draft |
| `copilot/conduct-marketing-legal-audit` | [`07bfdf2fbb19`](https://github.com/timelabs-npo/Blueshoes/commit/07bfdf2fbb19a32072ab0910f9eaa1eef68b74b9) | 1 / 47 | [#1](https://github.com/timelabs-npo/Blueshoes/pull/1) open draft |
| `copilot/explain-repository-structure` | [`ce316791fc17`](https://github.com/timelabs-npo/Blueshoes/commit/ce316791fc171d48584b32d7e1790008fb8328c6) | 0 / 52 | none in retrieved PR history |
| `copilot/review-session-history` | [`6f4fc621d50c`](https://github.com/timelabs-npo/Blueshoes/commit/6f4fc621d50cbfd3ebb7c4c9588a8c55fa5ef301) | 0 / 50 | none in retrieved PR history |
| `copilot/timelabs-nothing` | [`9ad954c31d72`](https://github.com/timelabs-npo/Blueshoes/commit/9ad954c31d72e4f8a3f49171f799cae140e6b2f1) | 0 / 23 | none in retrieved PR history |
| `dependabot/cargo/runtime/bs-edge-agent/cargo-faef625f8c` | [`c5ee23dcb005`](https://github.com/timelabs-npo/Blueshoes/commit/c5ee23dcb005ffcae325199d060c41c858451b87) | 1 / 23 | [#2](https://github.com/timelabs-npo/Blueshoes/pull/2) open |
| `dependabot/go_modules/spanner_demo/go_modules-300547a8bd` | [`92a2c803796b`](https://github.com/timelabs-npo/Blueshoes/commit/92a2c803796bb884fa5d3ba1a759ff70a5540255) | 1 / 23 | [#4](https://github.com/timelabs-npo/Blueshoes/pull/4) open |
| `docs/blueshoes-internet-free-20260905` | [`7367fda31ed6`](https://github.com/timelabs-npo/Blueshoes/commit/7367fda31ed6502905e0482f3a68d0557f444aa7) | 1 / 23 | [#7](https://github.com/timelabs-npo/Blueshoes/pull/7) open draft |
| `evolution/clashmac-flow-observation-v1` | [`0851fa29f3b4`](https://github.com/timelabs-npo/Blueshoes/commit/0851fa29f3b4b175f462ef196c81f4cec052df54) | 4 / 1 | [#9](https://github.com/timelabs-npo/Blueshoes/pull/9) open draft |
| `main` | [`384abbd5cae6`](https://github.com/timelabs-npo/Blueshoes/commit/384abbd5cae60f93cf29a5fc07af4f16854313e1) | 0 / 0 | none in retrieved PR history |
| `readme/blue-death-to-vpn` | [`1c33fc7c3c7c`](https://github.com/timelabs-npo/Blueshoes/commit/1c33fc7c3c7c620345ec74ca8f097dd49129e966) | 15 / 23 | [#6](https://github.com/timelabs-npo/Blueshoes/pull/6) merged |

## Verification limits

All branch lists and PR lists fit within the 100-item first page. Comparisons cover every non-main branch. The checkpoint branch's explicit no-common-ancestor response is recorded as unrelated history. Recursive trees used for content identity checks were not truncated. GitHub comparison file lists can stop at 300 files; a 300-entry list is not a complete large-branch diff. No broad patch-equivalence analysis of the older Rhea histories was performed.

This pass used GitHub metadata, pinned trees, selected documents and local archive hashing. Component tests, builds, deployment checks, production runtime checks and pending WD artifact collection were not run. The archive's published Drive and scheduler receipts were read as historical records; those external states were not reverified or changed.
