# Network Toolset Topology

**Definition:** The universal set of diagnostic tools $U_{tools}$ is partitioned into sets $E_{native}$, $W_{remote}$, and $F_{banned}$.

## $E_{native}$ (Router Core Set)
**Condition (Necessary):** Elements of $E_{native}$ must exist within the base FreeBSD compilation and incur zero marginal flash cost.
- $Discovery = \{\text{ip}, \text{ss}, \text{arp}\}$
- $Performance = \{\text{ping}, \text{conntrack}\}$
- $Routing = \{\text{iproute2}, \text{nft}\}$
- $Capture = \{\text{tcpdump}\}$
- $Validation = \{\text{curl}\}$

## $W_{remote}$ (Workbench Set)
**Condition (Necessary):** Elements of $W_{remote}$ require dependency trees exceeding $Flash_{E} \le 5\text{MB}$ and therefore execute exclusively in space $W$.
- $DeepAnalysis = \{\text{Wireshark}, \text{tshark}\}$
- $HeavyDiscovery = \{\text{nmap}, \text{mtr}\}$
- $AdvancedDNS = \{\text{kdig}, \text{drill}\}$
- $LoadTest = \{\text{iperf3}, \text{h2load}\}$

## $F_{banned}$ (Null Set)
**Condition (Necessary):** Elements of $F_{banned}$ violate Axiom 2 (TLS Integrity) or execute unbounded heuristic mapping.
- $F_{banned} = \{\text{Charles Proxy (Runtime)}, \text{Masscan (On-Router)}\}$
