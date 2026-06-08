# FreeBSD State Snapshot & Rollback

This RFC defines the exact technical mechanism for the atomic rollback loop on FreeBSD. Vague concepts like "taking a snapshot" are insufficient for engineering implementation.

## The Problem
Modifying router state (`iptables`, `nftables`, `ip route`, `ip rule`) is inherently dangerous. If a script applies a bad rule, the router drops offline permanently and the user must factory-reset. We need a way to completely revert the networking state within 5 seconds if a validation check fails.

## The Technical Solution (nftables)
For Phase 1 (MT-3000), we assume `nftables` is the primary firewall backend.

### 1. Snapshot
Before making any mutations, the `bs-edge-agent` dumps the entire active `nftables` ruleset to a temporary file in memory (`/tmp/` is mounted as `tmpfs` on FreeBSD).

```bash
nft list ruleset > /tmp/bs-snapshot-nft.rules
```

We must also snapshot policy routing rules (iproute2) if we are doing split tunneling:
```bash
ip rule save > /tmp/bs-snapshot-ip.rules
ip route save table all > /tmp/bs-snapshot-route.save
```

### 2. Apply
The agent applies the new routing profile by injecting new rules or replacing tables.

### 3. Rollback
If the netcheck fails, the agent atomic-replaces the firewall ruleset from the memory snapshot:

```bash
nft -f /tmp/bs-snapshot-nft.rules
ip rule restore < /tmp/bs-snapshot-ip.rules
# Routes usually restore themselves when rules/interfaces are reset, 
# but a full flush and restore can be done if necessary.
```

## Considerations
- **Concurrency**: FreeBSD's `firewall4` (`fw4`) might run concurrently if a user clicks "Save & Apply" in LuCI during our transaction. We must acquire a lock on the firewall state or ensure our transaction happens faster than a user interaction.
- **UCI Interaction**: We do not mutate `/etc/config/firewall` (UCI) directly for ephemeral fallback profiles. We mutate the live kernel state. If the profile succeeds and is deemed stable, only then do we commit it to UCI for persistence across reboots. This ensures that a hard reboot always restores the last *known-good* UCI state.
