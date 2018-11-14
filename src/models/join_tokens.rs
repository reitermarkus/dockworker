#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct JoinTokens {
  pub worker: String,
  pub manager: String,
}
