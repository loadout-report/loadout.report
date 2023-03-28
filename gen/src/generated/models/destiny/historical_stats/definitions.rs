use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use serde_repr::{Serialize_repr, Deserialize_repr};
use serde_with::{serde_as, DisplayFromStr};

/// For historical reasons, this list will have both D1 and D2-relevant Activity Modes in it. Please don't take this to mean that some D1-only feature is coming back!
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize_repr, Deserialize_repr)]
#[repr(i32)]
pub enum DestinyActivityModeType {
    /// No documentation provided.
    None = 0,
    /// No documentation provided.
    Story = 2,
    /// No documentation provided.
    Strike = 3,
    /// No documentation provided.
    Raid = 4,
    /// No documentation provided.
    AllPvP = 5,
    /// No documentation provided.
    Patrol = 6,
    /// No documentation provided.
    AllPvE = 7,
    /// No documentation provided.
    Reserved9 = 9,
    /// No documentation provided.
    Control = 10,
    /// No documentation provided.
    Reserved11 = 11,
    /// Clash -> Destiny's name for Team Deathmatch. 4v4 combat, the team with the highest kills at the end of time wins.
    Clash = 12,
    /// No documentation provided.
    Reserved13 = 13,
    /// No documentation provided.
    CrimsonDoubles = 15,
    /// No documentation provided.
    Nightfall = 16,
    /// No documentation provided.
    HeroicNightfall = 17,
    /// No documentation provided.
    AllStrikes = 18,
    /// No documentation provided.
    IronBanner = 19,
    /// No documentation provided.
    Reserved20 = 20,
    /// No documentation provided.
    Reserved21 = 21,
    /// No documentation provided.
    Reserved22 = 22,
    /// No documentation provided.
    Reserved24 = 24,
    /// No documentation provided.
    AllMayhem = 25,
    /// No documentation provided.
    Reserved26 = 26,
    /// No documentation provided.
    Reserved27 = 27,
    /// No documentation provided.
    Reserved28 = 28,
    /// No documentation provided.
    Reserved29 = 29,
    /// No documentation provided.
    Reserved30 = 30,
    /// No documentation provided.
    Supremacy = 31,
    /// No documentation provided.
    PrivateMatchesAll = 32,
    /// No documentation provided.
    Survival = 37,
    /// No documentation provided.
    Countdown = 38,
    /// No documentation provided.
    TrialsOfTheNine = 39,
    /// No documentation provided.
    Social = 40,
    /// No documentation provided.
    TrialsCountdown = 41,
    /// No documentation provided.
    TrialsSurvival = 42,
    /// No documentation provided.
    IronBannerControl = 43,
    /// No documentation provided.
    IronBannerClash = 44,
    /// No documentation provided.
    IronBannerSupremacy = 45,
    /// No documentation provided.
    ScoredNightfall = 46,
    /// No documentation provided.
    ScoredHeroicNightfall = 47,
    /// No documentation provided.
    Rumble = 48,
    /// No documentation provided.
    AllDoubles = 49,
    /// No documentation provided.
    Doubles = 50,
    /// No documentation provided.
    PrivateMatchesClash = 51,
    /// No documentation provided.
    PrivateMatchesControl = 52,
    /// No documentation provided.
    PrivateMatchesSupremacy = 53,
    /// No documentation provided.
    PrivateMatchesCountdown = 54,
    /// No documentation provided.
    PrivateMatchesSurvival = 55,
    /// No documentation provided.
    PrivateMatchesMayhem = 56,
    /// No documentation provided.
    PrivateMatchesRumble = 57,
    /// No documentation provided.
    HeroicAdventure = 58,
    /// No documentation provided.
    Showdown = 59,
    /// No documentation provided.
    Lockdown = 60,
    /// No documentation provided.
    Scorched = 61,
    /// No documentation provided.
    ScorchedTeam = 62,
    /// No documentation provided.
    Gambit = 63,
    /// No documentation provided.
    AllPvECompetitive = 64,
    /// No documentation provided.
    Breakthrough = 65,
    /// No documentation provided.
    BlackArmoryRun = 66,
    /// No documentation provided.
    Salvage = 67,
    /// No documentation provided.
    IronBannerSalvage = 68,
    /// No documentation provided.
    PvPCompetitive = 69,
    /// No documentation provided.
    PvPQuickplay = 70,
    /// No documentation provided.
    ClashQuickplay = 71,
    /// No documentation provided.
    ClashCompetitive = 72,
    /// No documentation provided.
    ControlQuickplay = 73,
    /// No documentation provided.
    ControlCompetitive = 74,
    /// No documentation provided.
    GambitPrime = 75,
    /// No documentation provided.
    Reckoning = 76,
    /// No documentation provided.
    Menagerie = 77,
    /// No documentation provided.
    VexOffensive = 78,
    /// No documentation provided.
    NightmareHunt = 79,
    /// No documentation provided.
    Elimination = 80,
    /// No documentation provided.
    Momentum = 81,
    /// No documentation provided.
    Dungeon = 82,
    /// No documentation provided.
    Sundial = 83,
    /// No documentation provided.
    TrialsOfOsiris = 84,
    /// No documentation provided.
    Dares = 85,
    /// No documentation provided.
    Offensive = 86,
    /// No documentation provided.
    LostSector = 87,
    /// No documentation provided.
    Rift = 88,
    /// No documentation provided.
    ZoneControl = 89,
    /// No documentation provided.
    IronBannerRift = 90,
    /// No documentation provided.
    IronBannerZoneControl = 91,
}

/// No documentation provided.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DestinyHistoricalStatsDefinition {

    /// Category for the stat.
    pub category: crate::generated::models::destiny::historical_stats::definitions::DestinyStatsCategoryType,
    /// Statistic group
    pub group: crate::generated::models::destiny::historical_stats::definitions::DestinyStatsGroupType,
    /// Optional URI to an icon for the statistic
    pub icon_image: String,
    /// The tier associated with this medal - be it implicitly or explicitly.
    pub medal_tier_hash: Option<crate::id::Id<crate::generated::models::destiny::definitions::DestinyMedalTierDefinition>>,
    /// Optional icon for the statistic
    pub merge_method: Option<i32>,
    /// Game modes where this statistic can be reported.
    pub modes: i32,
    /// Time periods the statistic covers
    pub period_types: i32,
    /// Description of a stat if applicable.
    pub stat_description: String,
    /// Unique programmer friendly ID for this stat
    pub stat_id: String,
    /// Display name
    pub stat_name: String,
    /// Display name abbreviated
    pub stat_name_abbr: String,
    /// Localized Unit Name for the stat.
    pub unit_label: String,
    /// Unit, if any, for the statistic
    pub unit_type: crate::generated::models::destiny::historical_stats::definitions::UnitType,
    /// Weight assigned to this stat indicating its relative impressiveness.
    pub weight: i32,
}

/// No documentation provided.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize_repr, Deserialize_repr)]
#[repr(i32)]
pub enum DestinyStatsCategoryType {
    /// No documentation provided.
    None = 0,
    /// No documentation provided.
    Kills = 1,
    /// No documentation provided.
    Assists = 2,
    /// No documentation provided.
    Deaths = 3,
    /// No documentation provided.
    Criticals = 4,
    /// No documentation provided.
    KDa = 5,
    /// No documentation provided.
    KD = 6,
    /// No documentation provided.
    Score = 7,
    /// No documentation provided.
    Entered = 8,
    /// No documentation provided.
    TimePlayed = 9,
    /// No documentation provided.
    MedalWins = 10,
    /// No documentation provided.
    MedalGame = 11,
    /// No documentation provided.
    MedalSpecialKills = 12,
    /// No documentation provided.
    MedalSprees = 13,
    /// No documentation provided.
    MedalMultiKills = 14,
    /// No documentation provided.
    MedalAbilities = 15,
}

/// If the enum value is > 100, it is a "special" group that cannot be queried for directly (special cases apply to when they are returned, and are not relevant in general cases)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize_repr, Deserialize_repr)]
#[repr(i32)]
pub enum DestinyStatsGroupType {
    /// No documentation provided.
    None = 0,
    /// No documentation provided.
    General = 1,
    /// No documentation provided.
    Weapons = 2,
    /// No documentation provided.
    Medals = 3,
    /// This is purely to serve as the dividing line between filterable and un-filterable groups. Below this number is a group you can pass as a filter. Above it are groups used in very specific circumstances and not relevant for filtering.
    ReservedGroups = 100,
    /// Only applicable while generating leaderboards.
    Leaderboard = 101,
    /// These will *only* be consumed by GetAggregateStatsByActivity
    Activity = 102,
    /// These are only consumed and returned by GetUniqueWeaponHistory
    UniqueWeapon = 103,
    /// No documentation provided.
    Internal = 104,
}

/// No documentation provided.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize_repr, Deserialize_repr)]
#[repr(i32)]
pub enum DestinyStatsMergeMethod {
    /// When collapsing multiple instances of the stat together, add the values.
    Add = 0,
    /// When collapsing multiple instances of the stat together, take the lower value.
    Min = 1,
    /// When collapsing multiple instances of the stat together, take the higher value.
    Max = 2,
}

/// No documentation provided.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize_repr, Deserialize_repr)]
#[repr(i32)]
pub enum PeriodType {
    /// No documentation provided.
    None = 0,
    /// No documentation provided.
    Daily = 1,
    /// No documentation provided.
    AllTime = 2,
    /// No documentation provided.
    Activity = 3,
}

/// No documentation provided.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize_repr, Deserialize_repr)]
#[repr(i32)]
pub enum UnitType {
    /// No documentation provided.
    None = 0,
    /// Indicates the statistic is a simple count of something.
    Count = 1,
    /// Indicates the statistic is a per game average.
    PerGame = 2,
    /// Indicates the number of seconds
    Seconds = 3,
    /// Indicates the number of points earned
    Points = 4,
    /// Values represents a team ID
    Team = 5,
    /// Values represents a distance (units to-be-determined)
    Distance = 6,
    /// Ratio represented as a whole value from 0 to 100.
    Percent = 7,
    /// Ratio of something, shown with decimal places
    Ratio = 8,
    /// True or false
    Boolean = 9,
    /// The stat is actually a weapon type.
    WeaponType = 10,
    /// Indicates victory, defeat, or something in between.
    Standing = 11,
    /// Number of milliseconds some event spanned. For example, race time, or lap time.
    Milliseconds = 12,
    /// The value is a enumeration of the Completion Reason type.
    CompletionReason = 13,
}
