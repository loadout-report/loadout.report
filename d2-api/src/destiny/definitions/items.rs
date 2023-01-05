use serde::{Deserialize, Serialize};
use crate::destiny;
use crate::destiny::definitions;

/// Defines the tier type of an item. Mostly this provides human readable properties for types like Common, Rare, etc...
///
/// It also provides some base data for infusion that could be useful.
#[derive(Clone, Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct ItemTierTypeDefinition {
    pub display_properties: definitions::common::DisplayPropertiesDefinition,
    /// If this tier defines infusion properties, they will be contained here.
    pub infusion_process: ItemTierTypeInfusionBlock,
    #[serde(flatten)]
    pub definition: definitions::Definition<destiny::ItemTierTypeHash>,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct ItemTierTypeInfusionBlock {
    /// The default portion of quality that will transfer from the infuser to the infusee item.
    /// (InfuserQuality - InfuseeQuality) * baseQualityTransferRatio = base quality transferred.
    pub base_quality_transfer_ratio: f32,
    /// As long as InfuserQuality > InfuseeQuality, the amount of quality bestowed is guaranteed to be at least this value, even if the transferRatio would dictate that it should be less. The total amount of quality that ends up in the Infusee cannot exceed the Infuser's quality however (for instance, if you infuse a 300 item with a 301 item and the minimum quality increment is 10, the infused item will not end up with 310 quality)
    pub minimum_quality_increment: i32,
}

/// A shortcut for the fact that some items have a "Preview Vendor" - See DestinyInventoryItemDefinition.preview.previewVendorHash - that is intended to be used to show what items you can get as a result of acquiring or using this item.
///
/// A common example of this in Destiny 1 was Eververse "Boxes," which could have many possible items. This "Preview Vendor" is not a vendor you can actually see in the game, but it defines categories and sale items for all of the possible items you could get from the Box so that the game can show them to you. We summarize that info here so that you don't have to do that Vendor lookup and aggregation manually.
#[derive(Clone, Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct DerivedItemCategoryDefinition {
    pub category_description: String,
    pub items: Vec<DerivedItemDefinition>,
}

/// This is a reference to, and summary data for, a specific item that you can get as a result of Using or Acquiring some other Item (For example, this could be summary information for an Emote that you can get by opening an an Eververse Box) See DestinyDerivedItemCategoryDefinition for more information.
#[derive(Clone, Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct DerivedItemDefinition {
    /// The hash for the DestinyInventoryItemDefinition of this derived item, if there is one. Sometimes we are given this information as a manual override, in which case there won't be an actual DestinyInventoryItemDefinition for what we display, but you can still show the strings from this object itself.
    pub item_hash: Option<destiny::ItemHash>,
    /// The name of the derived item.
    pub item_name: String,
    /// Additional details about the derived item, in addition to the description.
    pub item_detail: String,
    /// A brief description of the item.
    pub item_description: String,
    /// An icon for the item.
    pub icon_path: String,
    /// If the item was derived from a "Preview Vendor", this will be an index into the DestinyVendorDefinition's itemList property. Otherwise, -1.
    pub vendor_item_index: i32,
}

/// If an item is a Plug, its DestinyInventoryItemDefinition.plug property will be populated with an instance of one of these bad boys.
///
/// This gives information about when it can be inserted, what the plug's category is (and thus whether it is compatible with a socket... see DestinySocketTypeDefinition for information about Plug Categories and socket compatibility), whether it is enabled and other Plug info.
#[derive(Clone, Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct ItemPlugDefinition {
    /// The rules around when this plug can be inserted into a socket, aside from the socket's individual restrictions.
    ///
    /// The live data DestinyItemPlugComponent.insertFailIndexes will be an index into this array, so you can pull out the failure strings appropriate for the user.
    pub insertion_rules: Vec<PlugRuleDefinition>,
    /// The string identifier for the plug's category. Use the socket's DestinySocketTypeDefinition.plugWhitelist to determine whether this plug can be inserted into the socket.
    pub plug_category_identifier: String,
    /// The hash for the plugCategoryIdentifier. You can use this instead if you wish: I put both in the definition for debugging purposes.
    pub plug_category_hash: destiny::PlugCategoryHash,
    /// If you successfully socket the item, this will determine whether or not you get "refunded" on the plug.
    pub on_action_recreate_self: bool,
    /// If inserting this plug requires materials, this is the hash identifier for looking up the DestinyMaterialRequirementSetDefinition for those requirements.
    pub insertion_material_requirement_hash: destiny::MaterialRequirementSetHash,
    /// In the game, if you're inspecting a plug item directly, this will be the item shown with the plug attached. Look up the DestinyInventoryItemDefinition for this hash for the item.
    pub preview_item_override_hash: destiny::ItemHash,
    /// It's not enough for the plug to be inserted. It has to be enabled as well. For it to be enabled, it may require materials. This is the hash identifier for the DestinyMaterialRequirementSetDefinition for those requirements, if there is one.
    pub enabled_material_requirement_hash: destiny::MaterialRequirementSetHash,
    /// The rules around whether the plug, once inserted, is enabled and providing its benefits.
    ///
    /// The live data DestinyItemPlugComponent.enableFailIndexes will be an index into this array, so you can pull out the failure strings appropriate for the user.
    pub enabled_rules: Vec<PlugRuleDefinition>,
    /// Plugs can have arbitrary, UI-defined identifiers that the UI designers use to determine the style applied to plugs. Unfortunately, we have neither a definitive list of these labels nor advance warning of when new labels might be applied or how that relates to how they get rendered. If you want to, you can refer to known labels to change your own styles: but know that new ones can be created arbitrarily, and we have no way of associating the labels with any specific UI style guidance... you'll have to piece that together on your end. Or do what we do, and just show plugs more generically, without specialized styles.
    pub ui_plug_label: String,
    pub plug_style: destiny::PlugUiStyles,
    /// Indicates the rules about when this plug can be used. See the PlugAvailabilityMode enumeration for more information!
    pub plug_availability: destiny::PlugAvailabilityMode,
    /// If the plug meets certain state requirements, it may have an alternative label applied to it. This is the alternative label that will be applied in such a situation.
    pub alternate_ui_plug_label: String,
    /// The alternate plug of the plug: only applies when the item is in states that only the server can know about and control, unfortunately. See AlternateUiPlugLabel for the related label info.
    pub alternate_plug_style: destiny::PlugUiStyles,
    /// If TRUE, this plug is used for UI display purposes only, and doesn't have any interesting effects of its own.
    pub is_dummy_plug: bool,
    /// Do you ever get the feeling that a system has become so overburdened by edge cases that it probably should have become some other system entirely? So do I!
    ///
    /// In totally unrelated news, Plugs can now override properties of their parent items. This is some of the relevant definition data for those overrides.
    ///
    /// If this is populated, it will have the override data to be applied when this plug is applied to an item.
    pub parent_item_override: ParentItemOverride,
    /// IF not null, this plug provides Energy capacity to the item in which it is socketed. In Armor 2.0 for example, is implemented in a similar way to Masterworks, where visually it's a single area of the UI being clicked on to "Upgrade" to higher energy levels, but it's actually socketing new plugs.
    pub energy_capacity: EnergyCapacityEntry,
    /// IF not null, this plug has an energy cost. This contains the details of that cost.
    pub energy_cost: EnergyCostEntry,
}

/// Dictates a rule around whether the plug is enabled or insertable.
///
/// In practice, the live Destiny data will refer to these entries by index. You can then look up that index in the appropriate property (enabledRules or insertionRules) to get the localized string for the failure message if it failed.
#[derive(Clone, Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct PlugRuleDefinition {
    /// The localized string to show if this rule fails.
    pub failure_message: String,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct ParentItemOverride {
    pub additional_equip_requirements_display_strings: Vec<String>,
    pub pip_icon: String,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct EnergyCapacityEntry {
    pub capacity_value: i32,
    pub energy_type_hash: destiny::EnergyTypeHash,
    pub energy_type: destiny::EnergyType,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct EnergyCostEntry {
    pub energy_cost: i32,
    pub energy_type_hash: destiny::EnergyTypeHash,
    pub energy_type: destiny::EnergyType,
}
