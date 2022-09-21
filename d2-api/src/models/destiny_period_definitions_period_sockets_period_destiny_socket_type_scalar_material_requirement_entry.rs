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
pub struct DestinyPeriodDefinitionsPeriodSocketsPeriodDestinySocketTypeScalarMaterialRequirementEntry {
    #[serde(rename = "currencyItemHash", skip_serializing_if = "Option::is_none")]
    pub currency_item_hash: Option<i32>,
    #[serde(rename = "scalarValue", skip_serializing_if = "Option::is_none")]
    pub scalar_value: Option<i32>,
}

impl DestinyPeriodDefinitionsPeriodSocketsPeriodDestinySocketTypeScalarMaterialRequirementEntry {
    pub fn new() -> DestinyPeriodDefinitionsPeriodSocketsPeriodDestinySocketTypeScalarMaterialRequirementEntry {
        DestinyPeriodDefinitionsPeriodSocketsPeriodDestinySocketTypeScalarMaterialRequirementEntry {
            currency_item_hash: None,
            scalar_value: None,
        }
    }
}


