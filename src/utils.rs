use chrono::{DateTime, Utc};
use serde::Serializer;

pub(crate) trait ToString {
    fn to_string(self) -> String;
}

impl<T: Into<&'static str>> ToString for T {
    fn to_string(self) -> String {
        let s: &'static str = self.into();
        s.into()
    }
}

/// Convert [u8] to Hex string
///
/// Example:
///
/// ```rust
/// assert_eq!(encode_hex(&[0, 15, 16, 255]), "000f10ff");
/// ```
pub(crate) fn encode_hex(input: &impl AsRef<[u8]>) -> String {
    input
        .as_ref()
        .iter()
        .map(|b| format!("{:0>2x}", b))
        .collect()
}

/// https://docs.rs/serde/latest/serde/ser/trait.Serializer.html#tymethod.serialize_none
pub(crate) fn none_to_empty_string<S>(
    input: &Option<String>,
    serializer: S,
) -> std::result::Result<S::Ok, S::Error>
where
    S: Serializer,
{
    match input {
        Some(ref s) => serializer.serialize_some(s),
        None => serializer.serialize_some(""),
    }
}

pub(crate) fn datetime_to_timestamp_string<S>(
    input: &DateTime<Utc>,
    serializer: S,
) -> std::result::Result<S::Ok, S::Error>
where
    S: Serializer,
{
    serializer.serialize_str(input.timestamp().to_string().as_str())
}
