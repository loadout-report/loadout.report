

use serde::{Serialize, Deserialize};


/// No documentation provided.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ClanBannerDecal {

    /// No documentation provided.
    pub identifier: String,
    /// No documentation provided.
    pub background_path: String,
    /// No documentation provided.
    pub foreground_path: String,
}

pub type ClanBannerSource = serde_json::Value;
