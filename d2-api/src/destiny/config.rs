use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct Manifest {
    pub version: String,
    pub mobile_asset_content_path: String,
    pub mobile_gear_asset_data_bases: Vec<GearAssetDataBaseDefinition>,
    pub mobile_world_content_paths: HashMap<Locale, String>,
    pub json_world_content_paths: HashMap<Locale, String>,
    pub json_world_component_content_paths: HashMap<Locale, HashMap<Definitions, String>>,
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

#[derive(Debug, Clone, Serialize, Deserialize, Hash, Eq, PartialEq, Copy)]
#[serde(deny_unknown_fields, rename_all = "kebab-case")]
pub enum Locale {
    En,
    Fr,
    Es,
    EsMx,
    De,
    It,
    Ja,
    PtBr,
    Ru,
    Pl,
    Ko,
    ZhCht,
    ZhChs,
}

#[derive(Debug, Clone, Serialize, Deserialize, Hash, Eq, PartialEq, Copy)]
#[serde(deny_unknown_fields, rename_all = "PascalCase")]
pub enum Definitions {
    DestinyNodeStepSummaryDefinition,
    DestinyArtDyeChannelDefinition,
    DestinyArtDyeReferenceDefinition,
    DestinyPlaceDefinition,
    DestinyActivityDefinition,
    DestinyActivityTypeDefinition,
    DestinyClassDefinition,
    DestinyGenderDefinition,
    DestinyInventoryBucketDefinition,
    DestinyRaceDefinition,
    DestinyTalentGridDefinition,
    DestinyUnlockDefinition,
    DestinySandboxPerkDefinition,
    DestinyStatGroupDefinition,
    DestinyProgressionMappingDefinition,
    DestinyFactionDefinition,
    DestinyVendorGroupDefinition,
    DestinyRewardSourceDefinition,
    DestinyUnlockValueDefinition,
    DestinyRewardMappingDefinition,
    DestinyRewardSheetDefinition,
    DestinyItemCategoryDefinition,
    DestinyDamageTypeDefinition,
    DestinyActivityModeDefinition,
    DestinyMedalTierDefinition,
    DestinyAchievementDefinition,
    DestinyActivityGraphDefinition,
    DestinyActivityInteractableDefinition,
    DestinyBondDefinition,
    DestinyCharacterCustomizationCategoryDefinition,
    DestinyCharacterCustomizationOptionDefinition,
    DestinyCollectibleDefinition,
    DestinyDestinationDefinition,
    DestinyEntitlementOfferDefinition,
    DestinyEquipmentSlotDefinition,
    DestinyEventCardDefinition,
    DestinyStatDefinition,
    DestinyInventoryItemDefinition,
    DestinyInventoryItemLiteDefinition,
    DestinyItemTierTypeDefinition,
    DestinyLocationDefinition,
    DestinyLoreDefinition,
    DestinyMaterialRequirementSetDefinition,
    DestinyMetricDefinition,
    DestinyObjectiveDefinition,
    DestinyPlatformBucketMappingDefinition,
    DestinyPlugSetDefinition,
    DestinyPowerCapDefinition,
    DestinyPresentationNodeDefinition,
    DestinyProgressionDefinition,
    DestinyProgressionLevelRequirementDefinition,
    DestinyRecordDefinition,
    DestinyRewardAdjusterPointerDefinition,
    DestinyRewardAdjusterProgressionMapDefinition,
    DestinyRewardItemListDefinition,
    DestinySackRewardItemListDefinition,
    DestinySandboxPatternDefinition,
    DestinySeasonDefinition,
    DestinySeasonPassDefinition,
    DestinySocketCategoryDefinition,
    DestinySocketTypeDefinition,
    DestinyTraitDefinition,
    DestinyTraitCategoryDefinition,
    DestinyUnlockCountMappingDefinition,
    DestinyUnlockEventDefinition,
    DestinyUnlockExpressionMappingDefinition,
    DestinyVendorDefinition,
    DestinyMilestoneDefinition,
    DestinyActivityModifierDefinition,
    DestinyReportReasonCategoryDefinition,
    DestinyArtifactDefinition,
    DestinyBreakerTypeDefinition,
    DestinyChecklistDefinition,
    DestinyEnergyTypeDefinition,
}
