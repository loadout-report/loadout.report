use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use serde_repr::{Serialize_repr, Deserialize_repr};
use serde_with::{serde_as, DisplayFromStr};

/// DestinyManifest is the external-facing contract for just the properties needed by those calling the Destiny Platform.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DestinyManifest {

    /// Information about the "Image Pyramid" for Destiny icons. Where possible, we create smaller versions of Destiny icons. These are found as subfolders under the location of the "original/full size" Destiny images, with the same file name and extension as the original image itself. (this lets us avoid sending largely redundant path info with every entity, at the expense of the smaller versions of the image being less discoverable)
    pub icon_image_pyramid_info: Vec<crate::generated::models::destiny::config::ImagePyramidEntry>,
    /// This points to the generated JSON that contains all the Definitions. Each key is a locale. The value is a dictionary, where the key is a definition type by name, and the value is the path to the file for that definition. WARNING: This is unsafe and subject to change - do not depend on data in these files staying around long-term.
    pub json_world_component_content_paths: i32,
    /// This points to the generated JSON that contains all the Definitions. Each key is a locale. The value is a path to the aggregated world definitions (warning: large file!)
    pub json_world_content_paths: i32,
    /// No documentation provided.
    pub mobile_asset_content_path: String,
    /// No documentation provided.
    pub mobile_clan_banner_database_path: String,
    /// No documentation provided.
    pub mobile_gear_asset_data_bases: Vec<crate::generated::models::destiny::config::GearAssetDataBaseDefinition>,
    /// No documentation provided.
    pub mobile_gear_cdn: i32,
    /// No documentation provided.
    pub mobile_world_content_paths: i32,
    /// No documentation provided.
    pub version: String,
}

/// No documentation provided.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GearAssetDataBaseDefinition {

    /// No documentation provided.
    pub path: String,
    /// No documentation provided.
    pub version: i32,
}

/// No documentation provided.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ImagePyramidEntry {

    /// The factor by which the original image size has been reduced.
    pub factor: f32,
    /// The name of the subfolder where these images are located.
    pub name: String,
}
