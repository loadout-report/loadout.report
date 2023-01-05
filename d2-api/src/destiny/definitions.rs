pub mod common;
pub mod items;
pub mod sockets;

use crate::destiny::definitions::common::DisplayPropertiesDefinition;
use crate::destiny::{Hash, ProgressionHash};
use enumflags2::BitFlags;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt::Debug;

pub trait Hashable: Copy + Clone + Serialize + Debug {}

/// Provides common properties for destiny definitions.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct Definition<T: Hashable> {
    /// The unique identifier for this entity. Guaranteed to be unique for the type of entity,
    /// but not globally.
    ///
    /// When entities refer to each other in Destiny content,
    /// it is this hash that they are referring to.
    pub hash: T,
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
    pub definition: Definition<ProgressionHash>,
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
    pub faction_hash: Option<super::FactionHash>,
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
    pub display_properties: DisplayPropertiesDefinition,
    /// Tooltips that only come up conditionally for the item.
    /// Check the live data DestinyItemComponent.tooltipNotificationIndexes
    /// property for which of these should be shown at runtime.
    pub tooltip_notifications: Vec<ItemTooltipNotification>,
    /// If this item has a collectible related to it,
    /// this is the hash identifier of that collectible entry.
    pub collectible_hash: Option<super::CollectibleHash>,
    /// If available, this is the original 'active' release watermark overlay for the icon.
    /// If the item has different versions, this can be overridden by the
    /// 'display version watermark icon' from the 'quality' block.
    /// Alternatively, if there is no watermark for the version,
    /// and the item version has a power cap below the current season power cap,
    /// this can be overridden by the iconWatermarkShelved property.
    pub icon_watermark: String,
    /// If available, this is the 'shelved' release watermark overlay for the icon.
    /// If the item version has a power cap below the current season power cap,
    /// it can be treated as 'shelved', and should be shown with this 'shelved' watermark overlay.
    pub icon_watermark_shelved: String,
    /// A secondary icon associated with the item.
    /// Currently this is used in very context specific applications, such as Emblem Nameplates.
    pub secondary_icon: String,
    /// Pulled from the secondary icon, this is the "secondary background" of the secondary icon.
    /// Confusing? Sure, that's why I call it "overlay" here:
    /// because as far as it's been used thus far, it has been for an optional overlay image.
    /// We'll see if that holds up, but at least for now it explains what this image is a bit better.
    pub secondary_overlay: String,
    /// Pulled from the Secondary Icon, this is the "special" background for the item.
    /// For Emblems, this is the background image used on the Details view:
    /// but it need not be limited to that for other types of items.
    pub secondary_special: String,
    /// Sometimes, an item will have a background color.
    /// Most notably this occurs with Emblems, who use the Background Color for small character
    /// nameplates such as the "friends" view you see in-game.
    /// There are almost certainly other items that have background color as well,
    /// though I have not bothered to investigate what items have it nor what purposes they serve:
    /// use it as you will.
    pub background_color: super::misc::Color,
    /// If we were able to acquire an in-game screenshot for the item, the path to that screenshot will be returned here. Note that not all items have screenshots: particularly not any non-equippable items.
    pub screenshot: String,
    /// The localized title/name of the item's type. This can be whatever the designers want, and has no guarantee of consistency between items.
    pub item_type_display_name: String,
    pub flavor_text: String,
    /// A string identifier that the game's UI uses to determine how the item should be rendered in inventory screens and the like. This could really be anything - at the moment, we don't have the time to really breakdown and maintain all the possible strings this could be, partly because new ones could be added ad hoc. But if you want to use it to dictate your own UI, or look for items with a certain display style, go for it!
    pub ui_item_display_style: String,
    /// It became a common enough pattern in our UI to show Item Type and Tier combined into a single localized string that I'm just going to go ahead and start pre-creating these for items.
    pub item_type_and_tier_display_name: String,
    /// In theory, it is a localized string telling you about how you can find the item. I really wish this was more consistent. Many times, it has nothing. Sometimes, it's instead a more narrative-forward description of the item. Which is cool, and I wish all properties had that data, but it should really be its own property.
    pub display_source: String,
    /// An identifier that the game UI uses to determine what type of tooltip to show for the item. These have no corresponding definitions that BNet can link to: so it'll be up to you to interpret and display your UI differently according to these styles (or ignore it).
    pub tooltip_style: String,
    /// If the item can be "used", this block will be non-null, and will have data related to the action performed when using the item. (Guess what? 99% of the time, this action is "dismantle". Shocker)
    pub action: ItemActionBlockDefinition,
    /// Recipe items will have relevant crafting information available here.
    pub crafting: ItemCraftingBlockDefinition,
    /// If this item can exist in an inventory, this block will be non-null. In practice, every item that currently exists has one of these blocks. But note that it is not necessarily guaranteed.
    pub inventory: Option<ItemInventoryBlockDefinition>,
    /// If this item is a quest, this block will be non-null. In practice, I wish I had called this the Quest block, but at the time it wasn't clear to me whether it would end up being used for purposes other than quests. It will contain data about the steps in the quest, and mechanics we can use for displaying and tracking the quest.
    pub set_data: Option<ItemSetBlockDefinition>,
    /// If this item can have stats (such as a weapon, armor, or vehicle), this block will be non-null and populated with the stats found on the item.
    pub stats: ItemStatBlockDefinition,
    /// If the item is an emblem that has a special Objective attached to it - for instance, if the emblem tracks PVP Kills, or what-have-you. This is a bit different from, for example, the Vanguard Kill Tracker mod, which pipes data into the "art channel". When I get some time, I would like to standardize these so you can get at the values they expose without having to care about what they're being used for and how they are wired up, but for now here's the raw data.
    pub emblem_objective_hash: Option<super::ObjectiveHash>,
    /// If this item can be equipped, this block will be non-null and will be populated with the conditions under which it can be equipped.
    pub equipping_block: EquippingBlockDefinition,
    /// If this item can be rendered, this block will be non-null and will be populated with rendering information.
    pub translation_block: ItemTranslationBlockDefinition,
    /// If this item can be Used or Acquired to gain other items (for instance, how Eververse Boxes can be consumed to get items from the box), this block will be non-null and will give summary information for the items that can be acquired.
    pub preview: ItemPreviewBlockDefinition,
    /// If this item can have a level or stats, this block will be non-null and will be populated with default quality (item level, "quality", and infusion) data. See the block for more details, there's often less upfront information in D2 so you'll want to be aware of how you use quality and item level on the definition level now.
    pub quality: ItemQualityBlockDefinition,
    /// The conceptual "Value" of an item, if any was defined. See the DestinyItemValueBlockDefinition for more details.
    pub value: ItemValueBlockDefinition,
    /// If this item has a known source, this block will be non-null and populated with source information. Unfortunately, at this time we are not generating sources: that is some aggressively manual work which we didn't have time for, and I'm hoping to get back to at some point in the future.
    pub source_data: ItemSourceBlockDefinition,
    /// If this item has Objectives (extra tasks that can be accomplished related to the item... most frequently when the item is a Quest Step and the Objectives need to be completed to move on to the next Quest Step), this block will be non-null and the objectives defined herein.
    pub objectives: ItemObjectiveBlockDefinition,
    /// If this item has available metrics to be shown, this block will be non-null have the appropriate hashes defined.
    pub metrics: ItemMetricBlockDefinition,
    /// If this item *is* a Plug, this will be non-null and the info defined herein. See DestinyItemPlugDefinition for more information.
    pub plug: ItemPlugDefinition,
    /// If this item has related items in a "Gear Set", this will be non-null and the relationships defined herein.
    pub gearset: ItemGearsetBlockDefinition,
    /// If this item is a "reward sack" that can be opened to provide other items, this will be non-null and the properties of the sack contained herein.
    pub sack: ItemSackBlockDefinition,
    /// If this item has any Sockets, this will be non-null and the individual sockets on the item will be defined herein.
    pub sockets: ItemSocketBlockDefinition,
    /// Summary data about the item.
    pub summary: ItemSummaryBlockDefinition,
    /// If the item has a Talent Grid, this will be non-null and the properties of the grid defined herein. Note that, while many items still have talent grids, the only ones with meaningful Nodes still on them will be Subclass/"Build" items.
    pub talent_grid: ItemTalentGridBlockDefinition,
    /// If the item has stats, this block will be defined. It has the "raw" investment stats for the item. These investment stats don't take into account the ways that the items can spawn, nor do they take into account any Stat Group transformations. I have retained them for debugging purposes, but I do not know how useful people will find them.
    pub investment_stats: Vec<ItemInvestmentStatDefinition>,
    /// If the item has any *intrinsic* Perks (Perks that it will provide regardless of Sockets, Talent Grid, and other transitory state), they will be defined here.
    pub perks: Vec<ItemPerkEntryDefinition>,
    /// If the item has any related Lore (DestinyLoreDefinition), this will be the hash identifier you can use to look up the lore definition.
    pub lore_hash: Option<super::LoreHash>,
    /// There are times when the game will show you a "summary/vague" version of an item - such as a description of its type represented as a DestinyInventoryItemDefinition - rather than display the item itself.
    ///
    /// This happens sometimes when summarizing possible rewards in a tooltip. This is the item displayed instead, if it exists.
    pub summary_item_hash: Option<super::ItemHash>,
    /// If any animations were extracted from game content for this item, these will be the definitions of those animations.
    pub animations: Vec<animations::AnimationReference>,
    /// BNet may forbid the execution of actions on this item via the API. If that is occurring, allowActions will be set to false.
    pub allow_actions: bool,
    /// If we added any help or informational URLs about this item, these will be those links.
    pub links: Vec<crate::links::HyperlinkReference>,
    /// The boolean will indicate to us (and you!) whether something *could* happen when you transfer this item from the Postmaster that might be considered a "destructive" action.
    ///
    /// It is not feasible currently to tell you (or ourelves!) in a consistent way whether this *will* actually cause a destructive action, so we are playing it safe: if it has the potential to do so, we will not allow it to be transferred from the Postmaster by default. You will need to check for this flag before transferring an item from the Postmaster, or else you'll end up receiving an error.
    pub does_postmaster_pull_have_side_effects: bool,
    /// The intrinsic transferability of an item.
    ///
    /// I hate that this boolean is negative - but there's a reason.
    ///
    /// Just because an item is intrinsically transferrable doesn't mean that it can be transferred, and we don't want to imply that this is the only source of that transferability.
    pub non_transferrable: bool,
    /// BNet attempts to make a more formal definition of item "Categories", as defined by DestinyItemCategoryDefinition. This is a list of all Categories that we were able to algorithmically determine that this item is a member of. (for instance, that it's a "Weapon", that it's an "Auto Rifle", etc...)
    ///
    /// The algorithm for these is, unfortunately, volatile. If you believe you see a miscategorized item, please let us know on the Bungie API forums.
    pub item_category_hashes: Vec<super::ItemCategoryHash>,
    /// In Destiny 1, we identified some items as having particular categories that we'd like to know about for various internal logic purposes. These are defined in SpecialItemType, and while these days the itemCategoryHashes are the preferred way of identifying types, we have retained this enum for its convenience.
    pub special_item_type: super::SpecialItemType,
    /// A value indicating the "base" the of the item. This enum is a useful but dramatic oversimplification of what it means for an item to have a "Type". Still, it's handy in many situations.
    ///
    /// itemCategoryHashes are the preferred way of identifying types, we have retained this enum for its convenience.
    pub item_type: super::ItemType,
    /// A value indicating the "sub-type" of the item. For instance, where an item might have an itemType value "Weapon", this will be something more specific like "Auto Rifle".
    ///
    /// itemCategoryHashes are the preferred way of identifying types, we have retained this enum for its convenience.
    pub item_sub_type: super::ItemSubType,
    /// We run a similarly weak-sauce algorithm to try and determine whether an item is restricted to a specific class. If we find it to be restricted in such a way, we set this classType property to match the class' enumeration value so that users can easily identify class restricted items.
    ///
    /// If you see a mis-classed item, please inform the developers in the Bungie API forum.
    pub class_type: super::Class,
    /// Some weapons and plugs can have a "Breaker Type": a special ability that works sort of like damage type vulnerabilities. This is (almost?) always set on items by plugs.
    pub breaker_type: super::BreakerType,
    /// Since we also have a breaker type definition, this is the hash for that breaker type for your convenience. Whether you use the enum or hash and look up the definition depends on what's cleanest for your code.
    pub breaker_type_hash: Option<super::BreakerTypeHash>,
    /// If true, then you will be allowed to equip the item if you pass its other requirements.
    ///  
    /// This being false means that you cannot equip the item under any circumstances.
    pub equippable: bool,
    /// Theoretically, an item can have many possible damage types. In *practice*, this is not true, but just in case weapons start being made that have multiple (for instance, an item where a socket has reusable plugs for every possible damage type that you can choose from freely), this field will return all of the possible damage types that are available to the weapon by default.
    pub damage_type_hashes: Vec<super::DamageTypeHash>,
    /// This is the list of all damage types that we know ahead of time the item can take on. Unfortunately, this does not preclude the possibility of something funky happening to give the item a damage type that cannot be predicted beforehand: for example, if some designer decides to create arbitrary non-reusable plugs that cause damage type to change.
    ///
    /// This damage type prediction will only use the following to determine potential damage types:
    /// - Intrinsic perks
    /// - Talent Node perks
    /// - Known, reusable plugs for sockets
    pub damage_types: Vec<super::DamageType>,
    /// If the item has a damage type that could be considered to be default, it will be populated here.
    ///
    /// For various upsetting reasons, it's surprisingly cumbersome to figure this out. I hope you're happy.
    pub default_damage_type: super::DamageType,
    /// Similar to defaultDamageType, but represented as the hash identifier for a DestinyDamageTypeDefinition.
    ///
    /// I will likely regret leaving in the enumeration versions of these properties, but for now they're very convenient.
    pub default_damage_type_hash: Option<super::DamageTypeHash>,
    /// If this item is related directly to a Season of Destiny, this is the hash identifier for that season.
    pub season_hash: Option<super::SeasonHash>,
    /// If true, this is a dummy vendor-wrapped item template. Items purchased from Eververse will be "wrapped" by one of these items so that we can safely provide refund capabilities before the item is "unwrapped".
    pub is_wrapper: bool,
    /// Traits are metadata tags applied to this item. For example: armor slot, weapon type, foundry, faction, etc. These IDs come from the game and don't map to any content, but should still be useful.
    pub trait_ids: Vec<String>,
    /// These are the corresponding trait definition hashes for the entries in traitIds.
    pub trait_hashes: Vec<super::TraitHash>,
    #[serde(flatten)]
    pub definition: Definition<super::ItemHash>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct ItemTooltipNotification {
    pub display_string: String,
    pub display_style: String,
}

/// If an item can have an action performed on it (like "Dismantle"), it will be defined here if you care.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct ItemActionBlockDefinition {
    /// Localized text for the verb of the action being performed.
    pub verb_name: String,
    /// Localized text describing the action being performed.
    pub verb_description: String,
    /// The content has this property, however it's not entirely clear how it is used.
    pub is_positive: bool,
    /// If the action has an overlay screen associated with it, this is the name of that screen. Unfortunately, we cannot return the screen's data itself.
    pub overlay_screen_name: String,
    /// The icon associated with the overlay screen for the action, if any.
    pub overlay_icon: String,
    /// The number of seconds to delay before allowing this action to be performed again.
    pub required_cooldown_seconds: i32,
    /// If the action requires other items to exist or be destroyed, this is the list of those items and requirements.
    pub required_items: Vec<ItemActionRequiredItemDefinition>,
    /// If performing this action earns you Progression, this is the list of progressions and values granted for those progressions by performing this action.
    pub progression_rewards: Vec<ProgressionRewardDefinition>,
    /// The internal identifier for the action.
    pub action_type_label: String,
    /// Theoretically, an item could have a localized string for a hint about the location in which the action should be performed. In practice, no items yet have this property.
    pub required_location: String,
    /// The identifier hash for the Cooldown associated with this action. We have not pulled this data yet for you to have more data to use for cooldowns.
    pub required_cooldown_hash: Option<super::CooldownHash>,
    /// If true, the item is deleted when the action completes.
    pub delete_on_action: bool,
    /// If true, the entire stack is deleted when the action completes.
    pub consume_entire_stack: bool,
    /// If true, this action will be performed as soon as you earn this item. Some rewards work this way, providing you a single item to pick up from a reward-granting vendor in-game and then immediately consuming itself to provide you multiple items.
    pub use_on_acquire: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct ItemActionRequiredItemDefinition {
    /// The minimum quantity of the item you have to have.
    pub count: i32,
    /// The hash identifier of the item you have to have. Use it to look up the DestinyInventoryItemDefinition of the required item.
    pub item_hash: super::ItemHash,
    /// If true, the item/quantity will be deleted from your inventory when the action is performed. Otherwise, you'll retain these required items after the action is complete.
    pub delete_on_action: bool,
}

/// Inventory Items can reward progression when actions are performed on them. A common example of this in Destiny 1 was Bounties, which would reward Experience on your Character and the like when you completed the bounty.
///
/// Note that this maps to a DestinyProgressionMappingDefinition, and *not* a DestinyProgressionDefinition directly. This is apparently so that multiple progressions can be granted progression points/experience at the same time.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct ProgressionRewardDefinition {
    /// The hash identifier of the DestinyProgressionMappingDefinition that contains the progressions for which experience should be applied.
    pub progression_mapping_hash: super::ProgressionMappingHash,
    /// The amount of experience to give to each of the mapped progressions.
    pub amount: i32,
    /// If true, the game's internal mechanisms to throttle progression should be applied.
    pub apply_throttles: bool,
}

/// Aggregations of multiple progressions.
///
/// These are used to apply rewards to multiple progressions at once. They can sometimes have human readable data as well, but only extremely sporadically.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct ProgressionMappingDefinition {
    /// Infrequently defined in practice. Defer to the individual progressions' display properties.
    pub display_properties: DisplayPropertiesDefinition,
    /// The localized unit of measurement for progression across the progressions defined in this mapping. Unfortunately, this is very infrequently defined. Defer to the individual progressions' display units.
    pub display_units: String,
    #[serde(flatten)]
    pub definition: Definition<super::ProgressionMappingHash>,
}

/// If an item can have an action performed on it (like "Dismantle"), it will be defined here if you care.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct ItemCraftingBlockDefinition {
    /// A reference to the item definition that is created when crafting with this 'recipe' item.
    pub output_item_hash: super::ItemHash,
    /// A list of socket type hashes that describes which sockets are required for crafting with this recipe.
    pub required_socket_type_hashes: Vec<super::SocketTypeHash>,
    pub failed_requirement_strings: Vec<String>,
    /// A reference to the base material requirements for crafting with this recipe.
    pub base_material_requirements: super::MaterialRequirementSetHash,
    /// A list of 'bonus' socket plugs that may be available if certain requirements are met.
    pub bonus_plugs: Vec<ItemCraftingBlockBonusPlugDefinition>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct ItemCraftingBlockBonusPlugDefinition {
    pub socket_type_hash: super::SocketTypeHash,
    pub plug_item_hash: super::ItemHash,
}

/// Represent a set of material requirements: Items that either need to be owned or need to be consumed in order to perform an action.
///
/// A variety of other entities refer to these as gatekeepers and payments for actions that can be performed in game.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct MaterialRequirementSetDefinition {
    /// The list of all materials that are required.
    pub materials: Vec<MaterialRequirement>,
    #[serde(flatten)]
    pub definition: Definition<super::MaterialRequirementSetHash>,
}

/// Many actions relating to items require you to expend materials: - Activating a talent node - Inserting a plug into a socket The items will refer to material requirements by a materialRequirementsHash in these cases, and this is the definition for those requirements in terms of the item required, how much of it is required and other interesting info. This is one of the rare/strange times where a single contract class is used both in definitions *and* in live data response contracts. I'm not sure yet whether I regret that.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct MaterialRequirement {
    /// The hash identifier of the material required. Use it to look up the material's DestinyInventoryItemDefinition.
    pub item_hash: super::ItemHash,
    /// If True, the material will be removed from the character's inventory when the action is performed.
    pub delete_on_action: bool,
    /// The amount of the material required.
    pub count: i32,
    /// If true, the material requirement count value is constant. Since The Witch Queen expansion, some material requirement counts can be dynamic and will need to be returned with an API call.
    pub count_is_constant: bool,
    /// If True, this requirement is "silent": don't bother showing it in a material requirements display. I mean, I'm not your mom: I'm not going to tell you you *can't* show it. But we won't show it in our UI.
    pub omit_from_requirements: bool,
}

/// If the item can exist in an inventory - the overwhelming majority of them can and do - then this is the basic properties regarding the item's relationship with the inventory.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct ItemInventoryBlockDefinition {
    /// If this string is populated, you can't have more than one stack with this label in a given inventory. Note that this is different from the equipping block's unique label, which is used for equipping uniqueness.
    pub stack_unique_label: String,
    /// The maximum quantity of this item that can exist in a stack.
    pub max_stack_size: i32,
    /// The hash identifier for the DestinyInventoryBucketDefinition to which this item belongs. I should have named this "bucketHash", but too many things refer to it now. Sigh.
    pub bucket_type_hash: super::InventoryBucketHash,
    /// If the item is picked up by the lost loot queue, this is the hash identifier for the DestinyInventoryBucketDefinition into which it will be placed. Again, I should have named this recoveryBucketHash instead.
    pub recovery_bucket_type_hash: super::InventoryBucketHash,
    /// The hash identifier for the Tier Type of the item, use to look up its DestinyItemTierTypeDefinition if you need to show localized data for the item's tier.
    pub tier_type_hash: super::ItemTierTypeHash,
    /// If TRUE, this item is instanced. Otherwise, it is a generic item that merely has a quantity in a stack (like Glimmer).
    pub is_instance_item: bool,
    /// The localized name of the tier type, which is a useful shortcut so you don't have to look up the definition every time. However, it's mostly a holdover from days before we had a DestinyItemTierTypeDefinition to refer to.
    pub tier_type_name: String,
    /// The enumeration matching the tier type of the item to known values, again for convenience sake.
    pub tier_type: super::TierType,
    /// The tooltip message to show, if any, when the item expires.
    pub expiration_tooltip: String,
    /// If the item expires while playing in an activity, we show a different message.
    pub expired_in_activity_message: String,
    /// If the item expires in orbit, we show a... more different message. ("Consummate V's, consummate!")
    pub expired_in_orbit_message: String,
    pub suppress_expiration_when_objectives_complete: bool,
    /// A reference to the associated crafting 'recipe' item definition, if this item can be crafted.
    pub recipe_item_hash: super::ItemHash,
}

/// An Inventory (be it Character or Profile level) is comprised of many Buckets. An example of a bucket is "Primary Weapons", where all of the primary weapons on a character are gathered together into a single visual element in the UI: a subset of the inventory that has a limited number of slots, and in this case also has an associated Equipment Slot for equipping an item in the bucket.
///
/// Item definitions declare what their "default" bucket is ([InventoryItemDefinition].inventory.bucketTypeHash), and Item instances will tell you which bucket they are currently residing in (DestinyItemComponent.bucketHash). You can use this information along with the DestinyInventoryBucketDefinition to show these items grouped by bucket.
///
/// You cannot transfer an item to a bucket that is not its Default without going through a Vendor's "accepted items" (DestinyVendorDefinition.acceptedItems). This is how transfer functionality like the Vault is implemented, as a feature of a Vendor. See the vendor's acceptedItems property for more details.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct InventoryBucketDefinition {
    pub display_properties: DisplayPropertiesDefinition,
    /// Where the bucket is found. 0 = Character, 1 = Account
    pub scope: super::BucketScope,
    /// An enum value for what items can be found in the bucket. See the BucketCategory enum for more details.
    pub category: super::BucketCategory,
    /// Use this property to provide a quick-and-dirty recommended ordering for buckets in the UI. Most UIs will likely want to forsake this for something more custom and manual.
    pub bucket_order: i32,
    /// The maximum # of item "slots" in a bucket. A slot is a given combination of item + quantity.
    ///
    /// For instance, a Weapon will always take up a single slot, and always have a quantity of 1. But a material could take up only a single slot with hundreds of quantity.
    pub item_count: i32,
    /// Sometimes, inventory buckets represent conceptual "locations" in the game that might not be expected. This value indicates the conceptual location of the bucket, regardless of where it is actually contained on the character/account.
    ///
    /// See ItemLocation for details.
    ///
    /// Note that location includes the Vault and the Postmaster (both of whom being just inventory buckets with additional actions that can be performed on them through a Vendor)
    pub location: super::ItemLocation,
    /// If TRUE, there is at least one Vendor that can transfer items to/from this bucket. See the DestinyVendorDefinition's acceptedItems property for more information on how transferring works.
    pub has_transfer_destination: bool,
    /// If True, this bucket is enabled. Disabled buckets may include buckets that were included for test purposes, or that were going to be used but then were abandoned but never removed from content *cough*.
    pub enabled: bool,
    /// if a FIFO bucket fills up, it will delete the oldest item from said bucket when a new item tries to be added to it. If this is FALSE, the bucket will not allow new items to be placed in it until room is made by the user manually deleting items from it. You can see an example of this with the Postmaster's bucket.
    pub fifo: bool,
    #[serde(flatten)]
    pub definition: Definition<super::InventoryBucketHash>,
}

/// Primarily for Quests, this is the definition of properties related to the item if it is a quest and its various quest steps.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct ItemSetBlockDefinition {
    /// A collection of hashes of set items, for items such as Quest Metadata items that possess this data.
    pub item_list: Vec<ItemSetBlockEntryDefinition>,
    /// If true, items in the set can only be added in increasing order, and adding an item will remove any previous item. For Quests, this is by necessity true. Only one quest step is present at a time, and previous steps are removed as you advance in the quest.
    pub require_ordered_set_item_add: bool,
    /// If true, the UI should treat this quest as "featured"
    pub set_is_featured: bool,
    /// A string identifier we can use to attempt to identify the category of the Quest.
    pub set_type: String,
    /// The name of the quest line that this quest step is a part of.
    pub quest_line_name: String,
    /// The description of the quest line that this quest step is a part of.
    pub quest_line_description: String,
    /// An additional summary of this step in the quest line.
    pub quest_step_summary: String,
}

/// Defines a particular entry in an ItemSet (AKA a particular Quest Step in a Quest)
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct ItemSetBlockEntryDefinition {
    /// Used for tracking which step a user reached. These values will be populated in the user's internal state, which we expose externally as a more usable DestinyQuestStatus object. If this item has been obtained, this value will be set in trackingUnlockValueHash.
    pub tracking_value: i32,
    /// This is the hash identifier for a DestinyInventoryItemDefinition representing this quest step.
    pub item_hash: super::ItemHash,
}

/// Information about the item's calculated stats, with as much data as we can find for the stats without having an actual instance of the item.
///
/// Note that this means the entire concept of providing these stats is fundamentally insufficient: we cannot predict with 100% accuracy the conditions under which an item can spawn, so we use various heuristics to attempt to simulate the conditions as accurately as possible. Actual stats for items in-game can and will vary, but these should at least be useful base points for comparison and display.
///
/// It is also worth noting that some stats, like Magazine size, have further calculations performed on them by scripts in-game and on the game servers that BNet does not have access to. We cannot know how those stats are further transformed, and thus some stats will be inaccurate even on instances of items in BNet vs. how they appear in-game. This is a known limitation of our item statistics, without any planned fix.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct ItemStatBlockDefinition {
    /// If true, the game won't show the "primary" stat on this item when you inspect it.
    ///
    /// NOTE: This is being manually mapped, because I happen to want it in a block that isn't going to directly create this derivative block.
    pub disable_primary_stat_display: bool,
    /// If the item's stats are meant to be modified by a DestinyStatGroupDefinition, this will be the identifier for that definition.
    ///
    /// If you are using live data or precomputed stats data on the DestinyInventoryItemDefinition.stats.stats property, you don't have to worry about statGroupHash and how it alters stats: the already altered stats are provided to you. But if you want to see how the sausage gets made, or perform computations yourself, this is valuable information.
    pub stat_group_hash: super::StatGroupHash,
    /// If you are looking for precomputed values for the stats on a weapon, this is where they are stored. Technically these are the "Display" stat values. Please see DestinyStatsDefinition for what Display Stat Values means, it's a very long story... but essentially these are the closest values BNet can get to the item stats that you see in-game.
    ///
    /// These stats are keyed by the DestinyStatDefinition's hash identifier for the stat that's found on the item.
    pub stats: HashMap<super::StatDefinitionHash, InventoryItemStatDefinition>,
    /// A quick and lazy way to determine whether any stat other than the "primary" stat is actually visible on the item. Items often have stats that we return in case people find them useful, but they're not part of the "Stat Group" and thus we wouldn't display them in our UI. If this is False, then we're not going to display any of these stats other than the primary one.
    pub has_displayable_stats: bool,
    /// This stat is determined to be the "primary" stat, and can be looked up in the stats or any other stat collection related to the item.
    ///
    /// Use this hash to look up the stat's value using DestinyInventoryItemDefinition.stats.stats, and the renderable data for the primary stat in the related DestinyStatDefinition.
    pub primary_base_stat_hash: super::StatDefinitionHash,
}

/// Defines a specific stat value on an item, and the minimum/maximum range that we could compute for the item based on our heuristics for how the item might be generated.
///
/// Not guaranteed to match real-world instances of the item, but should hopefully at least be close. If it's not close, let us know on the Bungie API forums.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct InventoryItemStatDefinition {
    /// The hash for the DestinyStatDefinition representing this stat.
    pub stat_hash: super::StatDefinitionHash,
    /// This value represents the stat value assuming the minimum possible roll but accounting for any mandatory bonuses that should be applied to the stat on item creation.
    ///
    /// In Destiny 1, this was different from the "minimum" value because there were certain conditions where an item could be theoretically lower level/value than the initial roll.
    ///
    /// In Destiny 2, this is not possible unless Talent Grids begin to be used again for these purposes or some other system change occurs... thus in practice, value and minimum should be the same in Destiny 2. Good riddance.
    pub value: i32,
    /// The minimum possible value for this stat that we think the item can roll.
    pub minimum: i32,
    /// The maximum possible value for this stat that we think the item can roll.
    ///
    /// WARNING: In Destiny 1, this field was calculated using the potential stat rolls on the item's talent grid. In Destiny 2, items no longer have meaningful talent grids and instead have sockets: but the calculation of this field was never altered to adapt to this change. As such, this field should be considered deprecated until we can address this oversight.
    #[deprecated(note = "deprecated in accordance with API spec")]
    pub maximum: i32,
    pub display_maximum: Option<i32>,
}

/// Items that can be equipped define this block. It contains information we need to understand how and when the item can be equipped.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct EquippingBlockDefinition {
    /// If the item is part of a gearset, this is a reference to that gearset item.
    pub gearset_item_hash: super::ItemHash,
    /// If defined, this is the label used to check if the item has other items of matching types already equipped.
    ///
    /// For instance, when you aren't allowed to equip more than one Exotic Weapon, that's because all exotic weapons have identical uniqueLabels and the game checks the to-be-equipped item's uniqueLabel vs. all other already equipped items (other than the item in the slot that's about to be occupied).
    pub unique_label: String,
    /// The hash of that unique label. Does not point to a specific definition.
    pub unique_label_hash: super::LabelHash,
    /// An equipped item *must* be equipped in an Equipment Slot. This is the hash identifier of the DestinyEquipmentSlotDefinition into which it must be equipped.
    pub equipment_slot_type_hash: super::EquipmentSlotHash,
    /// These are custom attributes on the equippability of the item.
    ///
    /// For now, this can only be "equip on acquire", which would mean that the item will be automatically equipped as soon as you pick it up.
    pub attributes: Option<BitFlags<super::EquippingItemBlockAttributes>>,
    /// Ammo type used by a weapon is no longer determined by the bucket in which it is contained. If the item has an ammo type - i.e. if it is a weapon - this will be the type of ammunition expected.
    pub ammo_type: Option<super::AmmunitionType>,
    /// These are strings that represent the possible Game/Account/Character state failure conditions that can occur when trying to equip the item. They match up one-to-one with requiredUnlockExpressions.
    pub display_strings: Vec<String>,
}

/// This Block defines the rendering data associated with the item, if any.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct ItemTranslationBlockDefinition {
    pub weapon_pattern_identifier: String,
    pub weapon_pattern_hash: super::SandboxPatternHash,
    pub default_dyes: Vec<super::DyeReference>,
    pub locked_dyes: Vec<super::DyeReference>,
    pub custom_dyes: Vec<super::DyeReference>,
    pub arrangements: Vec<GearArtArrangementReference>,
    pub has_geometry: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct GearArtArrangementReference {
    pub class_hash: super::ClassHash,
    pub art_arrangement_hash: super::ArtArrangementHash,
}

/// Items like Sacks or Boxes can have items that it shows in-game when you view details that represent the items you can obtain if you use or acquire the item.
///
/// This defines those categories, and gives some insights into that data's source.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct ItemPreviewBlockDefinition {
    /// A string that the game UI uses as a hint for which detail screen to show for the item. You, too, can leverage this for your own custom screen detail views. Note, however, that these are arbitrarily defined by designers: there's no guarantees of a fixed, known number of these - so fall back to something reasonable if you don't recognize it.
    pub screen_style: String,
    /// If the preview data is derived from a fake "Preview" Vendor, this will be the hash identifier for the DestinyVendorDefinition of that fake vendor.
    pub preview_vendor_hash: super::VendorHash,
    /// If this item should show you Artifact information when you preview it, this is the hash identifier of the DestinyArtifactDefinition for the artifact whose data should be shown.
    pub artifact_hash: Option<super::ArtifactHash>,
    /// If the preview has an associated action (like "Open"), this will be the localized string for that action.
    pub preview_action_string: String,
    /// This is a list of the items being previewed, categorized in the same way as they are in the preview UI.
    pub derived_item_categories: Vec<items::DerivedItemCategoryDefinition>,
}

/// An item's "Quality" determines its calculated stats. The Level at which the item spawns is combined with its "qualityLevel" along with some additional calculations to determine the value of those stats.
///
/// In Destiny 2, most items don't have default item levels and quality, making this property less useful: these apparently are almost always determined by the complex mechanisms of the Reward system rather than statically. They are still provided here in case they are still useful for people. This also contains some information about Infusion.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct ItemQualityBlockDefinition {}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct ProgressionRewardItemQuantity {
    pub reward_at_progression_level: i32,
    pub acquisition_behavior: super::ProgressionRewardItemAcquisitionBehavior,
    pub ui_display_style: String,
    pub claim_unlock_display_strings: Vec<String>,
    /// The hash identifier for the item in question. Use it to look up the item's DestinyInventoryItemDefinition.
    pub item_hash: super::ItemHash,
    /// If this quantity is referring to a specific instance of an item, this will have the item's instance ID. Normally, this will be null.
    pub item_instance_id: Option<i64>,
    /// The amount of the item needed/available depending on the context of where DestinyItemQuantity is being used.
    pub quantity: i32,
    /// Indicates that this item quantity may be conditionally shown or hidden, based on various sources of state. For example: server flags, account state, or character progress.
    pub hash_conditional_visibility: bool,
}
