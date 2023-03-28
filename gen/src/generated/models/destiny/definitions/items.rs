use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use serde_repr::{Serialize_repr, Deserialize_repr};
use serde_with::{serde_as, DisplayFromStr};

/// A shortcut for the fact that some items have a "Preview Vendor" - See DestinyInventoryItemDefinition.preview.previewVendorHash - that is intended to be used to show what items you can get as a result of acquiring or using this item.
/// A common example of this in Destiny 1 was Eververse "Boxes," which could have many possible items. This "Preview Vendor" is not a vendor you can actually see in the game, but it defines categories and sale items for all of the possible items you could get from the Box so that the game can show them to you. We summarize that info here so that you don't have to do that Vendor lookup and aggregation manually.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DestinyDerivedItemCategoryDefinition {

    /// The localized string for the category title. This will be something describing the items you can get as a group, or your likelihood/the quantity you'll get.
    pub category_description: String,
    /// This is the list of all of the items for this category and the basic properties we'll know about them.
    pub items: i32,
}

/// This is a reference to, and summary data for, a specific item that you can get as a result of Using or Acquiring some other Item (For example, this could be summary information for an Emote that you can get by opening an an Eververse Box) See DestinyDerivedItemCategoryDefinition for more information.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DestinyDerivedItemDefinition {

    /// An icon for the item.
    pub icon_path: String,
    /// A brief description of the item.
    pub item_description: String,
    /// Additional details about the derived item, in addition to the description.
    pub item_detail: String,
    /// The hash for the DestinyInventoryItemDefinition of this derived item, if there is one. Sometimes we are given this information as a manual override, in which case there won't be an actual DestinyInventoryItemDefinition for what we display, but you can still show the strings from this object itself.
    pub item_hash: Option<u32>,
    /// The name of the derived item.
    pub item_name: String,
    /// If the item was derived from a "Preview Vendor", this will be an index into the DestinyVendorDefinition's itemList property. Otherwise, -1.
    pub vendor_item_index: i32,
}

/// Items can have Energy Capacity, and plugs can provide that capacity such as on a piece of Armor in Armor 2.0. This is how much "Energy" can be spent on activating plugs for this item.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DestinyEnergyCapacityEntry {

    /// How much energy capacity this plug provides.
    pub capacity_value: i32,
    /// The Energy Type for this energy capacity, in enum form for easy use.
    pub energy_type: crate::generated::models::destiny::DestinyEnergyType,
    /// Energy provided by a plug is always of a specific type - this is the hash identifier for the energy type for which it provides Capacity.
    pub energy_type_hash: crate::id::Id<crate::generated::models::destiny::definitions::energy_types::DestinyEnergyTypeDefinition>,
}

/// Some plugs cost Energy, which is a stat on the item that can be increased by other plugs (that, at least in Armor 2.0, have a "masterworks-like" mechanic for upgrading). If a plug has costs, the details of that cost are defined here.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DestinyEnergyCostEntry {

    /// The Energy cost for inserting this plug.
    pub energy_cost: i32,
    /// The type of energy that this plug costs, in enum form.
    pub energy_type: crate::generated::models::destiny::DestinyEnergyType,
    /// The type of energy that this plug costs, as a reference to the DestinyEnergyTypeDefinition of the energy type.
    pub energy_type_hash: crate::id::Id<crate::generated::models::destiny::definitions::energy_types::DestinyEnergyTypeDefinition>,
}

/// If an item is a Plug, its DestinyInventoryItemDefinition.plug property will be populated with an instance of one of these bad boys.
/// This gives information about when it can be inserted, what the plug's category is (and thus whether it is compatible with a socket... see DestinySocketTypeDefinition for information about Plug Categories and socket compatibility), whether it is enabled and other Plug info.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DestinyItemPlugDefinition {

    /// The alternate plug of the plug: only applies when the item is in states that only the server can know about and control, unfortunately. See AlternateUiPlugLabel for the related label info.
    pub alternate_plug_style: crate::generated::models::destiny::PlugUiStyles,
    /// If the plug meets certain state requirements, it may have an alternative label applied to it. This is the alternative label that will be applied in such a situation.
    pub alternate_ui_plug_label: String,
    /// It's not enough for the plug to be inserted. It has to be enabled as well. For it to be enabled, it may require materials. This is the hash identifier for the DestinyMaterialRequirementSetDefinition for those requirements, if there is one.
    pub enabled_material_requirement_hash: crate::id::Id<crate::generated::models::destiny::definitions::DestinyMaterialRequirementSetDefinition>,
    /// The rules around whether the plug, once inserted, is enabled and providing its benefits.
/// The live data DestinyItemPlugComponent.enableFailIndexes will be an index into this array, so you can pull out the failure strings appropriate for the user.
    pub enabled_rules: i32,
    /// IF not null, this plug provides Energy capacity to the item in which it is socketed. In Armor 2.0 for example, is implemented in a similar way to Masterworks, where visually it's a single area of the UI being clicked on to "Upgrade" to higher energy levels, but it's actually socketing new plugs.
    pub energy_capacity: crate::generated::models::destiny::definitions::items::DestinyEnergyCapacityEntry,
    /// IF not null, this plug has an energy cost. This contains the details of that cost.
    pub energy_cost: crate::generated::models::destiny::definitions::items::DestinyEnergyCostEntry,
    /// If inserting this plug requires materials, this is the hash identifier for looking up the DestinyMaterialRequirementSetDefinition for those requirements.
    pub insertion_material_requirement_hash: crate::id::Id<crate::generated::models::destiny::definitions::DestinyMaterialRequirementSetDefinition>,
    /// The rules around when this plug can be inserted into a socket, aside from the socket's individual restrictions.
/// The live data DestinyItemPlugComponent.insertFailIndexes will be an index into this array, so you can pull out the failure strings appropriate for the user.
    pub insertion_rules: i32,
    /// If TRUE, this plug is used for UI display purposes only, and doesn't have any interesting effects of its own.
    pub is_dummy_plug: bool,
    /// If you successfully socket the item, this will determine whether or not you get "refunded" on the plug.
    pub on_action_recreate_self: bool,
    /// Do you ever get the feeling that a system has become so overburdened by edge cases that it probably should have become some other system entirely? So do I!
/// In totally unrelated news, Plugs can now override properties of their parent items. This is some of the relevant definition data for those overrides.
/// If this is populated, it will have the override data to be applied when this plug is applied to an item.
    pub parent_item_override: crate::generated::models::destiny::definitions::items::DestinyParentItemOverride,
    /// Indicates the rules about when this plug can be used. See the PlugAvailabilityMode enumeration for more information!
    pub plug_availability: crate::generated::models::destiny::PlugAvailabilityMode,
    /// The hash for the plugCategoryIdentifier. You can use this instead if you wish: I put both in the definition for debugging purposes.
    pub plug_category_hash: u32,
    /// The string identifier for the plug's category. Use the socket's DestinySocketTypeDefinition.plugWhitelist to determine whether this plug can be inserted into the socket.
    pub plug_category_identifier: String,
    /// No documentation provided.
    pub plug_style: crate::generated::models::destiny::PlugUiStyles,
    /// In the game, if you're inspecting a plug item directly, this will be the item shown with the plug attached. Look up the DestinyInventoryItemDefinition for this hash for the item.
    pub preview_item_override_hash: crate::id::Id<crate::generated::models::destiny::definitions::DestinyInventoryItemDefinition>,
    /// Plugs can have arbitrary, UI-defined identifiers that the UI designers use to determine the style applied to plugs. Unfortunately, we have neither a definitive list of these labels nor advance warning of when new labels might be applied or how that relates to how they get rendered. If you want to, you can refer to known labels to change your own styles: but know that new ones can be created arbitrarily, and we have no way of associating the labels with any specific UI style guidance... you'll have to piece that together on your end. Or do what we do, and just show plugs more generically, without specialized styles.
    pub ui_plug_label: String,
}

/// Defines the tier type of an item. Mostly this provides human readable properties for types like Common, Rare, etc...
/// It also provides some base data for infusion that could be useful.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DestinyItemTierTypeDefinition {

    /// No documentation provided.
    pub display_properties: crate::generated::models::destiny::definitions::common::DestinyDisplayPropertiesDefinition,
    /// The unique identifier for this entity. Guaranteed to be unique for the type of entity, but not globally.
/// When entities refer to each other in Destiny content, it is this hash that they are referring to.
    pub hash: u32,
    /// The index of the entity as it was found in the investment tables.
    pub index: i32,
    /// If this tier defines infusion properties, they will be contained here.
    pub infusion_process: crate::generated::models::destiny::definitions::items::DestinyItemTierTypeInfusionBlock,
    /// If this is true, then there is an entity with this identifier/type combination, but BNet is not yet allowed to show it. Sorry!
    pub redacted: bool,
}

/// No documentation provided.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DestinyItemTierTypeInfusionBlock {

    /// The default portion of quality that will transfer from the infuser to the infusee item. (InfuserQuality - InfuseeQuality) * baseQualityTransferRatio = base quality transferred.
    pub base_quality_transfer_ratio: f32,
    /// As long as InfuserQuality > InfuseeQuality, the amount of quality bestowed is guaranteed to be at least this value, even if the transferRatio would dictate that it should be less. The total amount of quality that ends up in the Infusee cannot exceed the Infuser's quality however (for instance, if you infuse a 300 item with a 301 item and the minimum quality increment is 10, the infused item will not end up with 310 quality)
    pub minimum_quality_increment: i32,
}

/// No documentation provided.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DestinyParentItemOverride {

    /// No documentation provided.
    pub additional_equip_requirements_display_strings: i32,
    /// No documentation provided.
    pub pip_icon: String,
}

/// Dictates a rule around whether the plug is enabled or insertable.
/// In practice, the live Destiny data will refer to these entries by index. You can then look up that index in the appropriate property (enabledRules or insertionRules) to get the localized string for the failure message if it failed.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DestinyPlugRuleDefinition {

    /// The localized string to show if this rule fails.
    pub failure_message: String,
}
