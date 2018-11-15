use serde_aux::prelude::*;

use models::RemovedImage;

#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct PrunedImages {
  #[serde(deserialize_with = "deserialize_default_from_null")]
  images_deleted: Vec<RemovedImage>,
  space_reclaimed: i64,
}
