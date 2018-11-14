use std::{fmt, result};

use serde_helpers::null_to_default;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Image {
    pub id: String,
    pub parent_id: String,
    #[serde(default, deserialize_with = "null_to_default")]
    pub repo_tags: Vec<String>,
    #[serde(default, deserialize_with = "null_to_default")]
    pub repo_digests: Vec<String>,
    pub created: u64,
    pub size: i64,
    #[serde(default)]
    pub shared_size: i64,
    pub virtual_size: i64,
    #[serde(default)]
    pub containers: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImageStatus {
    pub status: Option<String>,
    pub error: Option<String>,
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct ImageId {
    id: String,
}

impl From<String> for ImageId {
    fn from(id: String) -> Self {
        Self { id }
    }
}

impl fmt::Display for ImageId {
    fn fmt(&self, f: &mut fmt::Formatter) -> result::Result<(), fmt::Error> {
        self.id.fmt(f)
    }
}

impl ImageId {
    pub fn new<S: Into<String>>(id: S) -> Self {
        Self { id: id.into() }
    }

    pub fn id(&self) -> &str {
        &self.id
    }
}
