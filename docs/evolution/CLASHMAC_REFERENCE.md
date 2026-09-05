# ClashMac clean-room reference lock

Status: behavioral/architectural reference only; no source import.

Upstream: `666OS/ClashMac`
Pinned commit: `6bd4eee77ac3face93d6ba38fdc505e15a4e376e`
Observed repository class: public distribution/documentation repository for proprietary, closed-source software.

## Allowed use

- product behavior and UX inspiration from public documentation/screenshots/issues;
- protocol/interface ideas that are independently reimplemented;
- failure-mode and red-team case extraction from public issue history;
- public third-party attribution review where relevant.

## Forbidden use

- copying, reconstructing, decompiling, or redistributing proprietary ClashMac implementation;
- importing ClashMac binaries/assets as Blueshoes runtime dependencies;
- treating undocumented internal behavior as established fact;
- granting any ClashMac-like helper direct authority over Blueshoes state or policy.

## Architectural rule

ClashMac is a reference for native flow visibility and operator ergonomics. Blueshoes retains its own typed capability graph, deterministic policy boundary, and evidence discipline.

Any mutation inspired by ClashMac must pass through Blueshoes capability intent -> semantic validation -> host policy -> native adapter -> receipt. UI actions and privileged helpers are never independent authorities.
