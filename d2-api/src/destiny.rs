use enumflags2::{bitflags, BitFlags};
use serde::{Deserialize, Serialize};

pub mod activities;
pub mod advanced;
pub mod artifacts;
pub mod challenges;
pub mod character;
pub mod components;
pub mod constants;
pub mod definitions;
pub mod entities;
pub mod historical_stats;
pub mod milestones;
pub mod misc;
pub mod perks;
pub mod progression;
pub mod quests;
pub mod reporting;
pub mod requests;
pub mod responses;
pub mod sockets;
pub mod vendors;

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct FactionHash(pub Hash);
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Hash(pub u32);
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct ItemHash(pub Hash);
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct ProgressionHash(pub Hash);
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct SeasonHash(pub Hash);

// Entities
/// Information about a current character's status with a Progression.
/// A progression is a value that can increase with activity and has levels.
/// Think Character Level and Reputation Levels.
///
/// Combine this "live" data with the related DestinyProgressionDefinition
/// for a full picture of the Progression.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct Progression {
    /// The hash identifier of the Progression in question.
    /// Use it to look up the DestinyProgressionDefinition in static data.
    pub progression_hash: ProgressionHash,
    /// The amount of progress earned today for this progression.
    pub daily_progress: i32,
    /// If this progression has a daily limit, this is that limit.
    pub daily_limit: Option<i32>,
    /// The amount of progress earned toward this progression in the current week.
    pub weekly_progress: i32,
    /// If this progression has a weekly limit, this is that limit.
    pub weekly_limit: Option<i32>,
    /// This is the total amount of progress obtained overall for this progression
    /// (for instance, the total amount of Character Level experience earned)
    pub current_progress: i32,
    /// This is the level of the progression (for instance, the Character Level).
    pub level: i32,
    /// This is the maximum possible level you can achieve for this progression
    /// (for example, the maximum character level obtainable)
    pub level_cap: i32,
    /// Progressions define their levels in "steps". Since the last step may be repeatable,
    /// the user may be at a higher level than the actual Step achieved in the progression.
    /// Not necessarily useful, but potentially interesting for those cruising the API.
    /// Relate this to the "steps" property of the DestinyProgression to see
    /// which step the user is on, if you care about that.
    /// (Note that this is Content Version dependent since it refers to indexes.)
    pub step_index: i32,
    /// The amount of progression (i.e. "Experience") needed to reach the next level of
    /// this Progression. Jeez, progression is such an overloaded word.
    pub progress_to_next_level: i32,
    /// The total amount of progression (i.e. "Experience") needed in order to reach the next level.
    pub next_level_at: i32,
    /// The number of resets of this progression you've executed this season,
    /// if applicable to this progression.
    pub current_reset_count: Option<i32>,
    /// Information about historical resets of this progression, if there is any data for it.
    pub season_resets: Vec<ProgressionResetEntry>,
    /// Information about historical rewards for this progression, if there is any data for it.
    // todo: is option needed here?
    pub reward_item_states: Vec<BitFlags<ProgressionRewardItemState>>,
}

/// Represents a season and the number of resets you had in that season.
///
/// We do not necessarily - even for progressions with resets - track it over all seasons.
/// So be careful and check the season numbers being returned.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct ProgressionResetEntry {
    // todo: is this a hash?
    pub season: SeasonHash,
    pub resets: i32,
}

#[bitflags]
#[repr(u32)]
#[derive(Copy, Debug, Clone, Serialize, Deserialize)]
pub enum ProgressionRewardItemState {
    Invisible = 1,
    Earned = 2,
    Claimed = 4,
    ClaimAllowed = 8,
}

/// There are many Progressions in Destiny (think Character Level, or Reputation).
/// These are the various "Scopes" of Progressions, which affect many things:
///   * Where/if they are stored
///   * How they are calculated
///   * Where they can be used in other game logic
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ProgressionScope {
    Account = 0,
    Character = 1,
    Clan = 2,
    Item = 3,
    ImplicitFromEquipment = 4,
    Mapped = 5,
    MappedAggregate = 6,
    MappedStat = 7,
    MappedUnlockValue = 8,
}

/// If progression is earned, this determines whether the progression shows visual effects
/// on the character or its item - or neither.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ProgressionStepDisplayEffect {
    None = 0,
    Character = 1,
    Item = 2,
}

/// Used in a number of Destiny contracts to return data about an item stack and its quantity.
/// Can optionally return an itemInstanceId if the item is instanced
/// - in which case, the quantity returned will be 1. If it's not... uh, let me know okay?
/// Thanks.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct ItemQuantity {
    /// The hash identifier for the item in question. Use it to look up the item's DestinyInventoryItemDefinition.
    pub item_hash: ItemHash,
    /// If this quantity is referring to a specific instance of an item, this will have the item's
    /// instance ID. Normally, this will be null.
    pub item_instance_id: Option<i64>,
    /// The amount of the item needed/available depending on the context of where
    /// DestinyItemQuantity is being used.
    pub quantity: i32,
    /// Indicates that this item quantity may be conditionally shown or hidden,
    /// based on various sources of state.
    /// For example: server flags, account state, or character progress.
    pub has_conditional_visibility: bool,
}

// DestinyItemQuantity
// SocketTypeActionType
// DestinySocketVisibility
// DestinySocketCategoryStyle
// TierType
// BucketScope
// BucketCategory
// ItemLocation
// DestinyStatAggregationType
// DestinyStatCategory
// EquippingItemBlockAttributes
// DestinyAmmunitionType
// DyeReference
// DestinyClass
// DestinyGender
// DestinyVendorProgressionType
// VendorDisplayCategorySortOrder
// DestinyVendorInteractionRewardSelection
// DestinyVendorReplyType
// VendorInteractionType
// DestinyItemSortType
// DestinyVendorItemRefundPolicy
// DestinyGatingScope
// ActivityGraphNodeHighlightType
// DestinyUnlockValueUIStyle
// DestinyObjectiveGrantStyle
// DamageType
// DestinyObjectiveUiStyle
// DestinyActivityNavPointType
// DestinyActivityModeCategory
// DestinyItemSubType
// DestinyGraphNodeState
// DestinyPresentationNodeType
// DestinyScope
// DestinyPresentationDisplayStyle
// DestinyRecordValueStyle
// DestinyRecordToastStyle
// DestinyPresentationScreenStyle
// PlugUiStyles
// PlugAvailabilityMode
// DestinyEnergyType
// SocketPlugSources
// ItemPerkVisibility
// SpecialItemType
// DestinyItemType
// DestinyBreakerType
// DestinyProgressionRewardItemAcquisitionBehavior
// ItemBindStatus
// TransferStatuses
// ItemState
// DestinyGameVersions
// DestinyComponentType
// DestinyPresentationNodeState
// DestinyRecordState
// DestinyCollectibleState
// DestinyPartyMemberStates
// DestinyGamePrivacySetting
// DestinyJoinClosedReasons
// DestinyRace
// DestinyActivity
// DestinyActivityDifficultyTier
// DestinyStat
// EquipFailureReason
// DestinyTalentNode
// DestinyTalentNodeState
// DestinyTalentNodeStatBlock
// DestinyVendorFilter
// VendorItemStatus
// DestinyUnlockStatus
// DestinyVendorItemState
// DestinyEquipItemResults
// DestinyEquipItemResult
