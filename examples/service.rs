extern crate failure;
use failure::Error;

extern crate dockworker;
use dockworker::Docker;

fn main() -> Result<(), Error> {
  let docker = Docker::from_env()?;

  let service_info = docker.list_services(None, None, None, None)?;
  println!("{:#?}", service_info);

  Ok(())
}
