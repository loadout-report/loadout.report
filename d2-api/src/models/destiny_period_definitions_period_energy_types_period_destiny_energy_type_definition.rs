/*
 * Bungie.Net API
 *
 * These endpoints constitute the functionality exposed by Bungie.net, both for more traditional website functionality and for connectivity to Bungie video games and their related functionality.
 *
 * The version of the OpenAPI document: 2.16.0
 * Contact: support@bungie.com
 * Generated by: https://openapi-generator.tech
 */

/// DestinyPeriodDefinitionsPeriodEnergyTypesPeriodDestinyEnergyTypeDefinition : Represents types of Energy that can be used for costs and payments related to Armor 2.0 mods.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct DestinyPeriodDefinitionsPeriodEnergyTypesPeriodDestinyEnergyTypeDefinition {
    #[serde(rename = "displayProperties", skip_serializing_if = "Option::is_none")]
    pub display_properties: Option<Box<crate::models::DestinyDefinitionsEnergyTypesDestinyEnergyTypeDefinitionDisplayProperties>>,
    /// A variant of the icon that is transparent and colorless.
    #[serde(rename = "transparentIconPath", skip_serializing_if = "Option::is_none")]
    pub transparent_icon_path: Option<String>,
    /// If TRUE, the game shows this Energy type's icon. Otherwise, it doesn't. Whether you show it or not is up to you.
    #[serde(rename = "showIcon", skip_serializing_if = "Option::is_none")]
    pub show_icon: Option<bool>,
    /// We have an enumeration for Energy types for quick reference. This is the current definition's Energy type enum value.
    #[serde(rename = "enumValue", skip_serializing_if = "Option::is_none")]
    pub enum_value: Option<i32>,
    /// If this Energy Type can be used for determining the Type of Energy that an item can consume, this is the hash for the DestinyInvestmentStatDefinition that represents the stat which holds the Capacity for that energy type. (Note that this is optional because \"Any\" is a valid cost, but not valid for Capacity - an Armor must have a specific Energy Type for determining the energy type that the Armor is restricted to use)
    #[serde(rename = "capacityStatHash", skip_serializing_if = "Option::is_none")]
    pub capacity_stat_hash: Option<i32>,
    /// If this Energy Type can be used as a cost to pay for socketing Armor 2.0 items, this is the hash for the DestinyInvestmentStatDefinition that stores the plug's raw cost.
    #[serde(rename = "costStatHash", skip_serializing_if = "Option::is_none")]
    pub cost_stat_hash: Option<i32>,
    /// The unique identifier for this entity. Guaranteed to be unique for the type of entity, but not globally.  When entities refer to each other in Destiny content, it is this hash that they are referring to.
    #[serde(rename = "hash", skip_serializing_if = "Option::is_none")]
    pub hash: Option<i32>,
    /// The index of the entity as it was found in the investment tables.
    #[serde(rename = "index", skip_serializing_if = "Option::is_none")]
    pub index: Option<i32>,
    /// If this is true, then there is an entity with this identifier/type combination, but BNet is not yet allowed to show it. Sorry!
    #[serde(rename = "redacted", skip_serializing_if = "Option::is_none")]
    pub redacted: Option<bool>,
}

impl DestinyPeriodDefinitionsPeriodEnergyTypesPeriodDestinyEnergyTypeDefinition {
    /// Represents types of Energy that can be used for costs and payments related to Armor 2.0 mods.
    pub fn new() -> DestinyPeriodDefinitionsPeriodEnergyTypesPeriodDestinyEnergyTypeDefinition {
        DestinyPeriodDefinitionsPeriodEnergyTypesPeriodDestinyEnergyTypeDefinition {
            display_properties: None,
            transparent_icon_path: None,
            show_icon: None,
            enum_value: None,
            capacity_stat_hash: None,
            cost_stat_hash: None,
            hash: None,
            index: None,
            redacted: None,
        }
    }
}


