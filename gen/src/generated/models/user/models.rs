

use serde::{Serialize, Deserialize};


/// No documentation provided.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetCredentialTypesForAccountResponse {

    /// No documentation provided.
    pub credential_display_name: String,
    /// No documentation provided.
    pub is_public: bool,
    /// No documentation provided.
    pub credential_as_string: String,
    /// No documentation provided.
    pub credential_type: i32,
}
