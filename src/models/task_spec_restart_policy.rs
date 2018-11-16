#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct TaskSpecRestartPolicy {
  pub condition: String,
  pub delay: i64,
  pub max_attempts: i64,
  pub window: i64,
}
