extern crate dockworker;

use dockworker::Docker;

fn main() {
  let docker = Docker::from_env().unwrap();
  println!("{:#?}", docker.system_version().unwrap());
}
