use models::Details;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Component {
  pub name: String,
  pub version: String,
  pub details: Details,
}
