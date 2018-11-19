/// response of `/containers/{id}/changes`
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct FilesystemChange {
  pub path: String,
  pub kind: u8,
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn deserialize() {
    serde_json::from_str::<Vec<FilesystemChange>>(&r#"[{"Path":"/tmp","Kind":0}]"#).unwrap();
  }
}
