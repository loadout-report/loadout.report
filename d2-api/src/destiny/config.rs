use std::collections::HashMap;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct Manifest {
    pub version: String,
    pub mobile_asset_content_path: String,
    pub mobile_gear_asset_data_bases: Vec<GearAssetDataBaseDefinition>,
    pub mobile_world_content_paths: HashMap<String, String>,
    pub json_world_content_paths: HashMap<String, String>,
    pub json_world_component_content_paths: HashMap<String, String>,
    pub mobile_clan_banner_database_path: String,
    #[serde(rename = "mobileGearCDN")]
    pub mobile_gear_cdn: HashMap<String, String>,
    pub icon_image_pyramid_info: Vec<ImagePyramidEntry>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct GearAssetDataBaseDefinition {
    pub version: i32,
    pub path: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct ImagePyramidEntry {
    pub name: String,
    pub factor: f32,
}
