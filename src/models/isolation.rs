use serde::{Serialize, Deserialize, ser::Serializer, de::{self, Deserializer}};

#[derive(Debug, PartialEq, Clone)]
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
      .and_then(|s| {
        match s.as_str() {
          "default" => Ok(Isolation::Default),
          "process" => Ok(Isolation::Process),
          "hyperv" => Ok(Isolation::HyperV),
          unknown => Err(de::Error::unknown_variant(&unknown, &["default", "process", "hyperv"])),
        }
      })
  }
}

#[cfg(all(test, unix))]
mod tests {
  use super::*;

  #[test]
  fn serialize_default() {
    assert_eq!(serde_json::to_string(&Isolation::Default).unwrap(), r#""default""#);
  }

  #[test]
  fn serialize_process() {
    assert_eq!(serde_json::to_string(&Isolation::Process).unwrap(), r#""process""#);
  }

  #[test]
  fn serialize_hyperv() {
    assert_eq!(serde_json::to_string(&Isolation::HyperV).unwrap(), r#""hyperv""#);
  }

  #[test]
  fn deserialize_empty() {
    assert!(serde_json::from_str::<Isolation>(r#""unknown""#).is_err());
  }

  #[test]
  fn deserialize_default() {
    assert_eq!(serde_json::from_str::<Isolation>(r#""default""#).unwrap(), Isolation::Default);
  }

  #[test]
  fn deserialize_process() {
    assert_eq!(serde_json::from_str::<Isolation>(r#""process""#).unwrap(), Isolation::Process);
  }

  #[test]
  fn deserialize_hyperv() {
    assert_eq!(serde_json::from_str::<Isolation>(r#""hyperv""#).unwrap(), Isolation::HyperV);
  }
}
