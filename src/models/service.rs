use models::ObjectVersion;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Service {
  #[serde(rename = "ID")]
  pub id: String,
  pub version: ObjectVersion,
  pub created_at: String,
  pub updated_at: String,
  // pub spec: ServiceSpec,
  // pub endpoint: Endpoint,
}
