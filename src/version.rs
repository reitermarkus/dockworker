#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Version {
    pub version: String,
    pub api_version: String,
    #[serde(default, rename = "MinAPIVersion")]
    pub min_api_version: String,
    pub git_commit: String,
    pub go_version: String,
    pub os: String,
    pub arch: String,
    pub kernel_version: String,
    pub experimental: Option<bool>,
    pub build_time: Option<String>,
}
