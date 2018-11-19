mod init;

use Docker;

pub struct Swarm<'a> {
  docker: &'a Docker,
}

impl<'a> Swarm<'a> {
  pub(crate) fn new(docker: &'a Docker) -> Self {
    Self { docker: &docker }
  }

  pub fn init(&self) -> self::init::Builder<'a> {
    self::init::Builder::new(&self.docker)
  }
}
