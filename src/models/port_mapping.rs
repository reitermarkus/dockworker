#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct PortMapping {
  pub host_ip: String,
  pub host_port: String,
}
