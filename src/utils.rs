use std::str::FromStr;
use serde::{Deserialize, Deserializer};

pub fn string_to_u64<'de, D>(deserializer: D) -> Result<u64, D::Error>
    where D: Deserializer<'de> {
    let s = String::deserialize(deserializer)?;
    
    u64::from_str(&s).map_err(serde::de::Error::custom)
}