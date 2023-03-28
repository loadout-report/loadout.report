use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use serde_repr::{Serialize_repr, Deserialize_repr};
use serde_with::{serde_as, DisplayFromStr};

/// No documentation provided.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize_repr, Deserialize_repr)]
#[repr(i32)]
pub enum IgnoreLength {
    /// No documentation provided.
    None = 0,
    /// No documentation provided.
    Week = 1,
    /// No documentation provided.
    TwoWeeks = 2,
    /// No documentation provided.
    ThreeWeeks = 3,
    /// No documentation provided.
    Month = 4,
    /// No documentation provided.
    ThreeMonths = 5,
    /// No documentation provided.
    SixMonths = 6,
    /// No documentation provided.
    Year = 7,
    /// No documentation provided.
    Forever = 8,
    /// No documentation provided.
    ThreeMinutes = 9,
    /// No documentation provided.
    Hour = 10,
    /// No documentation provided.
    ThirtyDays = 11,
}

/// No documentation provided.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IgnoreResponse {

    /// No documentation provided.
    pub ignore_flags: crate::generated::models::ignores::IgnoreStatus,
    /// No documentation provided.
    pub is_ignored: bool,
}

/// No documentation provided.
/// todo: bitmask
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize_repr, Deserialize_repr)]
#[repr(i32)]
pub enum IgnoreStatus {
    /// No documentation provided.
    NotIgnored = 0,
    /// No documentation provided.
    IgnoredUser = 1,
    /// No documentation provided.
    IgnoredGroup = 2,
    /// No documentation provided.
    IgnoredByGroup = 4,
    /// No documentation provided.
    IgnoredPost = 8,
    /// No documentation provided.
    IgnoredTag = 16,
    /// No documentation provided.
    IgnoredGlobal = 32,
}
