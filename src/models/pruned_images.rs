use serde_helpers::null_to_default;

use models::RemovedImage;

#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct PrunedImages {
  #[serde(deserialize_with = "null_to_default")]
  images_deleted: Vec<RemovedImage>,
  space_reclaimed: i64,
}
