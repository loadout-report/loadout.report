use crate::destiny::ColorHash;
use serde::{Deserialize, Serialize};

/// Represents a color whose RGBA values are all represented as values between 0 and 255.
#[derive(Clone, Serialize, Deserialize, Debug, Default, Eq, PartialEq)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct Color {
    /// This field is not documented but seems to be present in the API response.
    pub color_hash: ColorHash,
    pub red: u8,
    pub green: u8,
    pub blue: u8,
    pub alpha: u8,
}
