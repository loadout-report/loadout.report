use crate::destiny::definitions::Hashable;
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
pub mod config;

#[derive(Copy, Clone, Serialize, Deserialize, Debug, PartialOrd, PartialEq, Ord, Eq, Hash)]
pub struct Hash(pub u32);

#[macro_export]
macro_rules! declare_hash_types {
    ( $( $name:ident ),* ) => {
        $(
            #[derive(Copy, Clone, Serialize, Deserialize, Debug, PartialOrd, PartialEq, Ord, Eq, Hash)]
            #[serde(transparent)]
            pub struct $name(pub Hash);
            impl Hashable for $name {}
        )*
    };
}

declare_hash_types![
    FactionHash,
    ItemHash,
    ProgressionHash,
    SeasonHash,
    CollectibleHash,
    LoreHash,
    ItemCategoryHash,
    BreakerTypeHash,
    DamageTypeHash,
    TraitHash,
    CooldownHash,
    ProgressionMappingHash,
    SocketTypeHash,
    MaterialRequirementSetHash,
    SocketCategoryHash,
    PlugCategoryHash,
    ObjectiveHash,
    InventoryBucketHash,
    ItemTierTypeHash,
    StatHash,
    StatGroupHash,
    LabelHash,
    EquipmentSlotHash,
    SandboxPatternHash,
    ChannelHash,
    DyeHash,
    ArtArrangementHash,
    ClassHash,
    VendorHash,
    ArtifactHash,
    InfusionCategoryHash,
    ProgressionLevelRequirementHash,
    PowerCapHash,
    RewardSourceHash,
    ActivityHash,
    QuestTypeHash,
    PresentationNodeHash,
    EnergyTypeHash,
    PlugSetHash,
    TalentGridHash,
    PerkHash
];

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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SocketVisibility {
    Visible = 0,
    Hidden = 1,
    HiddenWhenEmpty = 2,
    HiddenIfNoPlugsAvailable = 3,
}

/// Represents the possible and known UI styles used by the game for rendering Socket Categories.
#[derive(Clone, Serialize, Deserialize, Debug)]
pub enum SocketCategoryStyle {
    Unknown = 0,
    Reusable = 1,
    Consumable = 2,
    Unlockable = 3,
    Intrinsic = 4,
    EnergyMeter = 5,
    LargePerk = 6,
    Abilities = 7,
    Supers = 8,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TierType {
    Unknown = 0,
    Currency = 1,
    Basic = 2,
    Common = 3,
    Rare = 4,
    Superior = 5,
    Exotic = 6,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BucketScope {
    Character = 0,
    Account = 1,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BucketCategory {
    Invisible = 0,
    Item = 1,
    Currency = 2,
    Equippable = 3,
    Ignored = 4,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ItemLocation {
    Unknown = 0,
    Inventory = 1,
    Vault = 2,
    Vendor = 3,
    Postmaster = 4,
}

#[bitflags]
#[repr(u32)]
#[derive(Copy, Debug, Clone, Serialize, Deserialize)]
pub enum EquippingItemBlockAttributes {
    EquipOnAcquire = 1,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AmmunitionType {
    Primary = 1,
    Special = 2,
    Heavy = 3,
    Unknown = 4,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DyeReference {
    pub channel_hash: ChannelHash,
    pub dye_hash: DyeHash,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Class {
    Titan = 0,
    Hunter = 1,
    Warlock = 2,
    Unknown = 3,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Gender {
    Male = 0,
    Female = 1,
    Unknown = 2,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DamageType {
    Kinetic = 1,
    Arc = 2,
    Thermal = 3,
    Void = 4,
    Raid = 5,
    Stasis = 6,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ItemSubType {
    #[deprecated(note = "Items can be both 'Crucible' and something else interesting.")]
    Crucible = 1,
    #[deprecated(note = "Items can be both 'Vanguard' and something else interesting.")]
    Vanguard = 2,
    #[deprecated(note = "Items can be both Exotic and something else.")]
    Exotic = 5,
    AutoRifle = 6,
    Shotgun = 7,
    Machinegun = 8,
    HandCannon = 9,
    RocketLauncher = 10,
    FusionRifle = 11,
    SniperRifle = 12,
    PulseRifle = 13,
    ScoutRifle = 14,
    #[deprecated(note = "Items can be both 'CRM' and something else.")]
    Crm = 16,
    Sidearm = 17,
    Sword = 18,
    Mask = 19,
    Shader = 20,
    Ornament = 21,
    FusionRifleLine = 22,
    GrenadeLauncher = 23,
    Submachinegun = 24,
    TraceRifle = 25,
    HelmetArmor = 26,
    GauntletsArmor = 27,
    ChestArmor = 28,
    LegArmor = 29,
    ClassArmor = 30,
    Bow = 31,
    DummyRepeatableBounty = 32,
    Glaive = 33,
}

/// If the plug has a specific custom style, this enumeration will represent that style/those styles.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PlugUiStyles {
    None = 0,
    Masterwork = 1,
}

/// This enum determines whether the plug is available to be inserted.
/// - Normal means that all existing rules for plug insertion apply.
/// - UnavailableIfSocketContainsMatchingPlugCategory means that the plug is only available if the socket does NOT match the plug category.
/// - AvailableIfSocketContainsMatchingPlugCategory means that the plug is only available if the socket DOES match the plug category.
///
/// For category matching, use the plug's "plugCategoryIdentifier" property, comparing it to
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PlugAvailabilityMode {
    Normal = 0,
    UnavailableIfSocketContainsMatchingPlugCategory = 1,
    AvailableIfSocketContainsMatchingPlugCategory = 2,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EnergyType {
    Any = 0,
    Arc = 1,
    Thermal = 2,
    Void = 3,
    Ghost = 4,
    Subclass = 5,
    Stasis = 6,
}

/// Indicates how a socket is populated, and where you should look for valid plug data.
/// This is a flags enumeration/bitmask field, as you may have to look in multiple sources across multiple components for valid plugs.
/// For instance, a socket could have plugs that are sourced from its own definition, as well as plugs that are sourced from Character-scoped AND profile-scoped Plug Sets. Only by combining plug data for every indicated source will you be able to know all of the plugs available for a socket.
#[bitflags]
#[repr(u32)]
#[derive(Copy, Debug, Clone, Serialize, Deserialize)]
pub enum SocketPlugSource {
    /// If there's no way we can detect to insert new plugs.
    // None = 0,
    /// Use plugs found in the player's inventory, based on the socket type rules (see DestinySocketTypeDefinition for more info)
    ///
    /// Note that a socket - like Shaders - can have *both* reusable plugs and inventory items inserted theoretically.
    InventorySourced = 1,
    /// Use the DestinyItemSocketsComponent.sockets.reusablePlugs property to determine which plugs are valid for this socket. This may have to be combined with other sources, such as plug sets, if those flags are set.
    ///
    /// Note that "Reusable" plugs may not necessarily come from a plug set, nor from the "reusablePlugItems" in the socket's Definition data. They can sometimes be "randomized" in which case the only source of truth at the moment is still the runtime DestinyItemSocketsComponent.sockets.reusablePlugs property.
    ReusableSourced = 2,
    /// Use the ProfilePlugSets (DestinyProfileResponse.profilePlugSets) component data to determine which plugs are valid for this socket.
    ProfilePlugSet = 4,
    /// Use the CharacterPlugSets (DestinyProfileResponse.characterPlugSets) component data to determine which plugs are valid for this socket.
    CharacterPlugSet = 8,
}

/// Indicates how a perk should be shown, or if it should be, in the game UI. Maybe useful for those of you trying to filter out internal-use-only perks (or for those of you trying to figure out what they do!)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ItemPerkVisibility {
    Visible = 0,
    Disabled = 1,
    Hidden = 2,
}

/// As you run into items that need to be classified for Milestone purposes in ways that we cannot infer via direct data, add a new classification here and use a string constant to represent it in the local item config file.
///
/// NOTE: This is not all of the item types available, and some of these are holdovers from Destiny 1 that may or may not still exist.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SpecialItemType {
    SpecialCurrency = 0,
    Armor = 8,
    Weapon = 9,
    Engram = 23,
    Consumable = 24,
    ExchangeMaterial = 25,
    MissionReward = 27,
    Currency = 29,
}

/// An enumeration that indicates the high-level "type" of the item, attempting to iron out the context specific differences for specific instances of an entity. For instance, though a weapon may be of various weapon "Types", in DestinyItemType they are all classified as "Weapon". This allows for better filtering on a higher level of abstraction for the concept of types.
///
/// This enum is provided for historical compatibility with Destiny 1, but an ideal alternative is to use DestinyItemCategoryDefinitions and the DestinyItemDefinition.itemCategories property instead. Item Categories allow for arbitrary hierarchies of specificity, and for items to belong to multiple categories across multiple hierarchies simultaneously. For this enum, we pick a single type as a "best guess" fit.
///
/// NOTE: This is not all of the item types available, and some of these are holdovers from Destiny 1 that may or may not still exist.
///
/// I keep updating these because they're so damn convenient. I guess I shouldn't fight it.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ItemType {
    Currency = 1,
    Armor = 2,
    Weapon = 3,
    Message = 7,
    Engram = 8,
    Consumable = 9,
    ExchangeMaterial = 10,
    MissionReward = 11,
    QuestStep = 12,
    QuestStepComplete = 13,
    Emblem = 14,
    Quest = 15,
    Subclass = 16,
    ClanBanner = 17,
    Aura = 18,
    Mod = 19,
    Dummy = 20,
    Ship = 21,
    Vehicle = 22,
    Emote = 23,
    Ghost = 24,
    Package = 25,
    Bounty = 26,
    Wrapper = 27,
    SeasonalArtifact = 28,
    Finisher = 29,
    Pattern = 30,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BreakerType {
    ShieldPiercing = 1,
    Disruption = 2,
    Stagger = 3,
}

/// Represents the different kinds of acquisition behavior for progression reward items.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ProgressionRewardItemAcquisitionBehavior {
    Instant = 0,
    PlayerClaimRequired = 1,
}

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
// *DestinyProgressionRewardItemAcquisitionBehavior
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
