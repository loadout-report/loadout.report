use serde_derive::Deserialize;
use serde_derive::Serialize;
use serde_json::Value;

pub mod model;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Manifest {
    pub version: String,
    pub mobile_asset_content_path: String,
    pub mobile_gear_asset_data_bases: Vec<MobileGearAssetDataBase>,
    pub mobile_world_content_paths: InternationalisedContentPaths<String>,
    pub json_world_content_paths: InternationalisedContentPaths<String>,
    pub json_world_component_content_paths: InternationalisedContentPaths<Components>,
    pub mobile_clan_banner_database_path: String,
    #[serde(rename = "mobileGearCDN")]
    pub mobile_gear_cdn: MobileGearCdn,
    pub icon_image_pyramid_info: Vec<Value>,
}

#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MobileGearAssetDataBase {
    pub version: i64,
    pub path: String,
}

#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct InternationalisedContentPaths<T> {
    pub en: T,
    pub fr: T,
    pub es: T,
    pub es_mx: T,
    pub de: T,
    pub it: T,
    pub ja: T,
    pub pt_br: T,
    pub ru: T,
    pub pl: T,
    pub ko: T,
    pub zh_cht: T,
    pub zh_chs: T,
}

#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Components {
    pub destiny_node_step_summary_definition: String,
    pub destiny_art_dye_channel_definition: String,
    pub destiny_art_dye_reference_definition: String,
    pub destiny_place_definition: String,
    pub destiny_activity_definition: String,
    pub destiny_activity_type_definition: String,
    pub destiny_class_definition: String,
    pub destiny_gender_definition: String,
    pub destiny_inventory_bucket_definition: String,
    pub destiny_race_definition: String,
    pub destiny_talent_grid_definition: String,
    pub destiny_unlock_definition: String,
    pub destiny_sandbox_perk_definition: String,
    pub destiny_stat_group_definition: String,
    pub destiny_progression_mapping_definition: String,
    pub destiny_faction_definition: String,
    pub destiny_vendor_group_definition: String,
    pub destiny_reward_source_definition: String,
    pub destiny_unlock_value_definition: String,
    pub destiny_reward_mapping_definition: String,
    pub destiny_reward_sheet_definition: String,
    pub destiny_item_category_definition: String,
    pub destiny_damage_type_definition: String,
    pub destiny_activity_mode_definition: String,
    pub destiny_medal_tier_definition: String,
    pub destiny_achievement_definition: String,
    pub destiny_activity_graph_definition: String,
    pub destiny_activity_interactable_definition: String,
    pub destiny_bond_definition: String,
    pub destiny_character_customization_category_definition: String,
    pub destiny_character_customization_option_definition: String,
    pub destiny_collectible_definition: String,
    pub destiny_destination_definition: String,
    pub destiny_entitlement_offer_definition: String,
    pub destiny_equipment_slot_definition: String,
    pub destiny_event_card_definition: String,
    pub destiny_stat_definition: String,
    pub destiny_inventory_item_definition: String,
    pub destiny_inventory_item_lite_definition: String,
    pub destiny_item_tier_type_definition: String,
    pub destiny_location_definition: String,
    pub destiny_lore_definition: String,
    pub destiny_material_requirement_set_definition: String,
    pub destiny_metric_definition: String,
    pub destiny_objective_definition: String,
    pub destiny_platform_bucket_mapping_definition: String,
    pub destiny_plug_set_definition: String,
    pub destiny_power_cap_definition: String,
    pub destiny_presentation_node_definition: String,
    pub destiny_progression_definition: String,
    pub destiny_progression_level_requirement_definition: String,
    pub destiny_record_definition: String,
    pub destiny_reward_adjuster_pointer_definition: String,
    pub destiny_reward_adjuster_progression_map_definition: String,
    pub destiny_reward_item_list_definition: String,
    pub destiny_sack_reward_item_list_definition: String,
    pub destiny_sandbox_pattern_definition: String,
    pub destiny_season_definition: String,
    pub destiny_season_pass_definition: String,
    pub destiny_socket_category_definition: String,
    pub destiny_socket_type_definition: String,
    pub destiny_trait_definition: String,
    pub destiny_trait_category_definition: String,
    pub destiny_unlock_count_mapping_definition: String,
    pub destiny_unlock_event_definition: String,
    pub destiny_unlock_expression_mapping_definition: String,
    pub destiny_vendor_definition: String,
    pub destiny_milestone_definition: String,
    pub destiny_activity_modifier_definition: String,
    pub destiny_report_reason_category_definition: String,
    pub destiny_artifact_definition: String,
    pub destiny_breaker_type_definition: String,
    pub destiny_checklist_definition: String,
    pub destiny_energy_type_definition: String,
}

#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct MobileGearCdn {
    pub geometry: String,
    pub texture: String,
    pub plate_region: String,
    pub gear: String,
    pub shader: String,
}