

use serde::{Serialize, Deserialize};


/// Represents a color whose RGBA values are all represented as values between 0 and 255.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DestinyColor {

    /// No documentation provided.
    pub alpha: String,
    /// No documentation provided.
    pub blue: String,
    /// No documentation provided.
    pub green: String,
    /// No documentation provided.
    pub red: String,
}
