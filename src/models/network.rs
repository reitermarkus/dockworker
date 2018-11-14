#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Network {
  pub aliases: Option<Vec<String>>,
  #[serde(rename = "EndpointID")]
  pub endpoint_id: String,
  pub gateway: String,
  #[serde(rename = "GlobalIPv6Address")]
  pub global_ipv6_address: String,
  #[serde(rename = "GlobalIPv6PrefixLen")]
  pub global_ipv6_prefix_len: u32,
  //pub IPAMConfig: ,
  #[serde(rename = "IPAddress")]
  pub ip_address: String,
  #[serde(rename = "IPPrefixLen")]
  pub ip_prefix_len: u32,
  #[serde(rename = "IPv6Gateway")]
  pub ipv6_gateway: String,
  //pub Links:
  pub mac_address: String,
  #[serde(rename = "NetworkID")]
  pub network_id: String,
}
