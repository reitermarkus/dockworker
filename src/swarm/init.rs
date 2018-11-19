use {Docker};
use error::Result;
use models::SwarmSpec;

pub struct Builder<'a> {
  docker: &'a Docker,
  advertise_addr: Option<&'a str>,
  listen_addr: Option<&'a str>,
  force_new_cluster: Option<bool>,
  spec: Option<&'a SwarmSpec>,
}

impl<'a> Builder<'a> {
  pub fn new(docker: &'a Docker) -> Self {
    Self {
      docker: docker,
      advertise_addr: None,
      listen_addr: None,
      force_new_cluster: None,
      spec: None,
    }
  }

  pub fn advertise_addr(&'a mut self, advertise_addr: &'a str) -> &'a mut Self {
    self.advertise_addr = Some(advertise_addr);
    self
  }

  pub fn listen_addr(&'a mut self, listen_addr: &'a str) -> &'a mut Self {
    self.listen_addr = Some(listen_addr);
    self
  }

  pub fn force_new_cluster(&'a mut self, force_new_cluster: bool) -> &'a mut Self {
    self.force_new_cluster = Some(force_new_cluster);
    self
  }

  pub fn spec(&'a mut self, spec: &'a SwarmSpec) -> &'a mut Self {
    self.spec = Some(spec);
    self
  }

  pub fn finish(&self) -> Result<String> {
    self.docker.swarm_init(self.advertise_addr, self.listen_addr, self.force_new_cluster, self.spec)
  }
}
