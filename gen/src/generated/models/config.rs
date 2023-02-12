pub mod clan_banner;

use serde::{Serialize, Deserialize};


/// No documentation provided.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GroupTheme {

    /// No documentation provided.
    pub name: String,
    /// No documentation provided.
    pub description: String,
    /// No documentation provided.
    pub folder: String,
}

/// No documentation provided.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UserTheme {

    /// No documentation provided.
    pub user_theme_name: String,
    /// No documentation provided.
    pub user_theme_id: i32,
    /// No documentation provided.
    pub user_theme_description: String,
}
