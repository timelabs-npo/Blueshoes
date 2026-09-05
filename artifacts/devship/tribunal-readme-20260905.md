# Tribunal Request — README redesign

Date: 2026-09-05
Scope: documentation / presentation only
Branch: `readme/blue-death-to-vpn`

## Proposed change

Replace the repository README with a high-impact project presentation and add presentation-only visual assets. The README will describe Blueshoes as an OpenBSD-first, RAM-only, AI-driven adaptive router OS / OpenWrt replacement, while explicitly separating the **target architecture** from what is **currently implemented and verified in this repository**.

## Authority / safety boundaries

- No runtime mutation.
- No executor, watchdog, rollback, secrets, firewall, tunnel, persistence, boot, or kernel code changes.
- No new cloud dependency.
- No autonomous release or merge.
- Existing repository claims are not upgraded to “verified” without direct evidence.
- Legacy FreeBSD implementation paths are labeled as legacy/current bootstrap where applicable rather than silently rewritten as OpenBSD-complete.
- “AI-driven”, “self-evolving”, “self-healing”, “RAM-only OS”, “Rheknel in BSD core”, and “OpenWrt replacement” are presented as target architecture unless backed by current repository evidence.

## Independent review prompts

### Security reviewer
Does the README accidentally imply unsafe autonomous routing mutation, weaken the double-gate model, or misrepresent privacy/censorship-resilience properties as guarantees?

### Governance reviewer
Does the README preserve human authority, distinguish target architecture from verified implementation, and avoid claiming that aspirational components are already enforced?

### Architecture consistency reviewer
Does the README correctly describe the intended OpenBSD-first RAM-only firmware architecture while acknowledging the repository’s present Rust/FreeBSD-oriented runtime paths and current dry-run behavior?

## Aggregate pre-execution verdict

Proceed with documentation-only changes if and only if the final README keeps a visible **Reality Check / Current State** section and makes no claim that anonymity, censorship resistance, self-healing, or OpenBSD migration is already proven.
