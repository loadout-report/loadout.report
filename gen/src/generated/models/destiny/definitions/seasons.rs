use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use serde_repr::{Serialize_repr, Deserialize_repr};
use serde_with::{serde_as, DisplayFromStr};

/// Defines the properties of an 'Event Card' in Destiny 2, to coincide with a seasonal event for additional challenges, premium rewards, a new seal, and a special title. For example: Solstice of Heroes 2022.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DestinyEventCardDefinition {

    /// No documentation provided.
    pub color: crate::generated::models::destiny::misc::DestinyColor,
    /// No documentation provided.
    pub display_properties: crate::generated::models::destiny::definitions::common::DestinyDisplayPropertiesDefinition,
    /// No documentation provided.
    #[serde(with = "crate::unfuck_js::stringified_numbers")]
    pub end_time: i64,
    /// The unique identifier for this entity. Guaranteed to be unique for the type of entity, but not globally.
/// When entities refer to each other in Destiny content, it is this hash that they are referring to.
    pub hash: u32,
    /// No documentation provided.
    pub images: crate::generated::models::destiny::definitions::seasons::DestinyEventCardImages,
    /// The index of the entity as it was found in the investment tables.
    pub index: i32,
    /// No documentation provided.
    pub link_redirect_path: String,
    /// If this is true, then there is an entity with this identifier/type combination, but BNet is not yet allowed to show it. Sorry!
    pub redacted: bool,
    /// No documentation provided.
    pub seal_presentation_node_hash: crate::id::Id<crate::generated::models::destiny::definitions::presentation::DestinyPresentationNodeDefinition>,
    /// No documentation provided.
    pub ticket_currency_item_hash: crate::id::Id<crate::generated::models::destiny::definitions::DestinyInventoryItemDefinition>,
    /// No documentation provided.
    pub ticket_vendor_category_hash: u32,
    /// No documentation provided.
    pub ticket_vendor_hash: crate::id::Id<crate::generated::models::destiny::definitions::DestinyVendorDefinition>,
    /// No documentation provided.
    pub triumphs_presentation_node_hash: crate::id::Id<crate::generated::models::destiny::definitions::presentation::DestinyPresentationNodeDefinition>,
}

/// No documentation provided.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DestinyEventCardImages {

    /// No documentation provided.
    pub card_complete_image_path: String,
    /// No documentation provided.
    pub card_complete_wrap_image_path: String,
    /// No documentation provided.
    pub card_incomplete_image_path: String,
    /// No documentation provided.
    pub progress_icon_image_path: String,
    /// No documentation provided.
    pub theme_background_image_path: String,
    /// No documentation provided.
    pub unowned_card_sleeve_image_path: String,
    /// No documentation provided.
    pub unowned_card_sleeve_wrap_image_path: String,
}

/// Defines a canonical "Season" of Destiny: a range of a few months where the game highlights certain challenges, provides new loot, has new Clan-related rewards and celebrates various seasonal events.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DestinySeasonDefinition {

    /// No documentation provided.
    pub artifact_item_hash: Option<crate::id::Id<crate::generated::models::destiny::definitions::DestinyInventoryItemDefinition>>,
    /// No documentation provided.
    pub background_image_path: String,
    /// No documentation provided.
    pub display_properties: crate::generated::models::destiny::definitions::common::DestinyDisplayPropertiesDefinition,
    /// No documentation provided.
    pub end_date: Option<chrono::DateTime<chrono::Utc>>,
    /// The unique identifier for this entity. Guaranteed to be unique for the type of entity, but not globally.
/// When entities refer to each other in Destiny content, it is this hash that they are referring to.
    pub hash: u32,
    /// The index of the entity as it was found in the investment tables.
    pub index: i32,
    /// Optional - Defines the promotional text, images, and links to preview this season.
    pub preview: crate::generated::models::destiny::definitions::seasons::DestinySeasonPreviewDefinition,
    /// If this is true, then there is an entity with this identifier/type combination, but BNet is not yet allowed to show it. Sorry!
    pub redacted: bool,
    /// No documentation provided.
    pub seal_presentation_node_hash: Option<crate::id::Id<crate::generated::models::destiny::definitions::presentation::DestinyPresentationNodeDefinition>>,
    /// No documentation provided.
    pub season_number: i32,
    /// No documentation provided.
    pub season_pass_hash: Option<crate::id::Id<crate::generated::models::destiny::definitions::seasons::DestinySeasonPassDefinition>>,
    /// No documentation provided.
    pub season_pass_progression_hash: Option<crate::id::Id<crate::generated::models::destiny::definitions::DestinyProgressionDefinition>>,
    /// No documentation provided.
    pub seasonal_challenges_presentation_node_hash: Option<crate::id::Id<crate::generated::models::destiny::definitions::presentation::DestinyPresentationNodeDefinition>>,
    /// No documentation provided.
    pub start_date: Option<chrono::DateTime<chrono::Utc>>,
}

/// No documentation provided.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DestinySeasonPassDefinition {

    /// No documentation provided.
    pub display_properties: crate::generated::models::destiny::definitions::common::DestinyDisplayPropertiesDefinition,
    /// The unique identifier for this entity. Guaranteed to be unique for the type of entity, but not globally.
/// When entities refer to each other in Destiny content, it is this hash that they are referring to.
    pub hash: u32,
    /// The index of the entity as it was found in the investment tables.
    pub index: i32,
    /// I know what you're thinking, but I promise we're not going to duplicate and drown you. Instead, we're giving you sweet, sweet power bonuses.
///  Prestige progression is further progression that you can make on the Season pass after you gain max ranks, that will ultimately increase your power/light level over the theoretical limit.
    pub prestige_progression_hash: crate::id::Id<crate::generated::models::destiny::definitions::DestinyProgressionDefinition>,
    /// If this is true, then there is an entity with this identifier/type combination, but BNet is not yet allowed to show it. Sorry!
    pub redacted: bool,
    /// This is the progression definition related to the progression for the initial levels 1-100 that provide item rewards for the Season pass. Further experience after you reach the limit is provided in the "Prestige" progression referred to by prestigeProgressionHash.
    pub reward_progression_hash: crate::id::Id<crate::generated::models::destiny::definitions::DestinyProgressionDefinition>,
}

/// Defines the promotional text, images, and links to preview this season.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DestinySeasonPreviewDefinition {

    /// A localized description of the season.
    pub description: String,
    /// A list of images to preview the seasonal content. Should have at least three to show.
    pub images: i32,
    /// A relative path to learn more about the season. Web browsers should be automatically redirected to the user's Bungie.net locale. For example: "/SeasonOfTheChosen" will redirect to "/7/en/Seasons/SeasonOfTheChosen" for English users.
    pub link_path: String,
    /// An optional link to a localized video, probably YouTube.
    pub video_link: String,
}

/// Defines the thumbnail icon, high-res image, and video link for promotional images
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DestinySeasonPreviewImageDefinition {

    /// An optional path to a high-resolution image, probably 1920x1080.
    pub high_res_image: String,
    /// A thumbnail icon path to preview seasonal content, probably 480x270.
    pub thumbnail_image: String,
}
