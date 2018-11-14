use std::path::PathBuf;

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct DeviceMapping {
  path_on_host: PathBuf,
  path_in_container: PathBuf,
  /// combination of r,w,m
  cgroup_permissions: String,
}

impl DeviceMapping {
  pub fn new(
    path_on_host: PathBuf,
    path_in_container: PathBuf,
    cgroup_permissions: String,
  ) -> Self {
    Self {
      path_on_host: path_on_host,
      path_in_container: path_in_container,
      cgroup_permissions: cgroup_permissions,
    }
  }
}
