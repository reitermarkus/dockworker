#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct EndpointPortConfig {
  #[serde(skip_serializing_if = "Option::is_none")]
  pub name: Option<String>,
  pub protocol: String,
  pub target_port: u64,
  pub published_port: u64,
  pub publish_mode: String,
}
