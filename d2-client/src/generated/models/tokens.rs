use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use serde_repr::{Serialize_repr, Deserialize_repr};
use serde_with::{serde_as, DisplayFromStr};

/// No documentation provided.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BungieRewardDisplay {

    /// No documentation provided.
    pub objective_display_properties: crate::generated::models::tokens::RewardDisplayProperties,
    /// No documentation provided.
    pub reward_display_properties: crate::generated::models::tokens::RewardDisplayProperties,
    /// No documentation provided.
    pub user_reward_availability_model: crate::generated::models::tokens::UserRewardAvailabilityModel,
}

/// No documentation provided.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CollectibleDefinitions {

    /// No documentation provided.
    pub collectible_definition: crate::generated::models::destiny::definitions::collectibles::DestinyCollectibleDefinition,
    /// No documentation provided.
    pub destiny_inventory_item_definition: crate::generated::models::destiny::definitions::DestinyInventoryItemDefinition,
}

/// No documentation provided.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PartnerOfferClaimRequest {

    /// No documentation provided.
    #[serde(with = "crate::unfuck_js::stringified_numbers")]
    pub bungie_net_membership_id: i64,
    /// No documentation provided.
    pub partner_offer_id: String,
    /// No documentation provided.
    pub transaction_id: String,
}

/// No documentation provided.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PartnerOfferHistoryResponse {

    /// No documentation provided.
    pub apply_date: Option<chrono::DateTime<chrono::Utc>>,
    /// No documentation provided.
    pub is_consumable: bool,
    /// No documentation provided.
    pub localized_description: String,
    /// No documentation provided.
    pub localized_name: String,
    /// No documentation provided.
    #[serde(with = "crate::unfuck_js::nullable_stringified_numbers")]
    pub membership_id: Option<i64>,
    /// No documentation provided.
    pub membership_type: Option<i32>,
    /// No documentation provided.
    pub partner_offer_key: String,
    /// No documentation provided.
    pub quantity_applied: i32,
}

/// No documentation provided.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PartnerOfferSkuHistoryResponse {

    /// No documentation provided.
    pub all_offers_applied: bool,
    /// No documentation provided.
    pub claim_date: chrono::DateTime<chrono::Utc>,
    /// No documentation provided.
    pub localized_description: String,
    /// No documentation provided.
    pub localized_name: String,
    /// No documentation provided.
    pub sku_identifier: String,
    /// No documentation provided.
    pub sku_offers: Vec<crate::generated::models::tokens::PartnerOfferHistoryResponse>,
    /// No documentation provided.
    pub transaction_id: String,
}

/// No documentation provided.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PartnerRewardHistoryResponse {

    /// No documentation provided.
    pub partner_offers: Vec<crate::generated::models::tokens::PartnerOfferSkuHistoryResponse>,
    /// No documentation provided.
    pub twitch_drops: Vec<crate::generated::models::tokens::TwitchDropHistoryResponse>,
}

/// No documentation provided.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RewardAvailabilityModel {

    /// No documentation provided.
    pub collectible_definitions: Vec<crate::generated::models::tokens::CollectibleDefinitions>,
    /// No documentation provided.
    pub decrypted_token: String,
    /// No documentation provided.
    pub game_earn_by_date: chrono::DateTime<chrono::Utc>,
    /// No documentation provided.
    pub has_existing_code: bool,
    /// No documentation provided.
    pub has_offer: bool,
    /// No documentation provided.
    pub is_loyalty_reward: bool,
    /// No documentation provided.
    pub is_offer: bool,
    /// No documentation provided.
    pub offer_applied: bool,
    /// No documentation provided.
    pub record_definitions: Vec<crate::generated::models::destiny::definitions::records::DestinyRecordDefinition>,
    /// No documentation provided.
    pub redemption_end_date: chrono::DateTime<chrono::Utc>,
    /// No documentation provided.
    pub shopify_end_date: Option<chrono::DateTime<chrono::Utc>>,
}

/// No documentation provided.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RewardDisplayProperties {

    /// No documentation provided.
    pub description: String,
    /// No documentation provided.
    pub image_path: String,
    /// No documentation provided.
    pub name: String,
}

/// No documentation provided.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TwitchDropHistoryResponse {

    /// No documentation provided.
    pub claim_state: Option<u8>,
    /// No documentation provided.
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    /// No documentation provided.
    pub description: String,
    /// No documentation provided.
    pub title: String,
}

/// No documentation provided.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UserRewardAvailabilityModel {

    /// No documentation provided.
    pub availability_model: crate::generated::models::tokens::RewardAvailabilityModel,
    /// No documentation provided.
    pub is_available_for_user: bool,
    /// No documentation provided.
    pub is_unlocked_for_user: bool,
}