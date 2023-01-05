use serde::{Deserialize, Serialize};

/// Defines the tier type of an item. Mostly this provides human readable properties for types like Common, Rare, etc...
///
/// It also provides some base data for infusion that could be useful.
#[derive(Clone, Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct ItemTierTypeDefinition {
    pub display_properties: super::common::DisplayPropertiesDefinition,
    /// If this tier defines infusion properties, they will be contained here.
    pub infusion_process: ItemTierTypeInfusionBlock,
    #[serde(flatten)]
    pub definition: super::Definition<super::super::ItemTierTypeHash>,
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
    pub item_hash: Option<super::super::ItemHash>,
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
