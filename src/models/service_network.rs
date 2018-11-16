#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ServiceNetwork {
  pub target: String,
  pub aliases: Vec<String>,
}
