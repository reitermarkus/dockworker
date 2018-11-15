use std::collections::HashMap as Map;
use models::Driver;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct SecretSpec {
  pub name: String,
  pub labels: Map<String, String>,
  #[serde(default)]
  pub driver: Option<Driver>,
}