extern crate dockworker;

use dockworker::Docker;

fn main() {
  let docker = Docker::from_env().unwrap();

  let my_secret_name = "my_secret";

  let secret = docker.secret_create(my_secret_name, "my_secret_ps").unwrap();
  let secret_id = secret.id;

  println!("{}: {:?}", my_secret_name, docker.secret_inspect(&secret_id).unwrap());
  println!("{:?}", docker.secret_list().unwrap());
}
