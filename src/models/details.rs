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
