extern crate dockworker;

use dockworker::Docker;

fn main() {
    let docker = Docker::from_env().unwrap();

    let name = "debian";
    let tag = "latest";
    println!("create an image {}:{} ...", name, tag);
    let stats = docker.create_image(name, tag).unwrap();
    for stat in stats {
        println!("{:?}", stat);
    }
}
