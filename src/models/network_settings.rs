use std::collections::HashMap as Map;

use models::{Network, PortMapping};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct NetworkSettings {
  pub bridge: String,
  #[serde(rename = "EndpointID")]
  pub endpoint_id: String,
  pub gateway: String,
  #[serde(rename = "GlobalIPv6Address")]
  pub global_ipv6_address: String,
  #[serde(rename = "GlobalIPv6PrefixLen")]
  pub global_ipv6_prefix_len: u32,
  pub hairpin_mode: bool,
  #[serde(rename = "IPAddress")]
  pub ip_address: String,
  #[serde(rename = "IPPrefixLen")]
  pub ip_prefix_len: u32,
  #[serde(rename = "IPv6Gateway")]
  pub ipv6_gateway: String,
  #[serde(rename = "LinkLocalIPv6Address")]
  pub link_local_ipv6_address: String,
  #[serde(rename = "LinkLocalIPv6PrefixLen")]
  pub link_local_ipv6_prefix_len: u32,
  pub mac_address: String,
  pub networks: Map<String, Network>,
  pub ports: Option<Map<String, Option<Vec<PortMapping>>>>,
  #[serde(rename = "SandboxID")]
  pub sandbox_id: String,
  pub sandbox_key: String,
  // These two are null in the current output.
  //pub SecondaryIPAddresses: ,
  //pub SecondaryIPv6Addresses: ,
}
