#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct UpdateConfig {
  pub parallelism: i64,
  pub delay: i64,
  pub failure_action: String,
  pub monitor: i64,
  pub max_failure_ratio: u64,
  pub order: String,
}
