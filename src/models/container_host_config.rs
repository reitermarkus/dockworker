use std::collections::HashMap as Map;

use models::{RestartPolicy, DeviceMapping};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ContainerHostConfig {
  binds: Vec<String>,
  tmpfs: Map<String, String>,
  links: Vec<String>,
  memory: u64,
  memory_swap: u64,
  memory_reservation: u64,
  kernel_memory: u64,
  cpu_percent: u64, // TODO: Not sure what this should actually be
  // it could either be 0 - 100, or it could be 100% per thread.
  cpu_shares: u64,           // TODO: I don't have a single clue what this even is
  cpu_period: u64,           // TODO: Still no clue
  cpu_quota: u64,            // TODO: Still no clue
  cpuset_cpus: String,       // TODO: Still no clue
  io_maximum_bandwidth: u64, // TODO: Still no clue
  io_maximum_ops: u64,       // TODO: Still no clue
  blkio_weight: u64,         // TODO: Still no clue
  // blkio_weight_device: not sure the type of this. the provided is [{}]
  // blkio_weight_read_bps: not sure the type of this. the provided is [{}],
  // blkio_weight_read_iops: not sure the type of this. the provided is [{}]
  // blkio_weight_write_bps: not sure the type of this. the provided is [{}]
  // blkio_weight_write_iops: not sure the type of this. the provided is [{}]
  memory_swappiness: i32, // TODO: Maybe this can be smaller?
  oom_kill_disable: bool,
  oom_score_adj: u16, // TODO: Maybe this can be smaller?
  pid_mode: String,
  pids_limit: i16,
  port_bindings: Map<String, Vec<Map<String, String>>>,
  publish_all_ports: bool,
  privileged: bool,
  readonly_rootfs: bool,
  dns: Vec<String>,
  dns_options: Vec<String>,
  dns_search: Vec<String>,
  // extra_hosts: Not sure the type of this. the provided is null
  auto_remove: bool,
  volumes_from: Vec<String>,
  cap_add: Vec<String>,
  cap_drop: Vec<String>,
  group_add: Vec<String>,
  restart_policy: RestartPolicy,
  network_mode: String,
  devices: Vec<DeviceMapping>,
  sysctls: Map<String, String>,
  // ulimits: TODO: Not sure the type of this. the provided is [{}]
  // log_config: TODO: not sure the type of this. the provided makes no sense
  // security_opt: TODO: Not sure the type of this. The provided is []
  // storage_opt: TODO: Not sure the type of this. the provided is {}
  cgroup_parent: String,
  volume_driver: String,
  shm_size: u64,
}

impl ContainerHostConfig {
  pub fn new() -> Self {
    Self {
      binds: Vec::new(),
      tmpfs: Map::new(),
      links: Vec::new(),
      memory: 0,
      memory_swap: 0,
      memory_reservation: 0,
      kernel_memory: 0,
      cpu_percent: 0,
      cpu_shares: 0,
      cpu_period: 0,
      cpu_quota: 0,
      cpuset_cpus: "".to_owned(),
      io_maximum_bandwidth: 0,
      io_maximum_ops: 0,
      blkio_weight: 0,
      memory_swappiness: -1,
      oom_kill_disable: false,
      oom_score_adj: 0,
      pid_mode: "".to_owned(),
      pids_limit: 0,
      port_bindings: Map::new(),
      publish_all_ports: false,
      privileged: false,
      readonly_rootfs: false,
      dns: Vec::new(),
      dns_options: Vec::new(),
      dns_search: Vec::new(),
      auto_remove: false,
      volumes_from: Vec::new(),
      cap_add: Vec::new(),
      cap_drop: Vec::new(),
      group_add: Vec::new(),
      restart_policy: RestartPolicy::default(),
      network_mode: "default".to_owned(),
      devices: Vec::new(),
      sysctls: Map::new(),
      cgroup_parent: "".to_owned(),
      volume_driver: "".to_owned(),
      /// 64MB
      shm_size: 64 * 1024 * 1024,
    }
  }

  pub fn binds(&mut self, bind: String) -> &mut Self {
    self.binds.push(bind);
    self
  }
  pub fn tmpfs(&mut self, path: &str, option: &str) -> &mut Self {
    self.tmpfs.insert(path.to_owned(), option.to_owned());
    self
  }
  pub fn links(&mut self, link: String) -> &mut Self {
    self.links.push(link);
    self
  }
  pub fn memory(&mut self, memory: u64) -> &mut Self {
    self.memory = memory;
    self
  }
  pub fn memory_swap(&mut self, memory_swap: u64) -> &mut Self {
    self.memory_swap = memory_swap;
    self
  }
  pub fn memory_reservation(&mut self, memory_reservation: u64) -> &mut Self {
    self.memory_reservation = memory_reservation;
    self
  }
  pub fn kernel_memory(&mut self, kernel_memory: u64) -> &mut Self {
    self.kernel_memory = kernel_memory;
    self
  }
  pub fn cpu_percent(&mut self, cpu_percent: u64) -> &mut Self {
    self.cpu_percent = cpu_percent;
    self
  }
  pub fn cpu_shares(&mut self, cpu_shares: u64) -> &mut Self {
    self.cpu_shares = cpu_shares;
    self
  }
  pub fn cpu_period(&mut self, cpu_period: u64) -> &mut Self {
    self.cpu_period = cpu_period;
    self
  }
  pub fn cpu_quota(&mut self, cpu_quota: u64) -> &mut Self {
    self.cpu_quota = cpu_quota;
    self
  }
  pub fn cpuset_cpus(&mut self, cpuset_cpus: String) -> &mut Self {
    self.cpuset_cpus = cpuset_cpus;
    self
  }
  pub fn io_maximum_bandwidth(&mut self, io_maximum_bandwidth: u64) -> &mut Self {
    self.io_maximum_bandwidth = io_maximum_bandwidth;
    self
  }
  pub fn io_maximum_ops(&mut self, io_maximum_ops: u64) -> &mut Self {
    self.io_maximum_ops = io_maximum_ops;
    self
  }
  pub fn blkio_weight(&mut self, blkio_weight: u64) -> &mut Self {
    self.blkio_weight = blkio_weight;
    self
  }
  pub fn memory_swappiness(&mut self, memory_swappiness: i32) -> &mut Self {
    self.memory_swappiness = memory_swappiness;
    self
  }
  pub fn oom_kill_disable(&mut self, oom_kill_disable: bool) -> &mut Self {
    self.oom_kill_disable = oom_kill_disable;
    self
  }
  pub fn oom_score_adj(&mut self, oom_score_adj: u16) -> &mut Self {
    self.oom_score_adj = oom_score_adj;
    self
  }
  pub fn pid_mode(&mut self, pid_mode: String) -> &mut Self {
    self.pid_mode = pid_mode;
    self
  }
  pub fn pids_limit(&mut self, pids_limit: i16) -> &mut Self {
    self.pids_limit = pids_limit;
    self
  }
  pub fn publish_all_ports(&mut self, publish_all_ports: bool) -> &mut Self {
    self.publish_all_ports = publish_all_ports;
    self
  }
  pub fn privileged(&mut self, privileged: bool) -> &mut Self {
    self.privileged = privileged;
    self
  }
  pub fn readonly_rootfs(&mut self, readonly_rootfs: bool) -> &mut Self {
    self.readonly_rootfs = readonly_rootfs;
    self
  }
  pub fn dns(&mut self, dns: String) -> &mut Self {
    self.dns.push(dns);
    self
  }
  pub fn dns_options(&mut self, dns_option: String) -> &mut Self {
    self.dns_options.push(dns_option);
    self
  }
  pub fn dns_search(&mut self, dns_search: String) -> &mut Self {
    self.dns_search.push(dns_search);
    self
  }
  pub fn auto_remove(&mut self, auto_remove: bool) -> &mut Self {
    self.auto_remove = auto_remove;
    self
  }
  pub fn volumes_from(&mut self, volumes_from: String) -> &mut Self {
    self.volumes_from.push(volumes_from);
    self
  }
  pub fn cap_add(&mut self, cap_add: String) -> &mut Self {
    self.cap_add.push(cap_add);
    self
  }
  pub fn cap_drop(&mut self, cap_drop: String) -> &mut Self {
    self.cap_drop.push(cap_drop);
    self
  }
  pub fn group_add(&mut self, group_add: String) -> &mut Self {
    self.group_add.push(group_add);
    self
  }
  pub fn restart_policy(&mut self, restart_policy: RestartPolicy) -> &mut Self {
    self.restart_policy = restart_policy;
    self
  }
  pub fn network_mode(&mut self, network_mode: String) -> &mut Self {
    self.network_mode = network_mode;
    self
  }
  pub fn devices(&mut self, device: DeviceMapping) -> &mut Self {
    self.devices.push(device);
    self
  }
  pub fn sysctls(&mut self, key: &str, value: &str) -> &mut Self {
    self.sysctls.insert(key.to_owned(), value.to_owned());
    self
  }
  pub fn cgroup_parent(&mut self, cgroup_parent: String) -> &mut Self {
    self.cgroup_parent = cgroup_parent;
    self
  }
  pub fn volume_driver(&mut self, volume_driver: String) -> &mut Self {
    self.volume_driver = volume_driver;
    self
  }
  pub fn shm_size(&mut self, shm_size: u64) -> &mut Self {
    self.shm_size = shm_size;
    self
  }
}
