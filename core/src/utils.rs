use risc0_zkp::core::digest::Digest;
use serde::de::{self};
use serde::{Deserialize, Deserializer};

// Utility function to convert a hex string to a Vec<u8>
fn hex_to_bytes(hex: &str) -> Result<Vec<u8>, hex::FromHexError> {
    hex::decode(hex)
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
