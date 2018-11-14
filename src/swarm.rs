use std::collections::HashMap as Map;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Orchestration {
  pub task_history_retention_limit: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Raft {
  pub snapshot_interval: u64,
  pub keep_old_snapshots: u64,
  pub log_entries_for_slow_followers: u64,
  pub election_tick: u64,
  pub heartbeat_tick: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Dispatcher {
  pub heartbeat_period: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ExternalCA {
  pub protocol: String,
  #[serde(rename = "URL")]
  pub url: String,
  pub options: Map<String, String>,
  #[serde(rename = "CACert")]
  pub ca_cert: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct CAConfig {
  pub node_cert_expiry: i64,
  #[serde(rename = "ExternalCAs", default = "Vec::default")]
  pub external_cas: Vec<ExternalCA>,
  #[serde(rename = "SigningCACert")]
  pub signing_ca_cert: Option<String>,
  #[serde(rename = "SigningCAKey")]
  pub signing_ca_key: Option<String>,
  pub force_rotate: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct EncryptionConfig {
  pub auto_lock_managers: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct LogDriver {
  pub name: String,
  pub options: Map<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct TaskDefaults {
  pub log_driver: Option<LogDriver>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Spec {
  pub labels: Map<String, String>,
  pub orchestration: Option<Orchestration>,
  pub raft: Raft,
  pub dispatcher: Option<Dispatcher>,
  #[serde(rename = "CAConfig")]
  pub ca_config: Option<CAConfig>,
  pub encryption_config: EncryptionConfig,
  pub task_defaults: TaskDefaults,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct TLSInfo {
  pub trust_root: String,
  pub cert_issuer_subject: String,
  pub cert_issuer_public_key: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct JoinTokens {
  pub worker: String,
  pub manager: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Swarm {
  #[serde(rename = "ID")]
  pub id: String,
  pub created_at: String,
  pub updated_at: String,
  pub spec: Spec,
  #[serde(rename = "TLSInfo")]
  pub tls_info: TLSInfo,
  pub root_rotation_in_progress: bool,
  pub join_tokens: JoinTokens,
}
