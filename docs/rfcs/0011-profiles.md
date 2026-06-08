# Static Bounded Profiles

To maintain determinism and stability, the `bs-edge-agent` does not write firewall rules dynamically on the fly. Instead, it selects from a set of pre-compiled, statically defined profiles based on observed network failures.

## Initial Profile Set

1. **DIRECT**: The baseline state. Standard FreeBSD routing with no obfuscation.
2. **DNS_PRIVACY**: Uses encrypted DNS upstreams (DoH/DoT) for router-side resolution only. It does not proxy or redirect arbitrary application-layer traffic.
3. **ECH_PRESERVE**: Preserves end-to-end TLS integrity and avoids behaviors that strip or downgrade ECH-related DNS records (HTTPS/SVCB). It does not attempt to inject ECH, and it does not intentionally break non-ECH sites.
4. **USER_TUNNEL (Optional, Explicit)**: If and only if the operator configures their own lawful egress endpoint, selected traffic may be routed through an explicit tunnel profile. Blueshoes strictly prohibits the inclusion of preconfigured commercial VPN services, affiliate defaults, or "one-click" paid tunnel integrations.
