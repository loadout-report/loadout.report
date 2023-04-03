use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use serde_repr::{Serialize_repr, Deserialize_repr};
use serde_with::{serde_as, DisplayFromStr};

/// No documentation provided.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InterpolationPoint {

    /// No documentation provided.
    pub value: i32,
    /// No documentation provided.
    pub weight: i32,
}

/// No documentation provided.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InterpolationPointFloat {

    /// No documentation provided.
    pub value: f32,
    /// No documentation provided.
    pub weight: f32,
}
