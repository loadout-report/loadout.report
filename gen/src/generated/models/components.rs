use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use serde_repr::{Serialize_repr, Deserialize_repr};
use serde_with::{serde_as, DisplayFromStr};

/// A set of flags for reason(s) why the component populated in the way that it did. Inspect the individual flags for the reasons.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize_repr, Deserialize_repr)]
#[repr(i32)]
pub enum ComponentPrivacySetting {
    /// No documentation provided.
    None = 0,
    /// No documentation provided.
    Public = 1,
    /// No documentation provided.
    Private = 2,
}

/// The base class for any component-returning object that may need to indicate information about the state of the component being returned.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ComponentResponse {

    /// No documentation provided.
    pub privacy: crate::generated::models::components::ComponentPrivacySetting,
    /// If true, this component is disabled.
    pub disabled: Option<bool>,
}
