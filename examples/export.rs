extern crate dockworker;

use dockworker::{Docker, container::ContainerFilters};
use std::fs::File;
use std::io;

fn main() {
    let docker = Docker::from_env().unwrap();
    let filter = ContainerFilters::new();
    let mut file = File::create("temp.tar").unwrap();
    if let Some(container) = docker.list_containers(None, None, None, filter).unwrap().get(0) {
        let mut res = docker.export_container(container).unwrap();
        io::copy(&mut res, &mut file).unwrap();
    }
}
