use serde::{Deserialize, Deserializer};

pub fn deserialize_empty_string_as_none<'de, D>(deserializer: D) -> Result<Option<String>, D::Error>
where
    D: Deserializer<'de>,
{
    let value: Option<String> = Option::deserialize(deserializer)?;

    return match value {
        Some(v) if !v.is_empty() => Ok(Some(v)),
        _ => Ok(None),
    };
}
