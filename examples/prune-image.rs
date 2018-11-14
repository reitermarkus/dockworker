extern crate dockworker;

use dockworker::{Docker, models::ContainerFilters};

fn main() {
    let docker = Docker::from_env().unwrap();

    let prunedt = docker.prune_image(true).unwrap();
    println!("pruned(true): {:?}", prunedt);

    let prunedf = docker.prune_image(false).unwrap();
    println!("pruned(false): {:?}", prunedf);

    let containers = docker
        .list_containers(Some(true), None, None, ContainerFilters::new())
        .unwrap();

    containers.iter().for_each(|c| {
        println!("image: {}", c.image);
    });
}
