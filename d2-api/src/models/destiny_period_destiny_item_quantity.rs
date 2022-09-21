/*
 * Bungie.Net API
 *
 * These endpoints constitute the functionality exposed by Bungie.net, both for more traditional website functionality and for connectivity to Bungie video games and their related functionality.
 *
 * The version of the OpenAPI document: 2.16.0
 * Contact: support@bungie.com
 * Generated by: https://openapi-generator.tech
 */

/// DestinyPeriodDestinyItemQuantity : Used in a number of Destiny contracts to return data about an item stack and its quantity. Can optionally return an itemInstanceId if the item is instanced - in which case, the quantity returned will be 1. If it's not... uh, let me know okay? Thanks.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct DestinyPeriodDestinyItemQuantity {
    /// The hash identifier for the item in question. Use it to look up the item's DestinyInventoryItemDefinition.
    #[serde(rename = "itemHash", skip_serializing_if = "Option::is_none")]
    pub item_hash: Option<i32>,
    /// If this quantity is referring to a specific instance of an item, this will have the item's instance ID. Normally, this will be null.
    #[serde(rename = "itemInstanceId", skip_serializing_if = "Option::is_none")]
    pub item_instance_id: Option<i64>,
    /// The amount of the item needed/available depending on the context of where DestinyItemQuantity is being used.
    #[serde(rename = "quantity", skip_serializing_if = "Option::is_none")]
    pub quantity: Option<i32>,
    /// Indicates that this item quantity may be conditionally shown or hidden, based on various sources of state. For example: server flags, account state, or character progress.
    #[serde(rename = "hasConditionalVisibility", skip_serializing_if = "Option::is_none")]
    pub has_conditional_visibility: Option<bool>,
}

impl DestinyPeriodDestinyItemQuantity {
    /// Used in a number of Destiny contracts to return data about an item stack and its quantity. Can optionally return an itemInstanceId if the item is instanced - in which case, the quantity returned will be 1. If it's not... uh, let me know okay? Thanks.
    pub fn new() -> DestinyPeriodDestinyItemQuantity {
        DestinyPeriodDestinyItemQuantity {
            item_hash: None,
            item_instance_id: None,
            quantity: None,
            has_conditional_visibility: None,
        }
    }
}


