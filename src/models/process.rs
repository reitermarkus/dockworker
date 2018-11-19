use std::collections::BTreeMap;

use models::Top;

#[derive(Debug)]
pub struct Process {
  pub user: Option<String>,
  pub pid: Option<String>,
  pub cpu: Option<String>,
  pub memory: Option<String>,
  pub vsz: Option<String>,
  pub rss: Option<String>,
  pub tty: Option<String>,
  pub stat: Option<String>,
  pub start: Option<String>,
  pub time: Option<String>,
  pub command: Option<String>,
}

impl From<Top> for Vec<Process> {
  fn from(top: Top) -> Self {
    top.processes.iter()
      .map(|process| {

        let map: BTreeMap<&str, String> = top.titles.iter().map(|s| s.as_str())
                                                    .zip(process.iter().map(Clone::clone))
                                                    .collect();

        Process {
          user:    map.get("UID").or(map.get("USER")).cloned(),
          pid:     map.get("PID").cloned(),
          cpu:     map.get("%CPU").cloned(),
          memory:  map.get("%MEM").cloned(),
          vsz:     map.get("VSZ").cloned(),
          rss:     map.get("RSS").cloned(),
          tty:     map.get("TTY").cloned(),
          stat:    map.get("STAT").cloned(),
          start:   map.get("START").or(map.get("STIME")).cloned(),
          time:    map.get("TIME").cloned(),
          command: map.get("CMD").or(map.get("COMMAND")).cloned(),
        }
      })
      .collect()
  }
}
