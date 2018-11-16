#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct VirtualIP {
  #[serde(rename = "NetworkID")]
  pub network_id: String,
  pub addr: String,
}
