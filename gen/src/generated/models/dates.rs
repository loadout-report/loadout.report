use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use serde_repr::{Serialize_repr, Deserialize_repr};
use serde_with::{serde_as, DisplayFromStr};

/// No documentation provided.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DateRange {

    /// No documentation provided.
    pub start: chrono::DateTime<chrono::Utc>,
    /// No documentation provided.
    pub end: chrono::DateTime<chrono::Utc>,
}
