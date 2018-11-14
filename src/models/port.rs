#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Port {
  #[serde(rename = "IP")]
  pub ip: Option<String>,
  pub private_port: u64,
  pub public_port: Option<u64>,
  #[serde(rename = "Type")]
  pub port_type: String, // Renamed because `type` is a keyword.
}
