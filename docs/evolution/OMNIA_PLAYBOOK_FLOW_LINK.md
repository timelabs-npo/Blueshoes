# Blueshoes ↔ omnia-playbook flow reconciliation link

This Blueshoes evolution branch is paired with:

- repository: `timelabs-npo/omnia-playbook`
- branch: `evolution/network-flow-semantic-redteam-v1`
- semantic scope: `FlowObservationV1` + independent network-flow red-team contracts
- ClashMac reference: `666OS/ClashMac@6bd4eee77ac3face93d6ba38fdc505e15a4e376e`

## Authority split

`omnia-playbook` defines semantic normalization and adversarial checks.

Blueshoes owns the typed Flow Graph / Flow Surgery capability model and native runtime integration.

No playbook document, UI event, model output, or helper process can authorize network mutation by itself.

## Initial shared objective

Establish a portable, observation-only flow object across darwin/win32/linux/openbsd before adding any mutation capability.

## Implemented read-only slice

The standalone `runtime/flow-observation` crate consumes only pinned JSON data and
schemas from omnia-playbook commit `30002c67533258691203391b4f0c30a3125d8e23`.
It imports no validator/oracle implementation. The corpus has 49 cases, including
four equivalent platform samples and independent adversarial/profile cases.
Windows TCP IPv4/IPv6 has actual query evidence; Darwin/Linux/OpenBSD native
collection remains NOT_EXECUTED. See [evidence](FLOW_OBSERVATION_EVIDENCE.md) for
the read-only gates, schema revision compatibility and remaining gaps.
