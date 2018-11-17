use dockworker::Docker;

pub mod file;
pub mod memory_stream;

pub fn pull_image(docker: &Docker, name: &str, tag: &str) {
  assert!(
    docker.image_create(name, tag)
          .map(|sts| sts.for_each(|st| println!("{:?}", st)))
          .is_ok()
  );
}

pub fn with_image<F>(docker: &Docker, name: &str, tag: &str, f: F)
where
    F: Fn(&str, &str),
{
  pull_image(&docker, name, tag);
  f(name, tag);
  assert!(
    docker.image_delete(&format!("{}:{}", name, tag), None, None)
          .is_ok()
  );
}
