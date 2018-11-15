extern crate dockworker;

use dockworker::Docker;

fn main() {
    let docker = Docker::from_env().unwrap();
    println!("{:?}", docker.secret_create("my_secret", "my_secret_ps").unwrap());
    println!("{:?}", docker.secrets().unwrap());
}
