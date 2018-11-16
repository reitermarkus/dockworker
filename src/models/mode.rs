#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum Mode {
  Replicated {
    #[serde(rename = "Replicas")]
    replicas: i64,
  },
  Global,
}
