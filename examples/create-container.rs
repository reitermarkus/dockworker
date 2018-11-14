extern crate dockworker;

use dockworker::{models::ContainerCreateOptions, Docker};

fn main() {
    let docker = Docker::from_env().unwrap();
    let create = ContainerCreateOptions::new("hello-world:linux");
    let container = docker.create_container(Some("testing"), &create).unwrap();
    println!("{:?}", container)
}
