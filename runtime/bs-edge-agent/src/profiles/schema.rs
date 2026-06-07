use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ProfileIntent {
    DnsPrivacy,
    EchPreserve,
    UserTunnel,
    SafeMtu,
    RecoverySafeMode,
    ObliviousDns,
    MasqueObfuscation,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RouteAction {
    pub target_cidr: String,
    pub via: Option<String>,
    pub interface: Option<String>,
    pub metric: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DnsAction {
    pub upstream_ips: Vec<String>,
    pub dot_hostname: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ProfileSchema {
    pub name: String,
    pub intent: ProfileIntent,
    pub description: String,
    pub routes: Option<Vec<RouteAction>>,
    pub dns: Option<DnsAction>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_profile_parsing() {
        let json_data = r#"{
            "name": "Cloudflare DNS over TLS",
            "intent": "DNS_PRIVACY",
            "description": "Route DNS requests over TLS to 1.1.1.1",
            "dns": {
                "upstream_ips": ["1.1.1.1", "1.0.0.1"],
                "dot_hostname": "cloudflare-dns.com"
            }
        }"#;

        let profile: Result<ProfileSchema, _> = serde_json::from_str(json_data);
        assert!(profile.is_ok());
        let p = profile.unwrap();
        assert_eq!(p.intent, ProfileIntent::DnsPrivacy);
        assert!(p.dns.is_some());
    }

    #[test]
    fn test_invalid_intent_rejected() {
        let json_data = r#"{
            "name": "Hacked Profile",
            "intent": "RUN_SHELL_SCRIPT",
            "description": "Bad",
            "routes": []
        }"#;

        let profile: Result<ProfileSchema, _> = serde_json::from_str(json_data);
        assert!(profile.is_err(), "Should reject invalid intents");
    }
}
