/*
 * Bungie.Net API
 *
 * These endpoints constitute the functionality exposed by Bungie.net, both for more traditional website functionality and for connectivity to Bungie video games and their related functionality.
 *
 * The version of the OpenAPI document: 2.16.0
 * Contact: support@bungie.com
 * Generated by: https://openapi-generator.tech
 */

/// DestinyPeriodDefinitionsPeriodMilestonesPeriodDestinyMilestoneDefinition : Milestones are an in-game concept where they're attempting to tell you what you can do next in-game.  If that sounds a lot like Advisors in Destiny 1, it is! So we threw out Advisors in the Destiny 2 API and tacked all of the data we would have put on Advisors onto Milestones instead.  Each Milestone represents something going on in the game right now:  - A \"ritual activity\" you can perform, like nightfall  - A \"special event\" that may have activities related to it, like Taco Tuesday (there's no Taco Tuesday in Destiny 2)  - A checklist you can fulfill, like helping your Clan complete all of its weekly objectives  - A tutorial quest you can play through, like the introduction to the Crucible.  Most of these milestones appear in game as well. Some of them are BNet only, because we're so extra. You're welcome.  There are some important caveats to understand about how we currently render Milestones and their deficiencies. The game currently doesn't have any content that actually tells you oughtright *what* the Milestone is: that is to say, what you'll be doing. The best we get is either a description of the overall Milestone, or of the Quest that the Milestone is having you partake in: which is usually something that assumes you already know what it's talking about, like \"Complete 5 Challenges\". 5 Challenges for what? What's a challenge? These are not questions that the Milestone data will answer for you unfortunately.  This isn't great, and in the future I'd like to add some custom text to give you more contextual information to pass on to your users. But for now, you can do what we do to render what little display info we do have:  Start by looking at the currently active quest (ideally, you've fetched DestinyMilestone or DestinyPublicMilestone data from the API, so you know the currently active quest for the Milestone in question). Look up the Quests property in the Milestone Definition, and check if it has display properties. If it does, show that as the description of the Milestone. If it doesn't, fall back on the Milestone's description.  This approach will let you avoid, whenever possible, the even less useful (and sometimes nonexistant) milestone-level names and descriptions.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct DestinyPeriodDefinitionsPeriodMilestonesPeriodDestinyMilestoneDefinition {
    #[serde(rename = "displayProperties", skip_serializing_if = "Option::is_none")]
    pub display_properties: Option<Box<crate::models::DestinyPeriodDefinitionsPeriodCommonPeriodDestinyDisplayPropertiesDefinition>>,
    /// A hint to the UI to indicate what to show as the display properties for this Milestone when showing \"Live\" milestone data. Feel free to show more than this if desired: this hint is meant to simplify our own UI, but it may prove useful to you as well.
    #[serde(rename = "displayPreference", skip_serializing_if = "Option::is_none")]
    pub display_preference: Option<i32>,
    /// A custom image someone made just for the milestone. Isn't that special?
    #[serde(rename = "image", skip_serializing_if = "Option::is_none")]
    pub image: Option<String>,
    /// An enumeration listing one of the possible types of milestones. Check out the DestinyMilestoneType enum for more info!
    #[serde(rename = "milestoneType", skip_serializing_if = "Option::is_none")]
    pub milestone_type: Option<i32>,
    /// If True, then the Milestone has been integrated with BNet's recruiting feature.
    #[serde(rename = "recruitable", skip_serializing_if = "Option::is_none")]
    pub recruitable: Option<bool>,
    /// If the milestone has a friendly identifier for association with other features - such as Recruiting - that identifier can be found here. This is \"friendly\" in that it looks better in a URL than whatever the identifier for the Milestone actually is.
    #[serde(rename = "friendlyName", skip_serializing_if = "Option::is_none")]
    pub friendly_name: Option<String>,
    /// If TRUE, this entry should be returned in the list of milestones for the \"Explore Destiny\" (i.e. new BNet homepage) features of Bungie.net (as long as the underlying event is active) Note that this is a property specifically used by BNet and the companion app for the \"Live Events\" feature of the front page/welcome view: it's not a reflection of what you see in-game.
    #[serde(rename = "showInExplorer", skip_serializing_if = "Option::is_none")]
    pub show_in_explorer: Option<bool>,
    /// Determines whether we'll show this Milestone in the user's personal Milestones list.
    #[serde(rename = "showInMilestones", skip_serializing_if = "Option::is_none")]
    pub show_in_milestones: Option<bool>,
    /// If TRUE, \"Explore Destiny\" (the front page of BNet and the companion app) prioritize using the activity image over any overriding Quest or Milestone image provided. This unfortunate hack is brought to you by Trials of The Nine.
    #[serde(rename = "explorePrioritizesActivityImage", skip_serializing_if = "Option::is_none")]
    pub explore_prioritizes_activity_image: Option<bool>,
    /// A shortcut for clients - and the server - to understand whether we can predict the start and end dates for this event. In practice, there are multiple ways that an event could have predictable date ranges, but not all events will be able to be predicted via any mechanism (for instance, events that are manually triggered on and off)
    #[serde(rename = "hasPredictableDates", skip_serializing_if = "Option::is_none")]
    pub has_predictable_dates: Option<bool>,
    /// The full set of possible Quests that give the overview of the Milestone event/activity in question. Only one of these can be active at a time for a given Conceptual Milestone, but many of them may be \"available\" for the user to choose from. (for instance, with Milestones you can choose from the three available Quests, but only one can be active at a time) Keyed by the quest item.  As of Forsaken (~September 2018), Quest-style Milestones are being removed for many types of activities. There will likely be further revisions to the Milestone concept in the future.
    #[serde(rename = "quests", skip_serializing_if = "Option::is_none")]
    pub quests: Option<::std::collections::HashMap<String, crate::models::DestinyPeriodDefinitionsPeriodMilestonesPeriodDestinyMilestoneQuestDefinition>>,
    /// If this milestone can provide rewards, this will define the categories into which the individual reward entries are placed.  This is keyed by the Category's hash, which is only guaranteed to be unique within a given Milestone.
    #[serde(rename = "rewards", skip_serializing_if = "Option::is_none")]
    pub rewards: Option<::std::collections::HashMap<String, crate::models::DestinyPeriodDefinitionsPeriodMilestonesPeriodDestinyMilestoneRewardCategoryDefinition>>,
    /// If you're going to show Vendors for the Milestone, you can use this as a localized \"header\" for the section where you show that vendor data. It'll provide a more context-relevant clue about what the vendor's role is in the Milestone.
    #[serde(rename = "vendorsDisplayTitle", skip_serializing_if = "Option::is_none")]
    pub vendors_display_title: Option<String>,
    /// Sometimes, milestones will have rewards provided by Vendors. This definition gives the information needed to understand which vendors are relevant, the order in which they should be returned if order matters, and the conditions under which the Vendor is relevant to the user.
    #[serde(rename = "vendors", skip_serializing_if = "Option::is_none")]
    pub vendors: Option<Vec<crate::models::DestinyPeriodDefinitionsPeriodMilestonesPeriodDestinyMilestoneVendorDefinition>>,
    /// Sometimes, milestones will have arbitrary values associated with them that are of interest to us or to third party developers. This is the collection of those values' definitions, keyed by the identifier of the value and providing useful definition information such as localizable names and descriptions for the value.
    #[serde(rename = "values", skip_serializing_if = "Option::is_none")]
    pub values: Option<::std::collections::HashMap<String, crate::models::DestinyPeriodDefinitionsPeriodMilestonesPeriodDestinyMilestoneValueDefinition>>,
    /// Some milestones are explicit objectives that you can see and interact with in the game. Some milestones are more conceptual, built by BNet to help advise you on activities and events that happen in-game but that aren't explicitly shown in game as Milestones. If this is TRUE, you can see this as a milestone in the game. If this is FALSE, it's an event or activity you can participate in, but you won't see it as a Milestone in the game's UI.
    #[serde(rename = "isInGameMilestone", skip_serializing_if = "Option::is_none")]
    pub is_in_game_milestone: Option<bool>,
    /// A Milestone can now be represented by one or more activities directly (without a backing Quest), and that activity can have many challenges, modifiers, and related to it.
    #[serde(rename = "activities", skip_serializing_if = "Option::is_none")]
    pub activities: Option<Vec<crate::models::DestinyPeriodDefinitionsPeriodMilestonesPeriodDestinyMilestoneChallengeActivityDefinition>>,
    #[serde(rename = "defaultOrder", skip_serializing_if = "Option::is_none")]
    pub default_order: Option<i32>,
    /// The unique identifier for this entity. Guaranteed to be unique for the type of entity, but not globally.  When entities refer to each other in Destiny content, it is this hash that they are referring to.
    #[serde(rename = "hash", skip_serializing_if = "Option::is_none")]
    pub hash: Option<i32>,
    /// The index of the entity as it was found in the investment tables.
    #[serde(rename = "index", skip_serializing_if = "Option::is_none")]
    pub index: Option<i32>,
    /// If this is true, then there is an entity with this identifier/type combination, but BNet is not yet allowed to show it. Sorry!
    #[serde(rename = "redacted", skip_serializing_if = "Option::is_none")]
    pub redacted: Option<bool>,
}

impl DestinyPeriodDefinitionsPeriodMilestonesPeriodDestinyMilestoneDefinition {
    /// Milestones are an in-game concept where they're attempting to tell you what you can do next in-game.  If that sounds a lot like Advisors in Destiny 1, it is! So we threw out Advisors in the Destiny 2 API and tacked all of the data we would have put on Advisors onto Milestones instead.  Each Milestone represents something going on in the game right now:  - A \"ritual activity\" you can perform, like nightfall  - A \"special event\" that may have activities related to it, like Taco Tuesday (there's no Taco Tuesday in Destiny 2)  - A checklist you can fulfill, like helping your Clan complete all of its weekly objectives  - A tutorial quest you can play through, like the introduction to the Crucible.  Most of these milestones appear in game as well. Some of them are BNet only, because we're so extra. You're welcome.  There are some important caveats to understand about how we currently render Milestones and their deficiencies. The game currently doesn't have any content that actually tells you oughtright *what* the Milestone is: that is to say, what you'll be doing. The best we get is either a description of the overall Milestone, or of the Quest that the Milestone is having you partake in: which is usually something that assumes you already know what it's talking about, like \"Complete 5 Challenges\". 5 Challenges for what? What's a challenge? These are not questions that the Milestone data will answer for you unfortunately.  This isn't great, and in the future I'd like to add some custom text to give you more contextual information to pass on to your users. But for now, you can do what we do to render what little display info we do have:  Start by looking at the currently active quest (ideally, you've fetched DestinyMilestone or DestinyPublicMilestone data from the API, so you know the currently active quest for the Milestone in question). Look up the Quests property in the Milestone Definition, and check if it has display properties. If it does, show that as the description of the Milestone. If it doesn't, fall back on the Milestone's description.  This approach will let you avoid, whenever possible, the even less useful (and sometimes nonexistant) milestone-level names and descriptions.
    pub fn new() -> DestinyPeriodDefinitionsPeriodMilestonesPeriodDestinyMilestoneDefinition {
        DestinyPeriodDefinitionsPeriodMilestonesPeriodDestinyMilestoneDefinition {
            display_properties: None,
            display_preference: None,
            image: None,
            milestone_type: None,
            recruitable: None,
            friendly_name: None,
            show_in_explorer: None,
            show_in_milestones: None,
            explore_prioritizes_activity_image: None,
            has_predictable_dates: None,
            quests: None,
            rewards: None,
            vendors_display_title: None,
            vendors: None,
            values: None,
            is_in_game_milestone: None,
            activities: None,
            default_order: None,
            hash: None,
            index: None,
            redacted: None,
        }
    }
}


