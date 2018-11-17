extern crate dockworker;
extern crate env_logger;

use dockworker::Docker;
use std::path::Path;

fn main() {
    env_logger::init();
    let docker = Docker::from_env().unwrap();
    let id = docker
        .image_load(false, Path::new("image.tar"))
        .expect("prepare a tar-archive like: $docker save busybox > image.tar");
    println!("loaded: {}", id);
}
