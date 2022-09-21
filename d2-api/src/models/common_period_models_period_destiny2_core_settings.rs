/*
 * Bungie.Net API
 *
 * These endpoints constitute the functionality exposed by Bungie.net, both for more traditional website functionality and for connectivity to Bungie video games and their related functionality.
 *
 * The version of the OpenAPI document: 2.16.0
 * Contact: support@bungie.com
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct CommonPeriodModelsPeriodDestiny2CoreSettings {
    #[serde(rename = "collectionRootNode", skip_serializing_if = "Option::is_none")]
    pub collection_root_node: Option<i32>,
    #[serde(rename = "badgesRootNode", skip_serializing_if = "Option::is_none")]
    pub badges_root_node: Option<i32>,
    #[serde(rename = "recordsRootNode", skip_serializing_if = "Option::is_none")]
    pub records_root_node: Option<i32>,
    #[serde(rename = "medalsRootNode", skip_serializing_if = "Option::is_none")]
    pub medals_root_node: Option<i32>,
    #[serde(rename = "metricsRootNode", skip_serializing_if = "Option::is_none")]
    pub metrics_root_node: Option<i32>,
    #[serde(rename = "activeTriumphsRootNodeHash", skip_serializing_if = "Option::is_none")]
    pub active_triumphs_root_node_hash: Option<i32>,
    #[serde(rename = "activeSealsRootNodeHash", skip_serializing_if = "Option::is_none")]
    pub active_seals_root_node_hash: Option<i32>,
    #[serde(rename = "legacyTriumphsRootNodeHash", skip_serializing_if = "Option::is_none")]
    pub legacy_triumphs_root_node_hash: Option<i32>,
    #[serde(rename = "legacySealsRootNodeHash", skip_serializing_if = "Option::is_none")]
    pub legacy_seals_root_node_hash: Option<i32>,
    #[serde(rename = "medalsRootNodeHash", skip_serializing_if = "Option::is_none")]
    pub medals_root_node_hash: Option<i32>,
    #[serde(rename = "exoticCatalystsRootNodeHash", skip_serializing_if = "Option::is_none")]
    pub exotic_catalysts_root_node_hash: Option<i32>,
    #[serde(rename = "loreRootNodeHash", skip_serializing_if = "Option::is_none")]
    pub lore_root_node_hash: Option<i32>,
    #[serde(rename = "craftingRootNodeHash", skip_serializing_if = "Option::is_none")]
    pub crafting_root_node_hash: Option<i32>,
    #[serde(rename = "currentRankProgressionHashes", skip_serializing_if = "Option::is_none")]
    pub current_rank_progression_hashes: Option<Vec<i32>>,
    #[serde(rename = "insertPlugFreeProtectedPlugItemHashes", skip_serializing_if = "Option::is_none")]
    pub insert_plug_free_protected_plug_item_hashes: Option<Vec<i32>>,
    #[serde(rename = "insertPlugFreeBlockedSocketTypeHashes", skip_serializing_if = "Option::is_none")]
    pub insert_plug_free_blocked_socket_type_hashes: Option<Vec<i32>>,
    #[serde(rename = "undiscoveredCollectibleImage", skip_serializing_if = "Option::is_none")]
    pub undiscovered_collectible_image: Option<String>,
    #[serde(rename = "ammoTypeHeavyIcon", skip_serializing_if = "Option::is_none")]
    pub ammo_type_heavy_icon: Option<String>,
    #[serde(rename = "ammoTypeSpecialIcon", skip_serializing_if = "Option::is_none")]
    pub ammo_type_special_icon: Option<String>,
    #[serde(rename = "ammoTypePrimaryIcon", skip_serializing_if = "Option::is_none")]
    pub ammo_type_primary_icon: Option<String>,
    #[serde(rename = "currentSeasonalArtifactHash", skip_serializing_if = "Option::is_none")]
    pub current_seasonal_artifact_hash: Option<i32>,
    #[serde(rename = "currentSeasonHash", skip_serializing_if = "Option::is_none")]
    pub current_season_hash: Option<i32>,
    #[serde(rename = "seasonalChallengesPresentationNodeHash", skip_serializing_if = "Option::is_none")]
    pub seasonal_challenges_presentation_node_hash: Option<i32>,
    #[serde(rename = "futureSeasonHashes", skip_serializing_if = "Option::is_none")]
    pub future_season_hashes: Option<Vec<i32>>,
    #[serde(rename = "pastSeasonHashes", skip_serializing_if = "Option::is_none")]
    pub past_season_hashes: Option<Vec<i32>>,
}

impl CommonPeriodModelsPeriodDestiny2CoreSettings {
    pub fn new() -> CommonPeriodModelsPeriodDestiny2CoreSettings {
        CommonPeriodModelsPeriodDestiny2CoreSettings {
            collection_root_node: None,
            badges_root_node: None,
            records_root_node: None,
            medals_root_node: None,
            metrics_root_node: None,
            active_triumphs_root_node_hash: None,
            active_seals_root_node_hash: None,
            legacy_triumphs_root_node_hash: None,
            legacy_seals_root_node_hash: None,
            medals_root_node_hash: None,
            exotic_catalysts_root_node_hash: None,
            lore_root_node_hash: None,
            crafting_root_node_hash: None,
            current_rank_progression_hashes: None,
            insert_plug_free_protected_plug_item_hashes: None,
            insert_plug_free_blocked_socket_type_hashes: None,
            undiscovered_collectible_image: None,
            ammo_type_heavy_icon: None,
            ammo_type_special_icon: None,
            ammo_type_primary_icon: None,
            current_seasonal_artifact_hash: None,
            current_season_hash: None,
            seasonal_challenges_presentation_node_hash: None,
            future_season_hashes: None,
            past_season_hashes: None,
        }
    }
}


