extern crate dockworker;

use dockworker::{Docker, models::ContainerFilters};

fn main() {
    let docker = Docker::from_env().unwrap();
    let filter = ContainerFilters::new();
    if let Some(container) = docker.list_containers(None, None, None, filter).unwrap().get(0) {
        for process in docker.processes(container).unwrap() {
            println!("{:#?}", process);
        }
    }
}
