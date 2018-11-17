#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct SystemVersionResponsePlatform {
  pub name: String,
}
