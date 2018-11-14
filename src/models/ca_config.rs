use models::ExternalCA;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct CAConfig {
  pub node_cert_expiry: i64,
  #[serde(rename = "ExternalCAs", default)]
  pub external_cas: Vec<ExternalCA>,
  #[serde(rename = "SigningCACert")]
  pub signing_ca_cert: Option<String>,
  #[serde(rename = "SigningCAKey")]
  pub signing_ca_key: Option<String>,
  pub force_rotate: Option<u64>,
}
