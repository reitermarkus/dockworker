extern crate dockworker;

use dockworker::Docker;

fn main() {
    let docker = Docker::from_env().unwrap();
    docker.ping().unwrap();
}
