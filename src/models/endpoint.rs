use serde_aux::prelude::*;

use models::{EndpointSpec, EndpointPortConfig, VirtualIP};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Endpoint {
  pub spec: EndpointSpec,
  #[serde(default, deserialize_with = "deserialize_default_from_null")]
  pub ports: Vec<EndpointPortConfig>,
  #[serde(rename = "VirtualIPs")]
  pub virtual_ips: Vec<VirtualIP>,
}
