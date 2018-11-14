use std::collections::HashMap as Map;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Driver {
  pub name: String,
  pub options: Map<String, String>,
}
