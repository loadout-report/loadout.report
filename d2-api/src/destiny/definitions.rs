pub mod common;

use crate::destiny::definitions::common::DisplayPropertiesDefinition;
use crate::destiny::Hash;
use serde::{Deserialize, Serialize};

/// Provides common properties for destiny definitions.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct Definition {
    /// The unique identifier for this entity. Guaranteed to be unique for the type of entity,
    /// but not globally.
    ///
    /// When entities refer to each other in Destiny content,
    /// it is this hash that they are referring to.
    pub hash: Hash,
    /// The index of the entity as it was found in the investment tables.
    pub index: i32,
    /// If this is true, then there is an entity with this identifier/type combination,
    /// but BNet is not yet allowed to show it. Sorry!
    pub redacted: bool,
}

/// A "Progression" in Destiny is best explained by an example.
///
/// A Character's "Level" is a progression: it has Experience that can be earned,
/// levels that can be gained, and is evaluated and displayed at various points in the game.
/// A Character's "Faction Reputation" is also a progression for much the same reason.
///
/// Progression is used by a variety of systems,
/// and the definition of a Progression will generally only be useful if combining with live data
/// (such as a character's DestinyCharacterProgressionComponent.progressions property,
/// which holds that character's live Progression states).
///
/// Fundamentally, a Progression measures your "Level" by evaluating the thresholds in its Steps
/// (one step per level, except for the last step which can be repeated indefinitely for "Levels"
/// that have no ceiling) against the total earned "progression points"/experience.
/// (for simplicity purposes, we will henceforth refer to earned progression points as experience,
/// though it need not be a mechanic that in any way resembles Experience in a traditional sense).
///
/// Earned experience is calculated in a variety of ways, determined by the Progression's scope.
/// These go from looking up a stored value to performing exceedingly obtuse calculations.
/// This is why we provide live data in DestinyCharacterProgressionComponent.progressions,
/// so you don't have to worry about those.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct ProgressionDefinition {
    #[serde(flatten)]
    pub definition: Definition,
    pub display_properties: ProgressionDisplayPropertiesDefinition,
    /// The "Scope" of the progression indicates the source of the progression's live data.
    ///
    /// See the DestinyProgressionScope enum for more info:
    /// but essentially, a Progression can either be backed by a stored value,
    /// or it can be a calculated derivative of other values.
    pub scope: super::ProgressionScope,
    /// If this is True, then the progression doesn't have a maximum level.
    pub repeat_last_step: bool,
    /// If there's a description of how to earn this progression in the local config,
    /// this will be that localized description.
    pub source: String,
    /// Progressions are divided into Steps, which roughly equate to "Levels" in the traditional
    /// sense of a Progression. Notably, the last step can be repeated indefinitely if
    /// repeatLastStep is true, meaning that the calculation for your level is not as simple as
    /// comparing your current progress to the max progress of the steps.
    ///
    /// These and more calculations are done for you if you grab live character progression data,
    /// such as in the DestinyCharacterProgressionComponent.
    pub steps: Vec<ProgressionStepDefinition>,
    /// If true, the Progression is something worth showing to users.
    ///
    /// If false, BNet isn't going to show it. But that doesn't mean you can't.
    /// We're all friends here.
    pub visible: bool,
    /// If the value exists, this is the hash identifier for the Faction that owns this Progression.
    ///
    /// This is purely for convenience, if you're looking at a progression and want to know if
    /// and who it's related to in terms of Faction Reputation.
    pub faction_hash: Option<DefinitionHash>,
    /// The #RGB string value for the color related to this progression, if there is one.
    pub color: Option<super::misc::Color>,
    /// For progressions that have it, this is the rank icon we use in the Companion,
    /// displayed above the progressions' rank value.
    #[serde(deserialize_with = "crate::util::serde::empty_string_as_none")]
    pub rank_icon: Option<String>,
    pub reward_items: Vec<ProgressionRewardItemQuantity>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct ProgressionDisplayPropertiesDefinition {
    /// When progressions show your "experience" gained, that bar has units
    /// (i.e. "Experience", "Bad Dudes Snuffed Out", whatever).
    /// This is the localized string for that unit of measurement.
    pub display_units_name: String,

    #[serde(flatten)]
    pub properties: DisplayPropertiesDefinition,
}

/// This defines a single Step in a progression (which roughly equates to a level.
/// See DestinyProgressionDefinition for caveats).
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct ProgressionStepDefinition {
    /// Very rarely, Progressions will have localized text describing the Level of the progression.
    /// This will be that localized text, if it exists.
    /// Otherwise, the standard appears to be to simply show the level numerically.
    pub step_name: String,
    /// This appears to be, when you "level up",
    /// whether a visual effect will display and on what entity.
    /// See DestinyProgressionStepDisplayEffect for slightly more info.
    pub display_effect_type: super::ProgressionStepDisplayEffect,
    /// The total amount of progression points/"experience"
    /// you will need to initially reach this step.
    /// If this is the last step and the progression is repeating indefinitely
    /// (DestinyProgressionDefinition.repeatLastStep),
    /// this will also be the progress needed to level it up further by repeating this step again.
    pub progress_total: i32,
    /// A listing of items rewarded as a result of reaching this level.
    pub reward_items: Vec<super::ItemQuantity>,
    /// If this progression step has a specific icon related to it, this is the icon to show.
    pub icon: String,
}

/// So much of what you see in Destiny is actually an Item used in a new and creative way.
/// This is the definition for Items in Destiny, which started off as just entities
/// that could exist in your Inventory but ended up being the backing data for so much more:
/// quests, reward previews, slots, and subclasses.
///
/// In practice, you will want to associate this data with "live" item data
/// from a Bungie.Net Platform call:
/// these definitions describe the item in generic, non-instanced terms:
/// but an actual instance of an item can vary widely from these generic definitions.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct InventoryItemDefinition {
    display_properties: DisplayPropertiesDefinition,
    /// Tooltips that only come up conditionally for the item.
    /// Check the live data DestinyItemComponent.tooltipNotificationIndexes
    /// property for which of these should be shown at runtime.
    tooltip_notifications: Vec<ItemTooltipNotification>,
    /// If this item has a collectible related to it,
    /// this is the hash identifier of that collectible entry.
    collectible_hash: Option<super::CollectibleHash>,
    /// If available, this is the original 'active' release watermark overlay for the icon.
    /// If the item has different versions, this can be overridden by the
    /// 'display version watermark icon' from the 'quality' block.
    /// Alternatively, if there is no watermark for the version,
    /// and the item version has a power cap below the current season power cap,
    /// this can be overridden by the iconWatermarkShelved property.
    icon_watermark: String,
    /// If available, this is the 'shelved' release watermark overlay for the icon.
    /// If the item version has a power cap below the current season power cap,
    /// it can be treated as 'shelved', and should be shown with this 'shelved' watermark overlay.
    icon_watermark_shelved: String,
    /// A secondary icon associated with the item.
    /// Currently this is used in very context specific applications, such as Emblem Nameplates.
    secondary_icon: String,
    /// Pulled from the secondary icon, this is the "secondary background" of the secondary icon.
    /// Confusing? Sure, that's why I call it "overlay" here:
    /// because as far as it's been used thus far, it has been for an optional overlay image.
    /// We'll see if that holds up, but at least for now it explains what this image is a bit better.
    secondary_overlay: String,
    /// Pulled from the Secondary Icon, this is the "special" background for the item.
    /// For Emblems, this is the background image used on the Details view:
    /// but it need not be limited to that for other types of items.
    secondary_special: String,
    /// Sometimes, an item will have a background color.
    /// Most notably this occurs with Emblems, who use the Background Color for small character
    /// nameplates such as the "friends" view you see in-game.
    /// There are almost certainly other items that have background color as well,
    /// though I have not bothered to investigate what items have it nor what purposes they serve:
    /// use it as you will.
    background_color: super::misc::Color,
    /// If we were able to acquire an in-game screenshot for the item, the path to that screenshot will be returned here. Note that not all items have screenshots: particularly not any non-equippable items.
    screenshot: String,
    /// The localized title/name of the item's type. This can be whatever the designers want, and has no guarantee of consistency between items.
    item_type_display_name: String,
    flavor_text: String,
    /// A string identifier that the game's UI uses to determine how the item should be rendered in inventory screens and the like. This could really be anything - at the moment, we don't have the time to really breakdown and maintain all the possible strings this could be, partly because new ones could be added ad hoc. But if you want to use it to dictate your own UI, or look for items with a certain display style, go for it!
    ui_item_display_style: String,
    /// It became a common enough pattern in our UI to show Item Type and Tier combined into a single localized string that I'm just going to go ahead and start pre-creating these for items.
    item_type_and_tier_display_name: String,
    /// In theory, it is a localized string telling you about how you can find the item. I really wish this was more consistent. Many times, it has nothing. Sometimes, it's instead a more narrative-forward description of the item. Which is cool, and I wish all properties had that data, but it should really be its own property.
    display_source: String,
    /// An identifier that the game UI uses to determine what type of tooltip to show for the item. These have no corresponding definitions that BNet can link to: so it'll be up to you to interpret and display your UI differently according to these styles (or ignore it).
    tooltip_style: String,
    /// If the item can be "used", this block will be non-null, and will have data related to the action performed when using the item. (Guess what? 99% of the time, this action is "dismantle". Shocker)
    action: ItemActionBlockDefinition,
    /// Recipe items will have relevant crafting information available here.
    crafting: ItemCraftingBlockDefinition,
    /// If this item can exist in an inventory, this block will be non-null. In practice, every item that currently exists has one of these blocks. But note that it is not necessarily guaranteed.
    inventory: Option<ItemInventoryBlockDefinition>,
    /// If this item is a quest, this block will be non-null. In practice, I wish I had called this the Quest block, but at the time it wasn't clear to me whether it would end up being used for purposes other than quests. It will contain data about the steps in the quest, and mechanics we can use for displaying and tracking the quest.
    set_data: Option<ItemSetBlockDefinition>,
    /// If this item can have stats (such as a weapon, armor, or vehicle), this block will be non-null and populated with the stats found on the item.
    stats: ItemStatBlockDefinition,
    /// If the item is an emblem that has a special Objective attached to it - for instance, if the emblem tracks PVP Kills, or what-have-you. This is a bit different from, for example, the Vanguard Kill Tracker mod, which pipes data into the "art channel". When I get some time, I would like to standardize these so you can get at the values they expose without having to care about what they're being used for and how they are wired up, but for now here's the raw data.
    emblem_objective_hash: Option<super::ObjectiveHash>,
    /// If this item can be equipped, this block will be non-null and will be populated with the conditions under which it can be equipped.
    equipping_block: EquippingBlockDefinition,
    /// If this item can be rendered, this block will be non-null and will be populated with rendering information.
    translation_block: ItemTranslationBlockDefinition,
    /// If this item can be Used or Acquired to gain other items (for instance, how Eververse Boxes can be consumed to get items from the box), this block will be non-null and will give summary information for the items that can be acquired.
    preview: ItemPreviewBlockDefinition,
    /// If this item can have a level or stats, this block will be non-null and will be populated with default quality (item level, "quality", and infusion) data. See the block for more details, there's often less upfront information in D2 so you'll want to be aware of how you use quality and item level on the definition level now.
    quality: ItemQualityBlockDefinition,
    /// The conceptual "Value" of an item, if any was defined. See the DestinyItemValueBlockDefinition for more details.
    value: ItemValueBlockDefinition,
    /// If this item has a known source, this block will be non-null and populated with source information. Unfortunately, at this time we are not generating sources: that is some aggressively manual work which we didn't have time for, and I'm hoping to get back to at some point in the future.
    source_data: ItemSourceBlockDefinition,
    /// If this item has Objectives (extra tasks that can be accomplished related to the item... most frequently when the item is a Quest Step and the Objectives need to be completed to move on to the next Quest Step), this block will be non-null and the objectives defined herein.
    objectives: ItemObjectiveBlockDefinition,
    /// If this item has available metrics to be shown, this block will be non-null have the appropriate hashes defined.
    metrics: ItemMetricBlockDefinition,
    /// If this item *is* a Plug, this will be non-null and the info defined herein. See DestinyItemPlugDefinition for more information.
    plug: ItemPlugDefinition,
    /// If this item has related items in a "Gear Set", this will be non-null and the relationships defined herein.
    gearset: ItemGearsetBlockDefinition,
    /// If this item is a "reward sack" that can be opened to provide other items, this will be non-null and the properties of the sack contained herein.
    sack: ItemSackBlockDefinition,
    /// If this item has any Sockets, this will be non-null and the individual sockets on the item will be defined herein.
    sockets: ItemSocketBlockDefinition,
    /// Summary data about the item.
    summary: ItemSummaryBlockDefinition,
    /// If the item has a Talent Grid, this will be non-null and the properties of the grid defined herein. Note that, while many items still have talent grids, the only ones with meaningful Nodes still on them will be Subclass/"Build" items.
    talent_grid: ItemTalentGridBlockDefinition,
    /// If the item has stats, this block will be defined. It has the "raw" investment stats for the item. These investment stats don't take into account the ways that the items can spawn, nor do they take into account any Stat Group transformations. I have retained them for debugging purposes, but I do not know how useful people will find them.
    investment_stats: Vec<ItemInvestmentStatDefinition>,
    /// If the item has any *intrinsic* Perks (Perks that it will provide regardless of Sockets, Talent Grid, and other transitory state), they will be defined here.
    perks: Vec<ItemPerkEntryDefinition>,
    /// If the item has any related Lore (DestinyLoreDefinition), this will be the hash identifier you can use to look up the lore definition.
    lore_hash: Option<super::LoreHash>,
    /// There are times when the game will show you a "summary/vague" version of an item - such as a description of its type represented as a DestinyInventoryItemDefinition - rather than display the item itself.
    ///
    /// This happens sometimes when summarizing possible rewards in a tooltip. This is the item displayed instead, if it exists.
    summary_item_hash: Option<super::InventoryItemHash>,
    /// If any animations were extracted from game content for this item, these will be the definitions of those animations.
    animations: Vec<animations::AnimationReference>,
    /// BNet may forbid the execution of actions on this item via the API. If that is occurring, allowActions will be set to false.
    allow_actions: bool,
    /// If we added any help or informational URLs about this item, these will be those links.
    links: Vec<crate::links::HyperlinkReference>,
    /// The boolean will indicate to us (and you!) whether something *could* happen when you transfer this item from the Postmaster that might be considered a "destructive" action.
    ///
    /// It is not feasible currently to tell you (or ourelves!) in a consistent way whether this *will* actually cause a destructive action, so we are playing it safe: if it has the potential to do so, we will not allow it to be transferred from the Postmaster by default. You will need to check for this flag before transferring an item from the Postmaster, or else you'll end up receiving an error.
    does_postmaster_pull_have_side_effects: bool,
    /// The intrinsic transferability of an item.
    ///
    /// I hate that this boolean is negative - but there's a reason.
    ///
    /// Just because an item is intrinsically transferrable doesn't mean that it can be transferred, and we don't want to imply that this is the only source of that transferability.
    non_transferrable: bool,
    /// BNet attempts to make a more formal definition of item "Categories", as defined by DestinyItemCategoryDefinition. This is a list of all Categories that we were able to algorithmically determine that this item is a member of. (for instance, that it's a "Weapon", that it's an "Auto Rifle", etc...)
    ///
    /// The algorithm for these is, unfortunately, volatile. If you believe you see a miscategorized item, please let us know on the Bungie API forums.
    item_category_hashes: Vec<super::ItemCategoryHash>,
    /// In Destiny 1, we identified some items as having particular categories that we'd like to know about for various internal logic purposes. These are defined in SpecialItemType, and while these days the itemCategoryHashes are the preferred way of identifying types, we have retained this enum for its convenience.
    special_item_type: super::SpecialItemType,
    /// A value indicating the "base" the of the item. This enum is a useful but dramatic oversimplification of what it means for an item to have a "Type". Still, it's handy in many situations.
    ///
    /// itemCategoryHashes are the preferred way of identifying types, we have retained this enum for its convenience.
    item_type: super::ItemType,
    /// A value indicating the "sub-type" of the item. For instance, where an item might have an itemType value "Weapon", this will be something more specific like "Auto Rifle".
    ///
    /// itemCategoryHashes are the preferred way of identifying types, we have retained this enum for its convenience.
    item_sub_type: super::ItemSubType,
    /// We run a similarly weak-sauce algorithm to try and determine whether an item is restricted to a specific class. If we find it to be restricted in such a way, we set this classType property to match the class' enumeration value so that users can easily identify class restricted items.
    ///
    /// If you see a mis-classed item, please inform the developers in the Bungie API forum.
    class_type: super::Class,
    /// Some weapons and plugs can have a "Breaker Type": a special ability that works sort of like damage type vulnerabilities. This is (almost?) always set on items by plugs.
    breaker_type: super::BreakerType,
    /// Since we also have a breaker type definition, this is the hash for that breaker type for your convenience. Whether you use the enum or hash and look up the definition depends on what's cleanest for your code.
    breaker_type_hash: Option<super::BreakerTypeHash>,
    /// If true, then you will be allowed to equip the item if you pass its other requirements.
    ///  
    /// This being false means that you cannot equip the item under any circumstances.
    equippable: bool,
    /// Theoretically, an item can have many possible damage types. In *practice*, this is not true, but just in case weapons start being made that have multiple (for instance, an item where a socket has reusable plugs for every possible damage type that you can choose from freely), this field will return all of the possible damage types that are available to the weapon by default.
    damage_type_hashes: Vec<super::DamageTypeHash>,
    /// This is the list of all damage types that we know ahead of time the item can take on. Unfortunately, this does not preclude the possibility of something funky happening to give the item a damage type that cannot be predicted beforehand: for example, if some designer decides to create arbitrary non-reusable plugs that cause damage type to change.
    ///
    /// This damage type prediction will only use the following to determine potential damage types:
    /// - Intrinsic perks
    /// - Talent Node perks
    /// - Known, reusable plugs for sockets
    damage_types: Vec<super::DamageType>,
    /// If the item has a damage type that could be considered to be default, it will be populated here.
    ///
    /// For various upsetting reasons, it's surprisingly cumbersome to figure this out. I hope you're happy.
    default_damage_type: super::DamageType,
    /// Similar to defaultDamageType, but represented as the hash identifier for a DestinyDamageTypeDefinition.
    ///
    /// I will likely regret leaving in the enumeration versions of these properties, but for now they're very convenient.
    default_damage_type_hash: Option<super::DamageTypeHash>,
    /// If this item is related directly to a Season of Destiny, this is the hash identifier for that season.
    season_hash: Option<super::SeasonHash>,
    /// If true, this is a dummy vendor-wrapped item template. Items purchased from Eververse will be "wrapped" by one of these items so that we can safely provide refund capabilities before the item is "unwrapped".
    is_wrapper: bool,
    /// Traits are metadata tags applied to this item. For example: armor slot, weapon type, foundry, faction, etc. These IDs come from the game and don't map to any content, but should still be useful.
    trait_ids: Vec<String>,
    /// These are the corresponding trait definition hashes for the entries in traitIds.
    trait_hashes: Vec<super::TraitHash>,
    #[serde(flatten)]
    definition: Definition,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct ItemTooltipNotification {
    display_string: String,
    display_style: String,
}

/// If an item can have an action performed on it (like "Dismantle"), it will be defined here if you care.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct ItemActionBlockDefinition {
    /// Localized text for the verb of the action being performed.
    verb_name: String,
    /// Localized text describing the action being performed.
    verb_description: String,
    /// The content has this property, however it's not entirely clear how it is used.
    is_positive: bool,
    /// If the action has an overlay screen associated with it, this is the name of that screen. Unfortunately, we cannot return the screen's data itself.
    overlay_screen_name: String,
    /// The icon associated with the overlay screen for the action, if any.
    overlay_icon: String,
    /// The number of seconds to delay before allowing this action to be performed again.
    required_cooldown_seconds: i32,
    /// If the action requires other items to exist or be destroyed, this is the list of those items and requirements.
    required_items: Vec<ItemActionRequiredItemDefinition>,
    /// If performing this action earns you Progression, this is the list of progressions and values granted for those progressions by performing this action.
    progression_rewards: Vec<ProgressionRewardDefinition>,
    /// The internal identifier for the action.
    action_type_label: String,
    /// Theoretically, an item could have a localized string for a hint about the location in which the action should be performed. In practice, no items yet have this property.
    required_location: String,
    /// The identifier hash for the Cooldown associated with this action. We have not pulled this data yet for you to have more data to use for cooldowns.
    required_cooldown_hash: Option<CooldowHash>,
    /// If true, the item is deleted when the action completes.
    delete_on_action: bool,
    /// If true, the entire stack is deleted when the action completes.
    consume_entire_stack: bool,
    /// If true, this action will be performed as soon as you earn this item. Some rewards work this way, providing you a single item to pick up from a reward-granting vendor in-game and then immediately consuming itself to provide you multiple items.
    use_on_acquire: bool,
}
