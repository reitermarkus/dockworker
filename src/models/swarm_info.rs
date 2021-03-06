use serde_aux::prelude::*;

use models::{Cluster, RemoteManager};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct SwarmInfo {
  #[serde(rename = "NodeID")]
  pub node_id: String,
  pub node_addr: String,
  pub local_node_state: String,
  pub control_available: bool,
  pub error: String,
  #[serde(default, deserialize_with = "deserialize_default_from_null")]
  pub remote_managers: Vec<RemoteManager>,
  #[serde(default, deserialize_with = "deserialize_default_from_null")]
  pub nodes: u64,
  #[serde(default, deserialize_with = "deserialize_default_from_null")]
  pub managers: u64,
  pub cluster: Option<Cluster>,
}
