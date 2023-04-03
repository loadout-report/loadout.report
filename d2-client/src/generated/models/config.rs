use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use serde_repr::{Serialize_repr, Deserialize_repr};
use serde_with::{serde_as, DisplayFromStr};
pub mod clan_banner;
/// No documentation provided.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GroupTheme {

    /// No documentation provided.
    pub description: String,
    /// No documentation provided.
    pub folder: String,
    /// No documentation provided.
    pub name: String,
}

/// No documentation provided.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UserTheme {

    /// No documentation provided.
    pub user_theme_description: String,
    /// No documentation provided.
    pub user_theme_id: i32,
    /// No documentation provided.
    pub user_theme_name: String,
}
