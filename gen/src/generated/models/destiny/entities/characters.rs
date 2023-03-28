use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use serde_repr::{Serialize_repr, Deserialize_repr};
use serde_with::{serde_as, DisplayFromStr};

/// This component holds activity data for a character. It will tell you about the character's current activity status, as well as activities that are available to the user.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DestinyCharacterActivitiesComponent {

    /// The list of activities that the user can play.
    pub available_activities: i32,
    /// If the user is in an activity, this will be the hash of the Activity being played. Note that you must combine this info with currentActivityModeHash to get a real picture of what the user is doing right now. For instance, PVP "Activities" are just maps: it's the ActivityMode that determines what type of PVP game they're playing.
    pub current_activity_hash: u32,
    /// If the user is in an activity, this will be the hash of the activity mode being played. Combine with currentActivityHash to give a person a full picture of what they're doing right now.
    pub current_activity_mode_hash: u32,
    /// If the user is in an activity, this will be the hashes of the DestinyActivityModeDefinition being played. Combine with currentActivityHash to give a person a full picture of what they're doing right now.
    pub current_activity_mode_hashes: i32,
    /// And the current activity's most specific mode type, if it can be found.
    pub current_activity_mode_type: Option<i32>,
    /// All Activity Modes that apply to the current activity being played, in enum form.
    pub current_activity_mode_types: i32,
    /// If the user is in a playlist, this is the hash identifier for the playlist that they chose.
    pub current_playlist_activity_hash: Option<u32>,
    /// The last date that the user started playing an activity.
    pub date_activity_started: chrono::DateTime<chrono::Utc>,
    /// This will have the activity hash of the last completed story/campaign mission, in case you care about that.
    pub last_completed_story_hash: u32,
}

/// This component contains base properties of the character. You'll probably want to always request this component, but hey you do you.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DestinyCharacterComponent {

    /// The "base" level of your character, not accounting for any light level.
    pub base_character_level: i32,
    /// The unique identifier for the character.
    #[serde(with = "crate::unfuck_js::stringified_numbers")]
    pub character_id: i64,
    /// Use this hash to look up the character's DestinyClassDefinition.
    pub class_hash: u32,
    /// Mostly for historical purposes at this point, this is an enumeration for the character's class.
/// It'll be preferable in the general case to look up the related definition: but for some people this was too convenient to remove.
    pub class_type: crate::generated::models::destiny::DestinyClass,
    /// The last date that the user played Destiny.
    pub date_last_played: chrono::DateTime<chrono::Utc>,
    /// A shortcut path to the user's currently equipped emblem background image. If you're just showing summary info for a user, this is more convenient than examining their equipped emblem and looking up the definition.
    pub emblem_background_path: String,
    /// A shortcut for getting the background color of the user's currently equipped emblem without having to do a DestinyInventoryItemDefinition lookup.
    pub emblem_color: crate::generated::models::destiny::misc::DestinyColor,
    /// The hash of the currently equipped emblem for the user. Can be used to look up the DestinyInventoryItemDefinition.
    pub emblem_hash: u32,
    /// A shortcut path to the user's currently equipped emblem image. If you're just showing summary info for a user, this is more convenient than examining their equipped emblem and looking up the definition.
    pub emblem_path: String,
    /// Use this hash to look up the character's DestinyGenderDefinition.
    pub gender_hash: u32,
    /// Mostly for historical purposes at this point, this is an enumeration for the character's Gender.
/// It'll be preferable in the general case to look up the related definition: but for some people this was too convenient to remove. And yeah, it's an enumeration and not a boolean. Fight me.
    pub gender_type: crate::generated::models::destiny::DestinyGender,
    /// The progression that indicates your character's level. Not their light level, but their character level: you know, the thing you max out a couple hours in and then ignore for the sake of light level.
    pub level_progression: crate::generated::models::destiny::DestinyProgression,
    /// The user's calculated "Light Level". Light level is an indicator of your power that mostly matters in the end game, once you've reached the maximum character level: it's a level that's dependent on the average Attack/Defense power of your items.
    pub light: i32,
    /// Every Destiny Profile has a membershipId. This is provided on the character as well for convenience.
    #[serde(with = "crate::unfuck_js::stringified_numbers")]
    pub membership_id: i64,
    /// membershipType tells you the platform on which the character plays. Examine the BungieMembershipType enumeration for possible values.
    pub membership_type: crate::generated::models::BungieMembershipType,
    /// If the user is currently playing, this is how long they've been playing.
    #[serde(with = "crate::unfuck_js::stringified_numbers")]
    pub minutes_played_this_session: i64,
    /// If this value is 525,600, then they played Destiny for a year. Or they're a very dedicated Rent fan. Note that this includes idle time, not just time spent actually in activities shooting things.
    #[serde(with = "crate::unfuck_js::stringified_numbers")]
    pub minutes_played_total: i64,
    /// A number between 0 and 100, indicating the whole and fractional % remaining to get to the next character level.
    pub percent_to_next_level: f32,
    /// Use this hash to look up the character's DestinyRaceDefinition.
    pub race_hash: u32,
    /// Mostly for historical purposes at this point, this is an enumeration for the character's race.
/// It'll be preferable in the general case to look up the related definition: but for some people this was too convenient to remove.
    pub race_type: crate::generated::models::destiny::DestinyRace,
    /// Your character's stats, such as Agility, Resilience, etc... *not* historical stats.
/// You'll have to call a different endpoint for those.
    pub stats: i32,
    /// If this Character has a title assigned to it, this is the identifier of the DestinyRecordDefinition that has that title information.
    pub title_record_hash: Option<u32>,
}

/// This component returns anything that could be considered "Progression" on a user: data where the user is gaining levels, reputation, completions, rewards, etc...
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DestinyCharacterProgressionComponent {

    /// The set of checklists that can be examined for this specific character, keyed by the hash identifier of the Checklist (DestinyChecklistDefinition)
/// For each checklist returned, its value is itself a Dictionary keyed by the checklist's hash identifier with the value being a boolean indicating if it's been discovered yet.
    pub checklists: i32,
    /// A dictionary of all known Factions, keyed by the Faction's hash. It contains data about this character's status with the faction.
    pub factions: i32,
    /// Milestones are related to the simple progressions shown in the game, but return additional and hopefully helpful information for users about the specifics of the Milestone's status.
    pub milestones: i32,
    /// A Dictionary of all known progressions for the Character, keyed by the Progression's hash.
/// Not all progressions have user-facing data, but those who do will have that data contained in the DestinyProgressionDefinition.
    pub progressions: i32,
    /// If the user has any active quests, the quests' statuses will be returned here.
///  Note that quests have been largely supplanted by Milestones, but that doesn't mean that they won't make a comeback independent of milestones at some point.
///  (Fun fact: quests came back as I feared they would, but we never looped back to populate this... I'm going to put that in the backlog.)
    pub quests: i32,
    /// Data related to your progress on the current season's artifact that can vary per character.
    pub seasonal_artifact: crate::generated::models::destiny::artifacts::DestinyArtifactCharacterScoped,
    /// Sometimes, you have items in your inventory that don't have instances, but still have Objective information. This provides you that objective information for uninstanced items. 
/// This dictionary is keyed by the item's hash: which you can use to look up the name and description for the overall task(s) implied by the objective. The value is the list of objectives for this item, and their statuses.
    pub uninstanced_item_objectives: i32,
    /// Sometimes, you have items in your inventory that don't have instances, but still have perks (for example: Trials passage cards). This gives you the perk information for uninstanced items.
/// This dictionary is keyed by item hash, which you can use to look up the corresponding item definition. The value is the list of perks states for the item.
    pub uninstanced_item_perks: i32,
}

/// Only really useful if you're attempting to render the character's current appearance in 3D, this returns a bare minimum of information, pre-aggregated, that you'll need to perform that rendering. Note that you need to combine this with other 3D assets and data from our servers.
/// Examine the Javascript returned by https://bungie.net/sharedbundle/spasm to see how we use this data, but be warned: the rabbit hole goes pretty deep.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DestinyCharacterRenderComponent {

    /// Custom dyes, calculated by iterating over the character's equipped items. Useful for pre-fetching all of the dye data needed from our server.
    pub custom_dyes: i32,
    /// This is actually something that Spasm.js *doesn't* do right now, and that we don't return assets for yet. This is the data about what character customization options you picked. You can combine this with DestinyCharacterCustomizationOptionDefinition to show some cool info, and hopefully someday to actually render a user's face in 3D. We'll see if we ever end up with time for that.
    pub customization: crate::generated::models::destiny::character::DestinyCharacterCustomization,
    /// A minimal view of:
/// - Equipped items
/// - The rendering-related custom options on those equipped items
/// Combined, that should be enough to render all of the items on the equipped character.
    pub peer_view: crate::generated::models::destiny::character::DestinyCharacterPeerView,
}
