use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum NftFamily {
    Inet,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum NftTable {
    Filter,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum NftChain {
    Forward,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum TransportProtocol {
    Tcp,
    Udp,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum NftAction {
    Accept,
    Drop,
    Reject,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "action")]
pub enum NetworkCapability {
    AddRoute {
        target: String,
        via: String, // Interface name or IP string, validated at compile/plan time
    },
    FlushRouteCache,
    AddNftRule {
        family: NftFamily,
        table: NftTable,
        chain: NftChain,
        protocol: TransportProtocol,
        dport: u16,
        rule_action: NftAction,
    },
    SetMtu {
        interface: String,
        mtu: u32,
    },
    EstablishMasqueTunnel {
        endpoint: String,
        sni: String,
        psk: String,
    },
    CommitConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CapabilityGraph {
    pub network_caps: Vec<NetworkCapability>,
}

#[allow(dead_code)]
impl CapabilityGraph {
    pub fn new() -> Self {
        Self {
            network_caps: Vec::new(),
        }
    }

    pub fn push(&mut self, cap: NetworkCapability) {
        self.network_caps.push(cap);
    }

    pub fn len(&self) -> usize {
        self.network_caps.len()
    }

    pub fn is_empty(&self) -> bool {
        self.network_caps.is_empty()
    }
}
