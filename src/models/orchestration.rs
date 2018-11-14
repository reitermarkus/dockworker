#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Orchestration {
  pub task_history_retention_limit: i64,
}
