extern crate dockworker;

use dockworker::{models::ContainerCreateOptions, Docker};

fn main() {
    let docker = Docker::from_env().unwrap();
    let mut create = ContainerCreateOptions::new("hello-world:linux");
    create.tty(true);
    let container = docker.container_create(Some("testing"), &create).unwrap();
    docker.container_start(&container.id).unwrap();
}
