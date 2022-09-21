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
pub struct TokensPeriodPartnerOfferClaimRequest {
    #[serde(rename = "PartnerOfferId", skip_serializing_if = "Option::is_none")]
    pub partner_offer_id: Option<String>,
    #[serde(rename = "BungieNetMembershipId", skip_serializing_if = "Option::is_none")]
    pub bungie_net_membership_id: Option<i64>,
    #[serde(rename = "TransactionId", skip_serializing_if = "Option::is_none")]
    pub transaction_id: Option<String>,
}

impl TokensPeriodPartnerOfferClaimRequest {
    pub fn new() -> TokensPeriodPartnerOfferClaimRequest {
        TokensPeriodPartnerOfferClaimRequest {
            partner_offer_id: None,
            bungie_net_membership_id: None,
            transaction_id: None,
        }
    }
}


