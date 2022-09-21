/*
 * Bungie.Net API
 *
 * These endpoints constitute the functionality exposed by Bungie.net, both for more traditional website functionality and for connectivity to Bungie video games and their related functionality.
 *
 * The version of the OpenAPI document: 2.16.0
 * Contact: support@bungie.com
 * Generated by: https://openapi-generator.tech
 */

/// DestinyDefinitionsDestinyInventoryItemDefinitionQuality : If this item can have a level or stats, this block will be non-null and will be populated with default quality (item level, \"quality\", and infusion) data. See the block for more details, there's often less upfront information in D2 so you'll want to be aware of how you use quality and item level on the definition level now.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct DestinyDefinitionsDestinyInventoryItemDefinitionQuality {
    /// The \"base\" defined level of an item. This is a list because, in theory, each Expansion could define its own base level for an item.  In practice, not only was that never done in Destiny 1, but now this isn't even populated at all. When it's not populated, the level at which it spawns has to be inferred by Reward information, of which BNet receives an imperfect view and will only be reliable on instanced data as a result.
    #[serde(rename = "itemLevels", skip_serializing_if = "Option::is_none")]
    pub item_levels: Option<Vec<i32>>,
    /// qualityLevel is used in combination with the item's level to calculate stats like Attack and Defense. It plays a role in that calculation, but not nearly as large as itemLevel does.
    #[serde(rename = "qualityLevel", skip_serializing_if = "Option::is_none")]
    pub quality_level: Option<i32>,
    /// The string identifier for this item's \"infusability\", if any.   Items that match the same infusionCategoryName are allowed to infuse with each other.  DEPRECATED: Items can now have multiple infusion categories. Please use infusionCategoryHashes instead.
    #[serde(rename = "infusionCategoryName", skip_serializing_if = "Option::is_none")]
    pub infusion_category_name: Option<String>,
    /// The hash identifier for the infusion. It does not map to a Definition entity.  DEPRECATED: Items can now have multiple infusion categories. Please use infusionCategoryHashes instead.
    #[serde(rename = "infusionCategoryHash", skip_serializing_if = "Option::is_none")]
    pub infusion_category_hash: Option<i32>,
    /// If any one of these hashes matches any value in another item's infusionCategoryHashes, the two can infuse with each other.
    #[serde(rename = "infusionCategoryHashes", skip_serializing_if = "Option::is_none")]
    pub infusion_category_hashes: Option<Vec<i32>>,
    /// An item can refer to pre-set level requirements. They are defined in DestinyProgressionLevelRequirementDefinition, and you can use this hash to find the appropriate definition.
    #[serde(rename = "progressionLevelRequirementHash", skip_serializing_if = "Option::is_none")]
    pub progression_level_requirement_hash: Option<i32>,
    /// The latest version available for this item.
    #[serde(rename = "currentVersion", skip_serializing_if = "Option::is_none")]
    pub current_version: Option<i32>,
    /// The list of versions available for this item.
    #[serde(rename = "versions", skip_serializing_if = "Option::is_none")]
    pub versions: Option<Vec<crate::models::DestinyPeriodDefinitionsPeriodDestinyItemVersionDefinition>>,
    /// Icon overlays to denote the item version and power cap status.
    #[serde(rename = "displayVersionWatermarkIcons", skip_serializing_if = "Option::is_none")]
    pub display_version_watermark_icons: Option<Vec<String>>,
}

impl DestinyDefinitionsDestinyInventoryItemDefinitionQuality {
    /// If this item can have a level or stats, this block will be non-null and will be populated with default quality (item level, \"quality\", and infusion) data. See the block for more details, there's often less upfront information in D2 so you'll want to be aware of how you use quality and item level on the definition level now.
    pub fn new() -> DestinyDefinitionsDestinyInventoryItemDefinitionQuality {
        DestinyDefinitionsDestinyInventoryItemDefinitionQuality {
            item_levels: None,
            quality_level: None,
            infusion_category_name: None,
            infusion_category_hash: None,
            infusion_category_hashes: None,
            progression_level_requirement_hash: None,
            current_version: None,
            versions: None,
            display_version_watermark_icons: None,
        }
    }
}


