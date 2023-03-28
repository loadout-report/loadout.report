use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use serde_repr::{Serialize_repr, Deserialize_repr};
use serde_with::{serde_as, DisplayFromStr};

/// The most essential summary information about a Profile (in Destiny 1, we called these "Accounts").
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DestinyProfileComponent {

    /// If populated, this is a reference to the event card that is currently active.
    pub active_event_card_hash: Option<crate::id::Id<crate::generated::models::destiny::definitions::seasons::DestinyEventCardDefinition>>,
    /// A list of the character IDs, for further querying on your part.
    pub character_ids: i32,
    /// The 'current' Guardian Rank value, which starts at rank 1.
    pub current_guardian_rank: crate::id::Id<crate::generated::models::destiny::definitions::guardian_ranks::DestinyGuardianRankDefinition>,
    /// If populated, this is a reference to the season that is currently active.
    pub current_season_hash: Option<crate::id::Id<crate::generated::models::destiny::definitions::seasons::DestinySeasonDefinition>>,
    /// If populated, this is the reward power cap for the current season.
    pub current_season_reward_power_cap: Option<i32>,
    /// The last time the user played with any character on this Profile.
    pub date_last_played: chrono::DateTime<chrono::Utc>,
    /// A list of hashes for event cards that a profile owns. Unlike most values in versionsOwned, these stay with the profile across all platforms.
    pub event_card_hashes_owned: i32,
    /// The 'lifetime highest' Guardian Rank value, which starts at rank 1.
    pub lifetime_highest_guardian_rank: crate::id::Id<crate::generated::models::destiny::definitions::guardian_ranks::DestinyGuardianRankDefinition>,
    /// A list of seasons that this profile owns. Unlike versionsOwned, these stay with the profile across Platforms, and thus will be valid.
///  It turns out that Stadia Pro subscriptions will give access to seasons but only while playing on Stadia and with an active subscription. So some users (users who have Stadia Pro but choose to play on some other platform) won't see these as available: it will be whatever seasons are available for the platform on which they last played.
    pub season_hashes: i32,
    /// If you need to render the Profile (their platform name, icon, etc...) somewhere, this property contains that information.
    pub user_info: crate::generated::models::user::UserInfoCard,
    /// If you want to know what expansions they own, this will contain that data.
///  IMPORTANT: This field may not return the data you're interested in for Cross-Saved users. It returns the last ownership data we saw for this account - which is to say, what they've purchased on the platform on which they last played, which now could be a different platform.
///  If you don't care about per-platform ownership and only care about whatever platform it seems they are playing on most recently, then this should be "good enough." Otherwise, this should be considered deprecated. We do not have a good alternative to provide at this time with platform specific ownership data for DLC.
    pub versions_owned: crate::generated::models::destiny::DestinyGameVersions,
}

/// For now, this isn't used for much: it's a record of the recent refundable purchases that the user has made. In the future, it could be used for providing refunds/buyback via the API. Wouldn't that be fun?
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DestinyVendorReceiptsComponent {

    /// The receipts for refundable purchases made at a vendor.
    pub receipts: i32,
}
