use serde_helpers::null_to_default;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Details {
  pub api_version: String,
  pub arch: String,
  pub build_time: String,
  pub experimental: String,
  pub git_commit: String,
  pub go_version: String,
  pub kernel_version: String,
  #[serde(rename = "MinAPIVersion")]
  pub min_api_version: String,
  pub os: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Component {
  pub name: String,
  pub version: String,
  pub details: Details,
}

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
