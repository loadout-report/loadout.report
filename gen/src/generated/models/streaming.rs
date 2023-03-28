use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use serde_repr::{Serialize_repr, Deserialize_repr};
use serde_with::{serde_as, DisplayFromStr};

/// No documentation provided.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum DropStateEnum {
    /// No documentation provided.
    Claimed = 0,
    /// No documentation provided.
    Applied = 1,
    /// No documentation provided.
    Fulfilled = 2,
}
