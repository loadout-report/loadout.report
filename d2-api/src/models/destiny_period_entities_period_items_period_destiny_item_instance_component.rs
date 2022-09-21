/*
 * Bungie.Net API
 *
 * These endpoints constitute the functionality exposed by Bungie.net, both for more traditional website functionality and for connectivity to Bungie video games and their related functionality.
 *
 * The version of the OpenAPI document: 2.16.0
 * Contact: support@bungie.com
 * Generated by: https://openapi-generator.tech
 */

/// DestinyPeriodEntitiesPeriodItemsPeriodDestinyItemInstanceComponent : If an item is \"instanced\", this will contain information about the item's instance that doesn't fit easily into other components. One might say this is the \"essential\" instance data for the item.  Items are instanced if they require information or state that can vary. For instance, weapons are Instanced: they are given a unique identifier, uniquely generated stats, and can have their properties altered. Non-instanced items have none of these things: for instance, Glimmer has no unique properties aside from how much of it you own.  You can tell from an item's definition whether it will be instanced or not by looking at the DestinyInventoryItemDefinition's definition.inventory.isInstanceItem property.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct DestinyPeriodEntitiesPeriodItemsPeriodDestinyItemInstanceComponent {
    /// If the item has a damage type, this is the item's current damage type.
    #[serde(rename = "damageType", skip_serializing_if = "Option::is_none")]
    pub damage_type: Option<i32>,
    /// The current damage type's hash, so you can look up localized info and icons for it.
    #[serde(rename = "damageTypeHash", skip_serializing_if = "Option::is_none")]
    pub damage_type_hash: Option<i32>,
    #[serde(rename = "primaryStat", skip_serializing_if = "Option::is_none")]
    pub primary_stat: Option<Box<crate::models::DestinyEntitiesItemsDestinyItemInstanceComponentPrimaryStat>>,
    /// The Item's \"Level\" has the most significant bearing on its stats, such as Light and Power.
    #[serde(rename = "itemLevel", skip_serializing_if = "Option::is_none")]
    pub item_level: Option<i32>,
    /// The \"Quality\" of the item has a lesser - but still impactful - bearing on stats like Light and Power.
    #[serde(rename = "quality", skip_serializing_if = "Option::is_none")]
    pub quality: Option<i32>,
    /// Is the item currently equipped on the given character?
    #[serde(rename = "isEquipped", skip_serializing_if = "Option::is_none")]
    pub is_equipped: Option<bool>,
    /// If this is an equippable item, you can check it here. There are permanent as well as transitory reasons why an item might not be able to be equipped: check cannotEquipReason for details.
    #[serde(rename = "canEquip", skip_serializing_if = "Option::is_none")]
    pub can_equip: Option<bool>,
    /// If the item cannot be equipped until you reach a certain level, that level will be reflected here.
    #[serde(rename = "equipRequiredLevel", skip_serializing_if = "Option::is_none")]
    pub equip_required_level: Option<i32>,
    /// Sometimes, there are limitations to equipping that are represented by character-level flags called \"unlocks\".  This is a list of flags that they need in order to equip the item that the character has not met. Use these to look up the descriptions to show in your UI by looking up the relevant DestinyUnlockDefinitions for the hashes.
    #[serde(rename = "unlockHashesRequiredToEquip", skip_serializing_if = "Option::is_none")]
    pub unlock_hashes_required_to_equip: Option<Vec<i32>>,
    /// If you cannot equip the item, this is a flags enum that enumerates all of the reasons why you couldn't equip the item. You may need to refine your UI further by using unlockHashesRequiredToEquip and equipRequiredLevel.
    #[serde(rename = "cannotEquipReason", skip_serializing_if = "Option::is_none")]
    pub cannot_equip_reason: Option<i32>,
    /// If populated, this item has a breaker type corresponding to the given value. See DestinyBreakerTypeDefinition for more details.
    #[serde(rename = "breakerType", skip_serializing_if = "Option::is_none")]
    pub breaker_type: Option<BreakerType>,
    /// If populated, this is the hash identifier for the item's breaker type. See DestinyBreakerTypeDefinition for more details.
    #[serde(rename = "breakerTypeHash", skip_serializing_if = "Option::is_none")]
    pub breaker_type_hash: Option<i32>,
    #[serde(rename = "energy", skip_serializing_if = "Option::is_none")]
    pub energy: Option<Box<crate::models::DestinyEntitiesItemsDestinyItemInstanceComponentEnergy>>,
}

impl DestinyPeriodEntitiesPeriodItemsPeriodDestinyItemInstanceComponent {
    /// If an item is \"instanced\", this will contain information about the item's instance that doesn't fit easily into other components. One might say this is the \"essential\" instance data for the item.  Items are instanced if they require information or state that can vary. For instance, weapons are Instanced: they are given a unique identifier, uniquely generated stats, and can have their properties altered. Non-instanced items have none of these things: for instance, Glimmer has no unique properties aside from how much of it you own.  You can tell from an item's definition whether it will be instanced or not by looking at the DestinyInventoryItemDefinition's definition.inventory.isInstanceItem property.
    pub fn new() -> DestinyPeriodEntitiesPeriodItemsPeriodDestinyItemInstanceComponent {
        DestinyPeriodEntitiesPeriodItemsPeriodDestinyItemInstanceComponent {
            damage_type: None,
            damage_type_hash: None,
            primary_stat: None,
            item_level: None,
            quality: None,
            is_equipped: None,
            can_equip: None,
            equip_required_level: None,
            unlock_hashes_required_to_equip: None,
            cannot_equip_reason: None,
            breaker_type: None,
            breaker_type_hash: None,
            energy: None,
        }
    }
}

/// If populated, this item has a breaker type corresponding to the given value. See DestinyBreakerTypeDefinition for more details.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum BreakerType {
    #[serde(rename = "0")]
    Variant0,
    #[serde(rename = "1")]
    Variant1,
    #[serde(rename = "2")]
    Variant2,
    #[serde(rename = "3")]
    Variant3,
}

impl Default for BreakerType {
    fn default() -> BreakerType {
        Self::Variant0
    }
}

