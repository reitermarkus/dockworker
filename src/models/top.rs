#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Top {
  pub titles: Vec<String>,
  pub processes: Vec<Vec<String>>,
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn deserialize() {
    serde_json::from_str::<Top>(&r#"
      {
        "Processes": [["4586", "999", "rust"]],
        "Titles": ["PID", "USER", "COMMAND"]
      }
    "#).unwrap();
  }
}
