use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use serde_repr::{Serialize_repr, Deserialize_repr};
use serde_with::{serde_as, DisplayFromStr};

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
