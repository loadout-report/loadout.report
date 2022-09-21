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
pub struct ContentPeriodModelsPeriodTagMetadataDefinition {
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "order", skip_serializing_if = "Option::is_none")]
    pub order: Option<i32>,
    #[serde(rename = "items", skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<crate::models::ContentPeriodModelsPeriodTagMetadataItem>>,
    #[serde(rename = "datatype", skip_serializing_if = "Option::is_none")]
    pub datatype: Option<String>,
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "isRequired", skip_serializing_if = "Option::is_none")]
    pub is_required: Option<bool>,
}

impl ContentPeriodModelsPeriodTagMetadataDefinition {
    pub fn new() -> ContentPeriodModelsPeriodTagMetadataDefinition {
        ContentPeriodModelsPeriodTagMetadataDefinition {
            description: None,
            order: None,
            items: None,
            datatype: None,
            name: None,
            is_required: None,
        }
    }
}


