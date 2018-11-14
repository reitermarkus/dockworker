use std::error::Error;

extern crate dockworker;
use dockworker::Docker;

fn main() -> Result<(), Box<Error>> {
  let docker = Docker::from_env()?;

  println!("{:#?}", docker.system_info()?.swarm.local_node_state);

  let swarm_id = docker.init_swarm(None, None, None, None)?;
  println!("{:#?}", swarm_id);

  let swarm_info = docker.inspect_swarm()?;
  println!("{:#?}", swarm_info);

  docker.leave_swarm(Some(true))?;

  Ok(())
}
