use serde::{Serialize, Deserialize, ser::Serializer, de::{self, Deserializer}};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "Type", rename_all = "lowercase")]
pub enum Mount {
  #[serde(rename_all = "PascalCase")]
  Bind   {
    target: String,
    source: String,
    read_only: Option<bool>,
    consistency: Option<Consistency>,
    // #[serde(rename = "BindOptions")]
    // options: Option<BindOptions>
  },
  #[serde(rename_all = "PascalCase")]
  Volume {
    target: String,
    source: String,
    read_only: Option<bool>,
    consistency: Option<Consistency>,
    // #[serde(rename = "VolumeOptions")]
    // options: Option<VolumeOptions>
  },
  #[serde(rename_all = "PascalCase")]
  Tmpfs  {
    target: String,
    source: String,
    read_only: Option<bool>,
    consistency: Option<Consistency>,
    // #[serde(rename = "TmpfsOptions")]
    // options: Option<TmpfsOptions>
  },
}

#[derive(Debug, Clone)]
pub enum Consistency {
  Default,
  Consistent,
  Cached,
  Delegated,
}

impl Serialize for Consistency {
  fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where S: Serializer
  {
    serializer.serialize_str(match *self {
      Consistency::Default => "default",
      Consistency::Consistent => "consistent",
      Consistency::Cached => "cached",
      Consistency::Delegated => "delegated",
    })
  }
}

impl<'de> Deserialize<'de> for Consistency {
  fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where D: Deserializer<'de>
  {
    String::deserialize(deserializer)
      .and_then(|s| {
        match s.as_str() {
          "default" => Ok(Consistency::Default),
          "consistent" => Ok(Consistency::Consistent),
          "cached" => Ok(Consistency::Cached),
          "delegated" => Ok(Consistency::Delegated),
          unknown => Err(de::Error::unknown_variant(&unknown, &["default", "consistent", "cached", "delegated"])),
        }
      })
  }
}
