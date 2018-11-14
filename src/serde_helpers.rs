use std::fmt;

use serde::de::{self, DeserializeOwned, Deserializer, Visitor};
use serde::Deserialize;

pub(crate) fn null_to_default<'de, D, T>(de: D) -> Result<T, D::Error>
where
  D: Deserializer<'de>,
  T: DeserializeOwned + Default,
{
  let actual: Option<T> = Option::deserialize(de)?;
  Ok(actual.unwrap_or_default())
}

struct NumToBoolVisitor;

impl<'de> Visitor<'de> for NumToBoolVisitor {
  type Value = bool;

  fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
    formatter.write_str("0 or 1 or true or false")
  }

  fn visit_bool<E>(self, value: bool) -> Result<Self::Value, E>
  where
    E: de::Error,
  {
    Ok(value)
  }

  fn visit_i64<E>(self, value: i64) -> Result<Self::Value, E>
  where
    E: de::Error,
  {
    Ok(value != 0)
  }

  fn visit_u64<E>(self, value: u64) -> Result<Self::Value, E>
  where
    E: de::Error,
  {
    Ok(value != 0)
  }
}

pub(crate) fn num_to_bool<'de, D>(deserializer: D) -> Result<bool, D::Error>
where
  D: Deserializer<'de>
{
  deserializer.deserialize_any(NumToBoolVisitor)
}
