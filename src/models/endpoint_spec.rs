use serde_aux::prelude::*;

use models::EndpointPortConfig;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct EndpointSpec {
  pub mode: String,
  #[serde(default, deserialize_with = "deserialize_default_from_null")]
  pub ports: Vec<EndpointPortConfig>,
}
