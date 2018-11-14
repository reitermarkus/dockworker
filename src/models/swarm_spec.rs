use std::collections::HashMap as Map;

use models::{CAConfig, Dispatcher, EncryptionConfig, Orchestration, Raft, TaskDefaults};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct SwarmSpec {
  pub labels: Map<String, String>,
  pub orchestration: Option<Orchestration>,
  pub raft: Raft,
  pub dispatcher: Option<Dispatcher>,
  #[serde(rename = "CAConfig")]
  pub ca_config: Option<CAConfig>,
  pub encryption_config: EncryptionConfig,
  pub task_defaults: TaskDefaults,
}
