use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use serde_repr::{Serialize_repr, Deserialize_repr};
use serde_with::{serde_as, DisplayFromStr};

/// The set of progression-related information that applies at a Profile-wide level for your Destiny experience. This differs from the Jimi Hendrix Experience because there's less guitars on fire. Yet. #spoileralert?
/// This will include information such as Checklist info.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DestinyProfileProgressionComponent {

    /// The set of checklists that can be examined on a profile-wide basis, keyed by the hash identifier of the Checklist (DestinyChecklistDefinition)
/// For each checklist returned, its value is itself a Dictionary keyed by the checklist's hash identifier with the value being a boolean indicating if it's been discovered yet.
    pub checklists: HashMap<u32, HashMap<u32, bool>>,
    /// Data related to your progress on the current season's artifact that is the same across characters.
    pub seasonal_artifact: crate::generated::models::destiny::artifacts::DestinyArtifactProfileScoped,
}

/// This is an experimental set of data that Bungie considers to be "transitory" - information that may be useful for API users, but that is coming from a non-authoritative data source about information that could potentially change at a more frequent pace than Bungie.net will receive updates about it.
/// This information is provided exclusively for convenience should any of it be useful to users: we provide no guarantees to the accuracy or timeliness of data that comes from this source. Know that this data can potentially be out-of-date or even wrong entirely if the user disconnected from the game or suddenly changed their status before we can receive refreshed data.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DestinyProfileTransitoryComponent {

    /// If you are in an activity, this is some transitory info about the activity currently being played.
    pub current_activity: crate::generated::models::destiny::components::profiles::DestinyProfileTransitoryCurrentActivity,
    /// Information about whether and what might prevent you from joining this person on a fireteam.
    pub joinability: crate::generated::models::destiny::components::profiles::DestinyProfileTransitoryJoinability,
    /// The hash identifier for the DestinyDestinationDefinition of the last location you were orbiting when in orbit.
    pub last_orbited_destination_hash: Option<crate::id::Id<crate::generated::models::destiny::definitions::DestinyDestinationDefinition>>,
    /// If you have any members currently in your party, this is some (very) bare-bones information about those members.
    pub party_members: Vec<crate::generated::models::destiny::components::profiles::DestinyProfileTransitoryPartyMember>,
    /// Information about tracked entities.
    pub tracking: Vec<crate::generated::models::destiny::components::profiles::DestinyProfileTransitoryTrackingEntry>,
}

/// If you are playing in an activity, this is some information about it.
/// Note that we cannot guarantee any of this resembles what ends up in the PGCR in any way. They are sourced by two entirely separate systems with their own logic, and the one we source this data from should be considered non-authoritative in comparison.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DestinyProfileTransitoryCurrentActivity {

    /// If you're still in it but it "ended" (like when folks are dancing around the loot after they beat a boss), this is when the activity ended.
    pub end_time: Option<chrono::DateTime<chrono::Utc>>,
    /// If you have human opponents, this is the highest opposing team's score.
    pub highest_opposing_faction_score: f32,
    /// This is how many human or poorly crafted aimbot opponents you have.
    pub number_of_opponents: i32,
    /// This is how many human or poorly crafted aimbots are on your team.
    pub number_of_players: i32,
    /// This is what our non-authoritative source thought the score was.
    pub score: f32,
    /// When the activity started.
    pub start_time: Option<chrono::DateTime<chrono::Utc>>,
}

/// Some basic information about whether you can be joined, how many slots are left etc. Note that this can change quickly, so it may not actually be useful. But perhaps it will be in some use cases?
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DestinyProfileTransitoryJoinability {

    /// Reasons why a person can't join this person's fireteam.
    pub closed_reasons: crate::generated::models::destiny::DestinyJoinClosedReasons,
    /// The number of slots still available on this person's fireteam.
    pub open_slots: i32,
    /// Who the person is currently allowing invites from.
    pub privacy_setting: crate::generated::models::destiny::DestinyGamePrivacySetting,
}

/// This is some bare minimum information about a party member in a Fireteam. Unfortunately, without great computational expense on our side we can only get at the data contained here. I'd like to give you a character ID for example, but we don't have it. But we do have these three pieces of information. May they help you on your quest to show meaningful data about current Fireteams.
/// Notably, we don't and can't feasibly return info on characters. If you can, try to use just the data below for your UI and purposes. Only hit us with further queries if you absolutely must know the character ID of the currently playing character. Pretty please with sugar on top.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DestinyProfileTransitoryPartyMember {

    /// The player's last known display name.
    pub display_name: String,
    /// The identifier for the DestinyInventoryItemDefinition of the player's emblem.
    pub emblem_hash: crate::id::Id<crate::generated::models::destiny::definitions::DestinyInventoryItemDefinition>,
    /// The Membership ID that matches the party member.
    #[serde(with = "crate::unfuck_js::stringified_numbers")]
    pub membership_id: i64,
    /// A Flags Enumeration value indicating the states that the player is in relevant to being on a fireteam.
    pub status: crate::generated::models::destiny::DestinyPartyMemberStates,
}

/// This represents a single "thing" being tracked by the player.
/// This can point to many types of entities, but only a subset of them will actually have a valid hash identifier for whatever it is being pointed to.
/// It's up to you to interpret what it means when various combinations of these entries have values being tracked.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DestinyProfileTransitoryTrackingEntry {

    /// OPTIONAL - If this is tracking the status of a DestinyActivityDefinition, this is the identifier for that activity.
    pub activity_hash: Option<crate::id::Id<crate::generated::models::destiny::definitions::DestinyActivityDefinition>>,
    /// OPTIONAL - If this is tracking the status of a DestinyInventoryItemDefinition, this is the identifier for that item.
    pub item_hash: Option<crate::id::Id<crate::generated::models::destiny::definitions::DestinyInventoryItemDefinition>>,
    /// OPTIONAL - If this is tracking a DestinyLocationDefinition, this is the identifier for that location.
    pub location_hash: Option<crate::id::Id<crate::generated::models::destiny::definitions::DestinyLocationDefinition>>,
    /// OPTIONAL - If this is tracking the status of a DestinyObjectiveDefinition, this is the identifier for that objective.
    pub objective_hash: Option<crate::id::Id<crate::generated::models::destiny::definitions::DestinyObjectiveDefinition>>,
    /// OPTIONAL - If this is tracking the status of a quest, this is the identifier for the DestinyInventoryItemDefinition that containst that questline data.
    pub questline_item_hash: Option<crate::id::Id<crate::generated::models::destiny::definitions::DestinyInventoryItemDefinition>>,
    /// OPTIONAL - I've got to level with you, I don't really know what this is. Is it when you started tracking it? Is it only populated for tracked items that have time limits?
/// I don't know, but we can get at it - when I get time to actually test what it is, I'll update this. In the meantime, bask in the mysterious data.
    pub tracked_date: Option<chrono::DateTime<chrono::Utc>>,
}
