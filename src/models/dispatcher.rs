#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Dispatcher {
  pub heartbeat_period: u64,
}
