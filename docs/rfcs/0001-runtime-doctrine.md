# Core Doctrine

Blueshoes operates on a strict set of foundational rules. These ensure the system remains safe, predictable, and fully recoverable.

## 1. The Core Goal
Maximize internet access availability globally while minimizing per-node access cost. We achieve this via deterministic routing adaptation.

## 2. Rollback Continuity
If a new routing profile breaks internet access, the router must revert to the previous known-good state within 5 seconds. The user should never be left permanently offline due to a failed obfuscation attempt.

## 3. Cryptographic Integrity
Transparent Man-In-The-Middle (MITM) architecture is forbidden. Blueshoes will never decrypt, inspect, or modify TLS payloads, nor will it require synthetic root certificates.

## 4. Diagnostic Boundary
Large Language Models (LLMs) are used strictly for offline log analysis and profile recommendations. They run on external hardware (the Workbench) and have **zero authority** to issue state-mutating shell commands on the router itself.

## 5. Non-Destructive Removability
Blueshoes must operate cleanly. Disabling or removing the Blueshoes agent must perfectly restore the vanilla FreeBSD routing configuration without leaving residual broken states behind.

## 6. Abuse Resistance & Compliance
Blueshoes is built to improve lawful access and availability, not to enable profiteering or covert rerouting.

1. **No Bundled Commercial VPN Defaults**: The project must not ship preconfigured paid VPN endpoints, affiliate defaults, or “one-click” commercial tunnel integrations.
2. **Explicitness Over Stealth**: Any tunneling, proxying, or traffic forwarding beyond normal routing must be explicit, operator-configured, and reversible. No transparent interception.
3. **Data Minimization**: Collect the minimum telemetry needed for rollback safety and troubleshooting, and treat diagnostics artifacts (logs, packet captures) as sensitive.

## Audit Trail (Cross-Team Review)
- 2026-06-07: Tightened ECH language to prevent claims of router-side “ECH enablement” via manipulation; clarified that any tunneling is explicit and operator-configured.
- 2026-06-07: Reframed profiles to avoid implicit proxy/VPN defaults; removed references that implied bundled obfuscated VPN implementations; added explicit “no commercial VPN defaults” constraint.
- 2026-06-07: Updated netcheck/transaction model to avoid hard-coded single canary targets to reduce privacy leakage and brittle validation.

## 6. Abuse Resistance & Compliance
Blueshoes is built to improve lawful access and availability, not to enable profiteering or covert rerouting.

1. **No Bundled Commercial VPN Defaults**: The project must not ship preconfigured paid VPN endpoints, affiliate defaults, or “one-click” commercial tunnel integrations.
2. **Explicitness Over Stealth**: Any tunneling, proxying, or traffic forwarding beyond normal routing must be explicit, operator-configured, and reversible. No transparent interception.
3. **Data Minimization**: Collect the minimum telemetry needed for rollback safety and troubleshooting, and treat diagnostics artifacts (logs, packet captures) as sensitive.

## Audit Trail (Cross-Team Review)
- 2026-06-07: Tightened ECH language to prevent claims of router-side “ECH enablement” via manipulation; clarified that any tunneling is explicit and operator-configured.
- 2026-06-07: Reframed profiles to avoid implicit proxy/VPN defaults; removed references that implied bundled obfuscated VPN implementations; added explicit “no commercial VPN defaults” constraint.
- 2026-06-07: Updated netcheck/transaction model to avoid hard-coded single canary targets to reduce privacy leakage and brittle validation.
