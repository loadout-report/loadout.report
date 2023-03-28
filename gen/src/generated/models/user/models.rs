use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use serde_repr::{Serialize_repr, Deserialize_repr};
use serde_with::{serde_as, DisplayFromStr};

/// No documentation provided.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetCredentialTypesForAccountResponse {

    /// No documentation provided.
    pub credential_as_string: String,
    /// No documentation provided.
    pub is_public: bool,
    /// No documentation provided.
    pub credential_type: crate::generated::models::BungieCredentialType,
    /// No documentation provided.
    pub credential_display_name: String,
}
