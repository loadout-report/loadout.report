/*
 * Bungie.Net API
 *
 * These endpoints constitute the functionality exposed by Bungie.net, both for more traditional website functionality and for connectivity to Bungie video games and their related functionality.
 *
 * The version of the OpenAPI document: 2.16.0
 * Contact: support@bungie.com
 * Generated by: https://openapi-generator.tech
 */

/// DestinyPeriodEntitiesPeriodProfilesPeriodDestinyVendorReceiptsComponent : For now, this isn't used for much: it's a record of the recent refundable purchases that the user has made. In the future, it could be used for providing refunds/buyback via the API. Wouldn't that be fun?



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct DestinyPeriodEntitiesPeriodProfilesPeriodDestinyVendorReceiptsComponent {
    /// The receipts for refundable purchases made at a vendor.
    #[serde(rename = "receipts", skip_serializing_if = "Option::is_none")]
    pub receipts: Option<Vec<crate::models::DestinyPeriodVendorsPeriodDestinyVendorReceipt>>,
}

impl DestinyPeriodEntitiesPeriodProfilesPeriodDestinyVendorReceiptsComponent {
    /// For now, this isn't used for much: it's a record of the recent refundable purchases that the user has made. In the future, it could be used for providing refunds/buyback via the API. Wouldn't that be fun?
    pub fn new() -> DestinyPeriodEntitiesPeriodProfilesPeriodDestinyVendorReceiptsComponent {
        DestinyPeriodEntitiesPeriodProfilesPeriodDestinyVendorReceiptsComponent {
            receipts: None,
        }
    }
}


