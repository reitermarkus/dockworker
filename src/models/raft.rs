#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Raft {
  pub snapshot_interval: u64,
  pub keep_old_snapshots: u64,
  pub log_entries_for_slow_followers: u64,
  pub election_tick: u64,
  pub heartbeat_tick: u64,
}
