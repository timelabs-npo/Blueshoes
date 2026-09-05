# ClashMac → Blueshoes adoption map

Status: proposed clean-room evolution slice; not runtime-qualified.

## Adopt conceptually

- Active flow observation: process/app identity, source/destination, protocol, byte counters, rates, timestamps, matched policy, route/interface metadata.
- Native connection/topology visualization as a projection of observed flow state.
- Operator UX for filtering/searching flows and proposing route/block/bypass/pin actions.
- Traffic analytics and local-first history projections.
- Native helper/adapter separation as a deployment lesson only.

## Reimplement behind Blueshoes contracts

ClashMac-like observations normalize into `FlowObservationV1` and never become authority by themselves.

ClashMac-like operator actions normalize into typed proposals/capabilities such as:

- `ObserveFlow`
- `TerminateFlow`
- `AddRoutePolicy`
- `RemoveRoutePolicy`
- `SetProxyMode`
- `EnableTunnel`
- `DisableTunnel`
- `PinEgress`
- `BypassDomain`
- `BlockDestination`
- `ChangeResolver`

The UI may express intent; only host policy can authorize effect.

## Explicitly reject/defer

- direct UI -> socket/process mutation;
- helper/XPC process as a second policy authority;
- direct one-click rule-file mutation;
- opaque engine auto-install/update or kernel lifecycle control;
- telemetry claims derived from obfuscated display coordinates without provenance labels;
- closed-source binary reuse;
- any action that bypasses `omnia-playbook` semantic checks or Blueshoes receipts.

## Product destination

| ClashMac product concept | Blueshoes destination |
|---|---|
| Connections table | Flow Observation workspace |
| Rule match / quick rule | Flow Surgery proposal maker |
| Route map | Flow topology / route projection |
| Connection topology | Semantic Flow Graph |
| Traffic curves | Telemetry projections |
| TUN/system proxy switch | Future native adapter capabilities |
| Privileged helper | Host-owned adapter behind policy grant |
| Connection cutoff | Future `TerminateFlow` capability |

## First implementation slice

Start read-only:

1. define `FlowObservationV1`;
2. add platform adapter interfaces for darwin/win32/linux/openbsd;
3. create deterministic fixture normalization tests;
4. render a topology projection from fixture data;
5. keep all mutations `NOT_EXECUTED` until independent checks exist.
