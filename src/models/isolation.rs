use serde::{Serialize, Deserialize, ser::Serializer, de::Deserializer};

#[derive(Debug, Clone)]
pub enum Isolation {
  Default,
  Process,
  HyperV,
}

impl Default for Isolation {
  fn default() -> Self {
    Isolation::Default
  }
}

impl Serialize for Isolation {
  fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where S: Serializer
  {
    serializer.serialize_str(match *self {
      Isolation::Default => "default",
      Isolation::Process => "process",
      Isolation::HyperV => "hyperv",
    })
  }
}

impl<'de> Deserialize<'de> for Isolation {
  fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where D: Deserializer<'de>
  {
    String::deserialize(deserializer)
      .map(|s| {
        match s.as_str() {
          "process" => Isolation::Process,
          "hyperv" => Isolation::HyperV,
          _ => Isolation::Default,
        }
      })
  }
}
