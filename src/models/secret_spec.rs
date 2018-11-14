use std::collections::HashMap as Map;
use models::Driver;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct SecretSpec {
  pub name: String,
  pub labels: Map<String, String>,
  pub data: String,
  pub driver: Driver,
  pub templating: Driver,
}
