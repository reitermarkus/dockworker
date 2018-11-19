use serde::{Serialize, Deserialize, ser::Serializer, de::{self, Deserializer}};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Mount {
  pub target: String,
  pub source: String,
  #[serde(rename = "Type")]
  pub mount_type: MountType,
  pub read_only: Option<bool>,
  pub consistency: Option<String>,
  // pub bind_options: Option<BindOptions>,
  // pub volume_options: Option<VolumeOptions>,
  // pub tmpfs_options: Option<TmpfsOptions>,
}

#[derive(Debug, Clone)]
pub enum MountType {
  Bind,
  Volume,
  Tmpfs,
}

impl Serialize for MountType {
  fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where S: Serializer
  {
    serializer.serialize_str(match *self {
      MountType::Bind => "bind",
      MountType::Volume => "volume",
      MountType::Tmpfs => "tmpfs",
    })
  }
}

impl<'de> Deserialize<'de> for MountType {
  fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where D: Deserializer<'de>
  {
    String::deserialize(deserializer)
      .and_then(|s| {
        match s.as_str() {
          "bind" => Ok(MountType::Bind),
          "volume" => Ok(MountType::Volume),
          "tmfps" => Ok(MountType::Tmpfs),
          unknown => Err(de::Error::unknown_variant(&unknown, &["bind", "volume", "tmfps"])),
        }
      })
  }
}
