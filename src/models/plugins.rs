use serde_helpers::null_to_default;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Plugins {
  #[serde(default, deserialize_with = "null_to_default")]
  pub volume: Vec<String>,
  #[serde(default, deserialize_with = "null_to_default")]
  pub network: Vec<String>,
  #[serde(default, deserialize_with = "null_to_default")]
  pub authorization: Vec<String>,
  #[serde(default, deserialize_with = "null_to_default")]
  pub log: Vec<String>,
}
