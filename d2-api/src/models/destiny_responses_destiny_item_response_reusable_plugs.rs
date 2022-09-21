/*
 * Bungie.Net API
 *
 * These endpoints constitute the functionality exposed by Bungie.net, both for more traditional website functionality and for connectivity to Bungie video games and their related functionality.
 *
 * The version of the OpenAPI document: 2.16.0
 * Contact: support@bungie.com
 * Generated by: https://openapi-generator.tech
 */

/// DestinyResponsesDestinyItemResponseReusablePlugs : Information about the Reusable Plugs for sockets on an item. These are plugs that you can insert into the given socket regardless of if you actually own an instance of that plug: they are logic-driven plugs rather than inventory-driven.   These may need to be combined with Plug Set component data to get a full picture of available plugs on a given socket.   COMPONENT TYPE: ItemReusablePlugs



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct DestinyResponsesDestinyItemResponseReusablePlugs {
    #[serde(rename = "data", skip_serializing_if = "Option::is_none")]
    pub data: Option<Box<crate::models::DestinyPeriodComponentsPeriodItemsPeriodDestinyItemReusablePlugsComponent>>,
    #[serde(rename = "privacy", skip_serializing_if = "Option::is_none")]
    pub privacy: Option<i32>,
    /// If true, this component is disabled.
    #[serde(rename = "disabled", skip_serializing_if = "Option::is_none")]
    pub disabled: Option<bool>,
}

impl DestinyResponsesDestinyItemResponseReusablePlugs {
    /// Information about the Reusable Plugs for sockets on an item. These are plugs that you can insert into the given socket regardless of if you actually own an instance of that plug: they are logic-driven plugs rather than inventory-driven.   These may need to be combined with Plug Set component data to get a full picture of available plugs on a given socket.   COMPONENT TYPE: ItemReusablePlugs
    pub fn new() -> DestinyResponsesDestinyItemResponseReusablePlugs {
        DestinyResponsesDestinyItemResponseReusablePlugs {
            data: None,
            privacy: None,
            disabled: None,
        }
    }
}


