extern crate dockworker;

use dockworker::Docker;

fn main() {
    let docker = Docker::from_env().unwrap();
    let images = docker.image_list(false).unwrap();

    for image in &images {
        println!(
            "{} -> Size: {}, Virtual Size: {}, Created: {}",
            image.id, image.size, image.virtual_size, image.created
        );
    }
}
