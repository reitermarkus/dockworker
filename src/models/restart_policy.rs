#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct RestartPolicy {
  name: String,
  maximum_retry_count: u16,
}

impl Default for RestartPolicy {
  fn default() -> Self {
    Self {
      name: "no".to_owned(),
      maximum_retry_count: 0,
    }
  }
}
