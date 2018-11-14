use std::collections::HashMap as Map;

use models::{HostConfig, Port};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Container {
  pub id: String,
  pub image: String,
  pub status: String,
  pub command: String,
  pub created: u64,
  pub names: Vec<String>,
  pub ports: Vec<Port>,
  pub size_rw: Option<u64>, // I guess it is optional on Mac.
  pub size_root_fs: Option<u64>,
  pub labels: Option<Map<String, String>>,
  pub host_config: HostConfig,
}

impl std::fmt::Display for Container {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::result::Result<(), std::fmt::Error> {
    write!(f, "{}", self.id)
  }
}
