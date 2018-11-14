#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct CreateContainerResponse {
  pub id: String,
  pub warnings: Option<Vec<String>>,
}
