
use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use serde_repr::{Serialize_repr, Deserialize_repr};
use serde_with::{serde_as, DisplayFromStr};
/// No documentation provided.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CoreSetting {

    /// No documentation provided.
    pub identifier: String,
    /// No documentation provided.
    pub is_default: bool,
    /// No documentation provided.
    pub child_settings: i32,
    /// No documentation provided.
    pub summary: String,
    /// No documentation provided.
    pub image_path: String,
    /// No documentation provided.
    pub display_name: String,
}

/// No documentation provided.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CoreSystem {

    /// No documentation provided.
    pub enabled: bool,
    /// No documentation provided.
    pub parameters: i32,
}

/// No documentation provided.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CoreSettingsConfiguration {

    /// No documentation provided.
    pub forum_categories: i32,
    /// No documentation provided.
    pub group_avatars: i32,
    /// No documentation provided.
    pub recruitment_platform_tags: i32,
    /// No documentation provided.
    pub clan_banner_standards: i32,
    /// No documentation provided.
    pub clan_banner_gonfalon_colors: i32,
    /// No documentation provided.
    pub clan_banner_decals: i32,
    /// No documentation provided.
    pub clan_banner_decal_colors: i32,
    /// No documentation provided.
    pub ignore_reasons: i32,
    /// No documentation provided.
    pub environment: String,
    /// No documentation provided.
    pub clan_banner_gonfalon_detail_colors: i32,
    /// No documentation provided.
    pub recruitment_misc_tags: i32,
    /// No documentation provided.
    pub user_content_locales: i32,
    /// No documentation provided.
    pub systems: i32,
    /// No documentation provided.
    pub clan_banner_gonfalons: i32,
    /// No documentation provided.
    pub system_content_locales: i32,
    /// No documentation provided.
    pub fireteam_activities: i32,
    /// No documentation provided.
    pub recruitment_activities: i32,
    /// No documentation provided.
    pub email_settings: i32,
    /// No documentation provided.
    pub destiny_membership_types: i32,
    /// No documentation provided.
    pub default_group_theme: i32,
    /// No documentation provided.
    pub destiny_2_core_settings: i32,
    /// No documentation provided.
    pub clan_banner_gonfalon_details: i32,
}

/// No documentation provided.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Destiny2CoreSettings {

    /// No documentation provided.
    pub insert_plug_free_blocked_socket_type_hashes: i32,
    /// No documentation provided.
    pub legacy_triumphs_root_node_hash: u32,
    /// No documentation provided.
    pub future_season_hashes: i32,
    /// No documentation provided.
    pub undiscovered_collectible_image: String,
    /// No documentation provided.
    pub past_season_hashes: i32,
    /// No documentation provided.
    pub medals_root_node_hash: u32,
    /// No documentation provided.
    pub current_seasonal_artifact_hash: u32,
    /// No documentation provided.
    pub ammo_type_primary_icon: String,
    /// No documentation provided.
    pub seasonal_challenges_presentation_node_hash: Option<u32>,
    /// No documentation provided.
    pub records_root_node: u32,
    /// No documentation provided.
    pub legacy_seals_root_node_hash: u32,
    /// No documentation provided.
    pub lore_root_node_hash: u32,
    /// No documentation provided.
    pub medals_root_node: u32,
    /// No documentation provided.
    pub crafting_root_node_hash: u32,
    /// No documentation provided.
    pub metrics_root_node: u32,
    /// No documentation provided.
    pub active_triumphs_root_node_hash: u32,
    /// No documentation provided.
    pub insert_plug_free_protected_plug_item_hashes: i32,
    /// No documentation provided.
    pub collection_root_node: u32,
    /// No documentation provided.
    pub current_season_hash: Option<u32>,
    /// No documentation provided.
    pub badges_root_node: u32,
    /// No documentation provided.
    pub active_seals_root_node_hash: u32,
    /// No documentation provided.
    pub ammo_type_heavy_icon: String,
    /// No documentation provided.
    pub current_rank_progression_hashes: i32,
    /// No documentation provided.
    pub ammo_type_special_icon: String,
    /// No documentation provided.
    pub exotic_catalysts_root_node_hash: u32,
}
