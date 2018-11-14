use models::{ClusterVersion, SwarmSpec, TLSInfo};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Cluster {
  #[serde(rename = "ID")]
  pub id: String,
  pub version: ClusterVersion,
  pub created_at: String,
  pub updated_at: String,
  pub spec: SwarmSpec,
  #[serde(rename = "TLSInfo")]
  pub tls_info: TLSInfo,
  pub root_rotation_in_progress: bool,
}
