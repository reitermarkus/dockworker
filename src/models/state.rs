#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct State {
  pub status: String,
  pub running: bool,
  pub paused: bool,
  pub restarting: bool,
  #[serde(rename = "OOMKilled")]
  pub oom_killed: bool,
  pub dead: bool,
  // I don't know whether PIDs can be negative here.  They're normally
  // positive, but sometimes negative PIDs are used in certain APIs.
  pub pid: i64,
  pub exit_code: i64,
  pub error: String,
  pub started_at: String,
  pub finished_at: String,
}
