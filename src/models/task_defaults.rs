use models::Driver;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct TaskDefaults {
  pub log_driver: Option<Driver>,
}
