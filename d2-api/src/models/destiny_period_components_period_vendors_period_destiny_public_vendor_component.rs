/*
 * Bungie.Net API
 *
 * These endpoints constitute the functionality exposed by Bungie.net, both for more traditional website functionality and for connectivity to Bungie video games and their related functionality.
 *
 * The version of the OpenAPI document: 2.16.0
 * Contact: support@bungie.com
 * Generated by: https://openapi-generator.tech
 */

/// DestinyPeriodComponentsPeriodVendorsPeriodDestinyPublicVendorComponent : This component contains essential/summary information about the vendor from the perspective of a character-agnostic view.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct DestinyPeriodComponentsPeriodVendorsPeriodDestinyPublicVendorComponent {
    /// The unique identifier for the vendor. Use it to look up their DestinyVendorDefinition.
    #[serde(rename = "vendorHash", skip_serializing_if = "Option::is_none")]
    pub vendor_hash: Option<i32>,
    /// The date when this vendor's inventory will next rotate/refresh.  Note that this is distinct from the date ranges that the vendor is visible/available in-game: this field indicates the specific time when the vendor's available items refresh and rotate, regardless of whether the vendor is actually available at that time. Unfortunately, these two values may be (and are, for the case of important vendors like Xur) different.  Issue https://github.com/Bungie-net/api/issues/353 is tracking a fix to start providing visibility date ranges where possible in addition to this refresh date, so that all important dates for vendors are available for use.
    #[serde(rename = "nextRefreshDate", skip_serializing_if = "Option::is_none")]
    pub next_refresh_date: Option<String>,
    /// If True, the Vendor is currently accessible.   If False, they may not actually be visible in the world at the moment.
    #[serde(rename = "enabled", skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
}

impl DestinyPeriodComponentsPeriodVendorsPeriodDestinyPublicVendorComponent {
    /// This component contains essential/summary information about the vendor from the perspective of a character-agnostic view.
    pub fn new() -> DestinyPeriodComponentsPeriodVendorsPeriodDestinyPublicVendorComponent {
        DestinyPeriodComponentsPeriodVendorsPeriodDestinyPublicVendorComponent {
            vendor_hash: None,
            next_refresh_date: None,
            enabled: None,
        }
    }
}


