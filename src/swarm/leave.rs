use {Docker};
use error::Result;

pub struct Builder<'a> {
  docker: &'a Docker,
  force: bool,
}

impl<'a> Builder<'a> {
  pub fn new(docker: &'a Docker) -> Self {
    Self { docker: docker, force: false, }
  }

  pub fn force(&'a mut self) -> &'a mut Self {
    self.force = true;
    self
  }

  pub fn finish(&self) -> Result<()> {
    self.docker.swarm_leave(self.force)
  }
}
