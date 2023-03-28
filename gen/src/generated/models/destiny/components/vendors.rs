use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use serde_repr::{Serialize_repr, Deserialize_repr};
use serde_with::{serde_as, DisplayFromStr};

/// This component contains essential/summary information about the vendor from the perspective of a character-agnostic view.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DestinyPublicVendorComponent {

    /// If True, the Vendor is currently accessible. 
/// If False, they may not actually be visible in the world at the moment.
    pub enabled: bool,
    /// The date when this vendor's inventory will next rotate/refresh.
/// Note that this is distinct from the date ranges that the vendor is visible/available in-game: this field indicates the specific time when the vendor's available items refresh and rotate, regardless of whether the vendor is actually available at that time. Unfortunately, these two values may be (and are, for the case of important vendors like Xur) different.
/// Issue https://github.com/Bungie-net/api/issues/353 is tracking a fix to start providing visibility date ranges where possible in addition to this refresh date, so that all important dates for vendors are available for use.
    pub next_refresh_date: chrono::DateTime<chrono::Utc>,
    /// The unique identifier for the vendor. Use it to look up their DestinyVendorDefinition.
    pub vendor_hash: crate::id::Id<crate::generated::models::destiny::definitions::DestinyVendorDefinition>,
}

/// Has character-agnostic information about an item being sold by a vendor.
/// Note that if you want instance, stats, etc... data for the item, you'll have to request additional components such as ItemInstances, ItemPerks etc... and acquire them from the DestinyVendorResponse's "items" property. For most of these, however, you'll have to ask for it in context of a specific character.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DestinyPublicVendorSaleItemComponent {

    /// If true, this item can be purchased through the Bungie.net API.
    pub api_purchasable: Option<bool>,
    /// A summary of the current costs of the item.
    pub costs: i32,
    /// The hash of the item being sold, as a quick shortcut for looking up the DestinyInventoryItemDefinition of the sale item.
    pub item_hash: crate::id::Id<crate::generated::models::destiny::definitions::DestinyInventoryItemDefinition>,
    /// If this item has its own custom date where it may be removed from the Vendor's rotation, this is that date.
/// Note that there's not actually any guarantee that it will go away: it could be chosen again and end up still being in the Vendor's sale items! But this is the next date where that test will occur, and is also the date that the game shows for availability on things like Bounties being sold. So it's the best we can give.
    pub override_next_refresh_date: Option<chrono::DateTime<chrono::Utc>>,
    /// If populated, this is the hash of the item whose icon (and other secondary styles, but *not* the human readable strings) should override whatever icons/styles are on the item being sold.
/// If you don't do this, certain items whose styles are being overridden by socketed items - such as the "Recycle Shader" item - would show whatever their default icon/style is, and it wouldn't be pretty or look accurate.
    pub override_style_item_hash: Option<crate::id::Id<crate::generated::models::destiny::definitions::DestinyInventoryItemDefinition>>,
    /// How much of the item you'll be getting.
    pub quantity: i32,
    /// The index into the DestinyVendorDefinition.itemList property. Note that this means Vendor data *is* Content Version dependent: make sure you have the latest content before you use Vendor data, or these indexes may mismatch. 
/// Most systems avoid this problem, but Vendors is one area where we are unable to reasonably avoid content dependency at the moment.
    pub vendor_item_index: i32,
}

/// This component contains essential/summary information about the vendor.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DestinyVendorBaseComponent {

    /// If True, the Vendor is currently accessible. 
/// If False, they may not actually be visible in the world at the moment.
    pub enabled: bool,
    /// The date when this vendor's inventory will next rotate/refresh.
/// Note that this is distinct from the date ranges that the vendor is visible/available in-game: this field indicates the specific time when the vendor's available items refresh and rotate, regardless of whether the vendor is actually available at that time. Unfortunately, these two values may be (and are, for the case of important vendors like Xur) different.
/// Issue https://github.com/Bungie-net/api/issues/353 is tracking a fix to start providing visibility date ranges where possible in addition to this refresh date, so that all important dates for vendors are available for use.
    pub next_refresh_date: chrono::DateTime<chrono::Utc>,
    /// The unique identifier for the vendor. Use it to look up their DestinyVendorDefinition.
    pub vendor_hash: crate::id::Id<crate::generated::models::destiny::definitions::DestinyVendorDefinition>,
}

/// Represents a specific group of vendors that can be rendered in the recommended order.
/// How do we figure out this order? It's a long story, and will likely get more complicated over time.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DestinyVendorGroup {

    /// No documentation provided.
    pub vendor_group_hash: crate::id::Id<crate::generated::models::destiny::definitions::DestinyVendorGroupDefinition>,
    /// The ordered list of vendors within a particular group.
    pub vendor_hashes: i32,
}

/// This component returns references to all of the Vendors in the response, grouped by categorizations that Bungie has deemed to be interesting, in the order in which both the groups and the vendors within that group should be rendered.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DestinyVendorGroupComponent {

    /// The ordered list of groups being returned.
    pub groups: i32,
}

/// The base class for Vendor Sale Item data. Has a bunch of character-agnostic state about the item being sold.
/// Note that if you want instance, stats, etc... data for the item, you'll have to request additional components such as ItemInstances, ItemPerks etc... and acquire them from the DestinyVendorResponse's "items" property.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DestinyVendorSaleItemBaseComponent {

    /// If true, this item can be purchased through the Bungie.net API.
    pub api_purchasable: Option<bool>,
    /// A summary of the current costs of the item.
    pub costs: i32,
    /// The hash of the item being sold, as a quick shortcut for looking up the DestinyInventoryItemDefinition of the sale item.
    pub item_hash: crate::id::Id<crate::generated::models::destiny::definitions::DestinyInventoryItemDefinition>,
    /// If this item has its own custom date where it may be removed from the Vendor's rotation, this is that date.
/// Note that there's not actually any guarantee that it will go away: it could be chosen again and end up still being in the Vendor's sale items! But this is the next date where that test will occur, and is also the date that the game shows for availability on things like Bounties being sold. So it's the best we can give.
    pub override_next_refresh_date: Option<chrono::DateTime<chrono::Utc>>,
    /// If populated, this is the hash of the item whose icon (and other secondary styles, but *not* the human readable strings) should override whatever icons/styles are on the item being sold.
/// If you don't do this, certain items whose styles are being overridden by socketed items - such as the "Recycle Shader" item - would show whatever their default icon/style is, and it wouldn't be pretty or look accurate.
    pub override_style_item_hash: Option<crate::id::Id<crate::generated::models::destiny::definitions::DestinyInventoryItemDefinition>>,
    /// How much of the item you'll be getting.
    pub quantity: i32,
    /// The index into the DestinyVendorDefinition.itemList property. Note that this means Vendor data *is* Content Version dependent: make sure you have the latest content before you use Vendor data, or these indexes may mismatch. 
/// Most systems avoid this problem, but Vendors is one area where we are unable to reasonably avoid content dependency at the moment.
    pub vendor_item_index: i32,
}
