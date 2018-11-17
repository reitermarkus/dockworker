extern crate dockworker;

use dockworker::{Docker, models::ContainerFilters};

fn main() {
    let docker = Docker::from_env().unwrap();

    let prunedt = docker.image_prune(true).unwrap();
    println!("pruned(true): {:?}", prunedt);

    let prunedf = docker.image_prune(false).unwrap();
    println!("pruned(false): {:?}", prunedf);

    let containers = docker
        .container_list(Some(true), None, None, ContainerFilters::new())
        .unwrap();

    containers.iter().for_each(|c| {
        println!("image: {}", c.image);
    });
}
