use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use serde_repr::{Serialize_repr, Deserialize_repr};
use serde_with::{serde_as, DisplayFromStr};

/// No documentation provided.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ClanBannerDecal {

    /// No documentation provided.
    pub background_path: String,
    /// No documentation provided.
    pub foreground_path: String,
    /// No documentation provided.
    pub identifier: String,
}

pub type ClanBannerSource = serde_json::Value;
