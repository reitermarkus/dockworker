#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct RemoteManager {
  #[serde(rename = "NodeID")]
  pub node_id: String,
  pub addr: String,
}
