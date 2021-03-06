extern crate dockworker;

use dockworker::{Docker, models::ContainerFilters};

fn main() {
    let docker = Docker::from_env().unwrap();
    let filter = ContainerFilters::new();
    let containers = docker.container_list(None, None, None, filter).unwrap();

    containers.iter().for_each(|c| {
        println!("{:?}", c);
    });
}
