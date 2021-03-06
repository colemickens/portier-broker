use url::Url;
use serde::de::{Deserialize, Deserializer, Error as DeserializeError};
use serde::ser::{Serialize, Serializer};


/// Unit struct which can be used to serialize a Url.
///
/// `#[serde(with = "UrlDef")]`
pub struct UrlDef;
impl UrlDef {
    pub fn deserialize<'de, D>(deserializer: D) -> Result<Url, D::Error>
            where D: Deserializer<'de> {
        <String>::deserialize(deserializer).and_then(|s| {
            s.parse().map_err(|e| DeserializeError::custom(format!("invalid URL: {}", e)))
        })
    }

    pub fn serialize<S>(url: &Url, serializer: S) -> Result<S::Ok, S::Error>
            where S: Serializer {
        url.as_str().serialize(serializer)
    }
}
