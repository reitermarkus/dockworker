extern crate dockworker;

use dockworker::{Docker, models::ContainerFilters};

fn main() {
    let docker = Docker::from_env().unwrap();
    let filter = ContainerFilters::new();
    if let Some(container) = docker.container_list(None, None, None, filter).unwrap().get(0) {
        for stats in docker.container_stats(container).unwrap() {
            println!("{:#?}", stats);
        }
    }
}
