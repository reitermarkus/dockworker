use models::{JoinTokens, SwarmSpec, TLSInfo};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Swarm {
  #[serde(rename = "ID")]
  pub id: String,
  pub created_at: String,
  pub updated_at: String,
  pub spec: SwarmSpec,
  #[serde(rename = "TLSInfo")]
  pub tls_info: TLSInfo,
  pub root_rotation_in_progress: bool,
  pub join_tokens: JoinTokens,
}
