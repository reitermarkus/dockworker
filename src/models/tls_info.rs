#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct TLSInfo {
  pub trust_root: String,
  pub cert_issuer_subject: String,
  pub cert_issuer_public_key: String,
}
