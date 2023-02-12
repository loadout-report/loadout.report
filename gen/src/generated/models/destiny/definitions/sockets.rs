

use serde::{Serialize, Deserialize};


/// All Sockets have a "Type": a set of common properties that determine when the socket allows Plugs to be inserted, what Categories of Plugs can be inserted, and whether the socket is even visible at all given the current game/character/account state.
/// See DestinyInventoryItemDefinition for more information about Socketed items and Plugs.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DestinySocketTypeDefinition {

    /// This property indicates if the socket type determines whether Emblem icons and nameplates should be overridden by the inserted plug item's icon and nameplate.
    pub overrides_ui_appearance: bool,
    /// Defines what happens when a plug is inserted into sockets of this type.
    pub insert_action: i32,
    /// If this is true, then there is an entity with this identifier/type combination, but BNet is not yet allowed to show it. Sorry!
    pub redacted: bool,
    /// The unique identifier for this entity. Guaranteed to be unique for the type of entity, but not globally.
/// When entities refer to each other in Destiny content, it is this hash that they are referring to.
    pub hash: u32,
    /// Sometimes a socket isn't visible. These are some of the conditions under which sockets of this type are not visible. Unfortunately, the truth of visibility is much, much more complex. Best to rely on the live data for whether the socket is visible and enabled.
    pub visibility: i32,
    /// No documentation provided.
    pub socket_category_hash: u32,
    /// A list of Plug "Categories" that are allowed to be plugged into sockets of this type.
/// These should be compared against a given plug item's DestinyInventoryItemDefinition.plug.plugCategoryHash, which indicates the plug item's category.
/// If the plug's category matches any whitelisted plug, or if the whitelist is empty, it is allowed to be inserted.
    pub plug_whitelist: i32,
    /// No documentation provided.
    pub currency_scalars: i32,
    /// No documentation provided.
    pub is_preview_enabled: bool,
    /// There are fields for this display data, but they appear to be unpopulated as of now. I am not sure where in the UI these would show if they even were populated, but I will continue to return this data in case it becomes useful.
    pub display_properties: i32,
    /// No documentation provided.
    pub avoid_duplicates_on_initialization: bool,
    /// The index of the entity as it was found in the investment tables.
    pub index: i32,
    /// No documentation provided.
    pub hide_duplicate_reusable_plugs: bool,
    /// No documentation provided.
    pub always_randomize_sockets: bool,
}

/// Defines a plug "Category" that is allowed to be plugged into a socket of this type.
/// This should be compared against a given plug item's DestinyInventoryItemDefinition.plug.plugCategoryHash, which indicates the plug item's category.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DestinyPlugWhitelistEntryDefinition {

    /// The hash identifier of the Plug Category to compare against the plug item's plug.plugCategoryHash.
/// Note that this does NOT relate to any Definition in itself, it is only used for comparison purposes.
    pub category_hash: u32,
    /// The string identifier for the category, which is here mostly for debug purposes.
    pub category_identifier: String,
    /// The list of all plug items (DestinyInventoryItemDefinition) that the socket may randomly be populated with when reinitialized.
/// Which ones you should actually show are determined by the plug being inserted into the socket, and the socket’s type.
/// When you inspect the plug that could go into a Masterwork Socket, look up the socket type of the socket being inspected and find the DestinySocketTypeDefinition.
/// Then, look at the Plugs that can fit in that socket. Find the Whitelist in the DestinySocketTypeDefinition that matches the plug item’s categoryhash.
/// That whitelist entry will potentially have a new “reinitializationPossiblePlugHashes” property.If it does, that means we know what it will roll if you try to insert this plug into this socket.
    pub reinitialization_possible_plug_hashes: i32,
}

/// Sometimes, we have large sets of reusable plugs that are defined identically and thus can (and in some cases, are so large that they *must*) be shared across the places where they are used. These are the definitions for those reusable sets of plugs. 
///  See DestinyItemSocketEntryDefinition.plugSource and reusablePlugSetHash for the relationship between these reusable plug sets and the sockets that leverage them (for starters, Emotes).
///  As of the release of Shadowkeep (Late 2019), these will begin to be sourced from game content directly - which means there will be many more of them, but it also means we may not get all data that we used to get for them.
///  DisplayProperties, in particular, will no longer be guaranteed to contain valid information. We will make a best effort to guess what ought to be populated there where possible, but it will be invalid for many/most plug sets.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DestinyPlugSetDefinition {

    /// If this is true, then there is an entity with this identifier/type combination, but BNet is not yet allowed to show it. Sorry!
    pub redacted: bool,
    /// The index of the entity as it was found in the investment tables.
    pub index: i32,
    /// This is a list of pre-determined plugs that can be plugged into this socket, without the character having the plug in their inventory.
/// If this list is populated, you will not be allowed to plug an arbitrary item in the socket: you will only be able to choose from one of these reusable plugs.
    pub reusable_plug_items: i32,
    /// The unique identifier for this entity. Guaranteed to be unique for the type of entity, but not globally.
/// When entities refer to each other in Destiny content, it is this hash that they are referring to.
    pub hash: u32,
    /// Mostly for our debugging or reporting bugs, BNet is making "fake" plug sets in a desperate effort to reduce socket sizes.
///  If this is true, the plug set was generated by BNet: if it looks wrong, that's a good indicator that it's bungie.net that fucked this up.
    pub is_fake_plug_set: bool,
    /// If you want to show these plugs in isolation, these are the display properties for them.
    pub display_properties: i32,
}

/// Sockets on an item are organized into Categories visually.
/// You can find references to the socket category defined on an item's DestinyInventoryItemDefinition.sockets.socketCategories property.
/// This has the display information for rendering the categories' header, and a hint for how the UI should handle showing this category.
/// The shitty thing about this, however, is that the socket categories' UI style can be overridden by the item's UI style. For instance, the Socket Category used by Emote Sockets says it's "consumable," but that's a lie: they're all reusable, and overridden by the detail UI pages in ways that we can't easily account for in the API.
/// As a result, I will try to compile these rules into the individual sockets on items, and provide the best hint possible there through the plugSources property. In the future, I may attempt to use this information in conjunction with the item to provide a more usable UI hint on the socket layer, but for now improving the consistency of plugSources is the best I have time to provide. (See https://github.com/Bungie-net/api/issues/522 for more info)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DestinySocketCategoryDefinition {

    /// A string hinting to the game's UI system about how the sockets in this category should be displayed.
/// BNet doesn't use it: it's up to you to find valid values and make your own special UI if you want to honor this category style.
    pub ui_category_style: u32,
    /// The unique identifier for this entity. Guaranteed to be unique for the type of entity, but not globally.
/// When entities refer to each other in Destiny content, it is this hash that they are referring to.
    pub hash: u32,
    /// If this is true, then there is an entity with this identifier/type combination, but BNet is not yet allowed to show it. Sorry!
    pub redacted: bool,
    /// No documentation provided.
    pub display_properties: i32,
    /// The index of the entity as it was found in the investment tables.
    pub index: i32,
    /// Same as uiCategoryStyle, but in a more usable enumeration form.
    pub category_style: i32,
}

/// Data related to what happens while a plug is being inserted, mostly for UI purposes.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DestinyInsertPlugActionDefinition {

    /// How long it takes for the Plugging of the item to be completed once it is initiated, if you care.
    pub action_execute_seconds: i32,
    /// The type of action being performed when you act on this Socket Type. The most common value is "insert plug", but there are others as well (for instance, a "Masterwork" socket may allow for Re-initialization, and an Infusion socket allows for items to be consumed to upgrade the item)
    pub action_type: i32,
}

/// No documentation provided.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DestinySocketTypeScalarMaterialRequirementEntry {

    /// No documentation provided.
    pub scalar_value: i32,
    /// No documentation provided.
    pub currency_item_hash: u32,
}
