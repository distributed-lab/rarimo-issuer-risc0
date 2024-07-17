use risc0_zkp::core::digest::Digest;

use serde::de::{self};
use serde::{ser::SerializeSeq, Deserialize, Deserializer, Serializer};

// Utility function to convert a hex string to a Vec<u8>
fn hex_to_bytes(hex: &str) -> Result<Vec<u8>, hex::FromHexError> {
    hex::decode(hex)
}

// Utility function to convert bytes to a hex string
fn bytes_to_hex(bytes: &[u8]) -> String {
    hex::encode(bytes)
}

/// Custom serializer for Vec<u8> to a hex string
pub(crate) fn serialize_hex_string<S>(bytes: &Vec<u8>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let hex_string = bytes_to_hex(bytes);
    serializer.serialize_str(&hex_string)
}

/// Custom serializer for Vec<Vec<u8>> to a vector of hex strings
pub(crate) fn serialize_hex_string_vec<S>(
    bytes_vec: &Vec<Vec<u8>>,
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let mut seq = serializer.serialize_seq(Some(bytes_vec.len()))?;
    for bytes in bytes_vec {
        seq.serialize_element(&bytes_to_hex(bytes))?;
    }
    seq.end()
}

/// Custom serializer for Digest to a hex string
pub(crate) fn serialize_digest<S>(digest: &Digest, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let hex_string = bytes_to_hex(digest.as_bytes());
    serializer.serialize_str(&hex_string)
}

/// Custom serializer for Vec<Digest> to a vector of hex strings
pub(crate) fn serialize_digest_vec<S>(
    digests: &Vec<Digest>,
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let mut seq = serializer.serialize_seq(Some(digests.len()))?;
    for digest in digests {
        seq.serialize_element(&bytes_to_hex(digest.as_bytes()))?;
    }
    seq.end()
}

/// Custom deserializer for Vec<u8> from a hex string
pub(crate) fn deserialize_hex_string<'de, D>(deserializer: D) -> Result<Vec<u8>, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    hex_to_bytes(&s).map_err(de::Error::custom)
}

/// Custom deserializer for Vec<Vec<u8>> from a vector of hex strings
pub(crate) fn deserialize_hex_string_vec<'de, D>(deserializer: D) -> Result<Vec<Vec<u8>>, D::Error>
where
    D: Deserializer<'de>,
{
    let vec = Vec::<String>::deserialize(deserializer)?;
    vec.into_iter()
        .map(|s| hex_to_bytes(&s).map_err(de::Error::custom))
        .collect()
}

/// Custom deserializer for Digest from a hex string
pub(crate) fn deserialize_digest<'de, D>(deserializer: D) -> Result<Digest, D::Error>
where
    D: Deserializer<'de>,
{
    let bytes: Vec<u8> = deserialize_hex_string(deserializer)?;
    Digest::try_from(&bytes[..]).map_err(de::Error::custom)
}

/// Custom deserializer for Vec<Digest> from a vector of hex strings
pub(crate) fn deserialize_digest_vec<'de, D>(deserializer: D) -> Result<Vec<Digest>, D::Error>
where
    D: Deserializer<'de>,
{
    let vec = Vec::<String>::deserialize(deserializer)?;
    vec.into_iter()
        .map(|s| {
            let bytes: Vec<u8> = hex_to_bytes(&s).map_err(de::Error::custom)?;
            Digest::try_from(&bytes[..]).map_err(de::Error::custom)
        })
        .collect()
}
