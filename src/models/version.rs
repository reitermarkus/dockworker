use serde_helpers::null_to_default;

use models::Component;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Version {
  #[serde(default, deserialize_with = "null_to_default")]
  pub components: Vec<Component>,
  pub version: String,
  pub api_version: String,
  #[serde(rename = "MinAPIVersion")]
  pub min_api_version: String,
  pub git_commit: String,
  pub go_version: String,
  pub os: String,
  pub arch: String,
  pub kernel_version: String,
  pub experimental: Option<bool>,
  pub build_time: Option<String>,
}
