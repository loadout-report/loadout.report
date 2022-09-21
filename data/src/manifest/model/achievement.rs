use serde::Deserialize;

use super::*;

#[derive(Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Achievement {
    display_properties: DisplayProperties,
    accumulator_threshold: i32,
    platform_index: i32,
    hash: Hash,
    index: i32,
    redacted: bool,
    blacklisted: bool,
}
