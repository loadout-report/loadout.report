

use serde::{Serialize, Deserialize};


/// No documentation provided.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DateRange {

    /// No documentation provided.
    pub end: chrono::DateTime<chrono::Utc>,
    /// No documentation provided.
    pub start: chrono::DateTime<chrono::Utc>,
}
