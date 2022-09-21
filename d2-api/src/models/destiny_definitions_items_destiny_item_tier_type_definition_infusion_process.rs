/*
 * Bungie.Net API
 *
 * These endpoints constitute the functionality exposed by Bungie.net, both for more traditional website functionality and for connectivity to Bungie video games and their related functionality.
 *
 * The version of the OpenAPI document: 2.16.0
 * Contact: support@bungie.com
 * Generated by: https://openapi-generator.tech
 */

/// DestinyDefinitionsItemsDestinyItemTierTypeDefinitionInfusionProcess : If this tier defines infusion properties, they will be contained here.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct DestinyDefinitionsItemsDestinyItemTierTypeDefinitionInfusionProcess {
    /// The default portion of quality that will transfer from the infuser to the infusee item. (InfuserQuality - InfuseeQuality) * baseQualityTransferRatio = base quality transferred.
    #[serde(rename = "baseQualityTransferRatio", skip_serializing_if = "Option::is_none")]
    pub base_quality_transfer_ratio: Option<f32>,
    /// As long as InfuserQuality > InfuseeQuality, the amount of quality bestowed is guaranteed to be at least this value, even if the transferRatio would dictate that it should be less. The total amount of quality that ends up in the Infusee cannot exceed the Infuser's quality however (for instance, if you infuse a 300 item with a 301 item and the minimum quality increment is 10, the infused item will not end up with 310 quality)
    #[serde(rename = "minimumQualityIncrement", skip_serializing_if = "Option::is_none")]
    pub minimum_quality_increment: Option<i32>,
}

impl DestinyDefinitionsItemsDestinyItemTierTypeDefinitionInfusionProcess {
    /// If this tier defines infusion properties, they will be contained here.
    pub fn new() -> DestinyDefinitionsItemsDestinyItemTierTypeDefinitionInfusionProcess {
        DestinyDefinitionsItemsDestinyItemTierTypeDefinitionInfusionProcess {
            base_quality_transfer_ratio: None,
            minimum_quality_increment: None,
        }
    }
}


