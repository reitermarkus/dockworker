extern crate dockworker;
extern crate hyper;

use dockworker::{models::{ContainerCreateOptions, ContainerHostConfig}, Docker, container::AttachContainer};
use std::io::{BufRead, BufReader};

fn main() {
    let docker = Docker::from_env().unwrap();
    let mut host_config = ContainerHostConfig::new();
    host_config.auto_remove(true);
    let mut create = ContainerCreateOptions::new("hello-world:linux");
    create.host_config(host_config);

    let container = docker.container_create(Some("testing"), &create).unwrap();
    docker.container_start(&container.id).unwrap();
    let res = docker
        .container_attach(&container.id, None, true, true, false, true, false)
        .unwrap();
    let cont: AttachContainer = res.into();
    let mut line_reader = BufReader::new(cont.stdout);

    loop {
        let mut line = String::new();
        let size = line_reader.read_line(&mut line).unwrap();
        print!("{:4}: {}", size, line);
        if size == 0 {
            break;
        }
    }
    println!("");
}
