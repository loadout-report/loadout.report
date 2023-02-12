

use serde::{Serialize, Deserialize};


/// No documentation provided.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InterpolationPoint {

    /// No documentation provided.
    pub weight: i32,
    /// No documentation provided.
    pub value: i32,
}

/// No documentation provided.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InterpolationPointFloat {

    /// No documentation provided.
    pub value: f32,
    /// No documentation provided.
    pub weight: f32,
}
