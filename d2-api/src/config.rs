use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct UserTheme {
    #[serde(rename = "userThemeId")]
    pub id: i32,
    #[serde(rename = "userThemeName")]
    pub name: String,
    #[serde(rename = "userThemeDescription")]
    pub description: String,
}
