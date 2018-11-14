#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Commit {
  #[serde(rename = "ID")]
  pub id: String,
  pub expected: String,
}
