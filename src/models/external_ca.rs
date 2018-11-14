use std::collections::HashMap as Map;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ExternalCA {
  pub protocol: String,
  #[serde(rename = "URL")]
  pub url: String,
  pub options: Map<String, String>,
  #[serde(rename = "CACert")]
  pub ca_cert: String,
}
