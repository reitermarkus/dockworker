extern crate failure;
use failure::Error;

extern crate dockworker;
use dockworker::Docker;

fn main() -> Result<(), Error> {
  let docker = Docker::from_env()?;

  println!("{:#?}", docker.system_info()?.swarm.local_node_state);

  let swarm_id = docker.swarm_init(None, None, None, None)?;
  println!("{:#?}", swarm_id);

  let swarm_info = docker.swarm_inspect()?;
  println!("{:#?}", swarm_info);

  docker.swarm_leave(true)?;

  Ok(())
}
