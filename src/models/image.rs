use serde_aux::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Image {
  pub id: String,
  pub parent_id: String,
  #[serde(default, deserialize_with = "deserialize_default_from_null")]
  pub repo_tags: Vec<String>,
  #[serde(default, deserialize_with = "deserialize_default_from_null")]
  pub repo_digests: Vec<String>,
  pub created: u64,
  pub size: i64,
  #[serde(default)]
  pub shared_size: i64,
  pub virtual_size: i64,
  #[serde(default)]
  pub containers: i64,
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn deserialize_list() {
    let images = serde_json::from_str::<Vec<Image>>(&r#"
      [
        {
          "Created": 1428533761,
          "Id": "533da4fa223bfbca0f56f65724bb7a4aae7a1acd6afa2309f370463eaf9c34a4",
          "ParentId": "84ac0b87e42afe881d36f03dea817f46893f9443f9fc10b64ec279737384df12",
          "RepoTags": ["ghmlee/rust:nightly"],
          "Size": 0,
          "VirtualSize": 806688288
        },
        {
          "Created": 1371157430,
          "Id": "511136ea3c5a64f264b78b5433614aec563103b4d4702f3ba7d4d2698e22c158",
          "ParentId": "",
          "RepoTags": [],
          "Size": 0,
          "VirtualSize": 0
        },
        {
          "Created": 1371157430,
          "Id": "511136ea3c5a64f264b78b5433614aec563103b4d4702f3ba7d4d2698e22c158",
          "ParentId": "",
          "RepoTags": null,
          "Size": 0,
          "VirtualSize": 0
        }
      ]
    "#).unwrap();
    assert_eq!(images.len(), 3);
  }
}
