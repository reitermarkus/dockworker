#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]
pub enum RemovedImage {
  Untagged(String),
  Deleted(String),
}
