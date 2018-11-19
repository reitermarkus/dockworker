#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "State", rename_all = "snake_case")]
pub enum UpdateStatus {
  #[serde(rename_all = "PascalCase")]
  Updating { started_at: String, message: String },
  #[serde(rename_all = "PascalCase")]
  Paused { started_at: String, message: String },
  #[serde(rename_all = "PascalCase")]
  Completed { started_at: String, completed_at: String, message: String },
  #[serde(rename_all = "PascalCase")]
  RollbackStarted { started_at: String, message: String },
  #[serde(rename_all = "PascalCase")]
  RollbackPaused { started_at: String, message: String },
  #[serde(rename_all = "PascalCase")]
  RollbackCompleted { started_at: String, completed_at: String, message: String },
}
