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
