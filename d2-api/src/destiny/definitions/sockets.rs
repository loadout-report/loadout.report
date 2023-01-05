use serde::{Deserialize, Serialize};

/// All Sockets have a "Type": a set of common properties that determine when the socket allows Plugs to be inserted, what Categories of Plugs can be inserted, and whether the socket is even visible at all given the current game/character/account state.
///
/// See DestinyInventoryItemDefinition for more information about Socketed items and Plugs.
#[derive(Clone, Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct SocketTypeDefinition {
    /// There are fields for this display data, but they appear to be unpopulated as of now. I am not sure where in the UI these would show if they even were populated, but I will continue to return this data in case it becomes useful.
    pub display_properties: super::common::DisplayPropertiesDefinition,
    /// Defines what happens when a plug is inserted into sockets of this type.
    pub insert_action: InsertPlugActionDefinition,
    /// A list of Plug "Categories" that are allowed to be plugged into sockets of this type.
    ///
    /// These should be compared against a given plug item's DestinyInventoryItemDefinition.plug.plugCategoryHash, which indicates the plug item's category.
    ///
    /// If the plug's category matches any whitelisted plug, or if the whitelist is empty, it is allowed to be inserted.
    pub plug_whitelist: Vec<PlugWhitelistEntryDefinition>,
    pub socket_category_hash: super::super::SocketCategoryHash,
    /// Sometimes a socket isn't visible. These are some of the conditions under which sockets of this type are not visible. Unfortunately, the truth of visibility is much, much more complex. Best to rely on the live data for whether the socket is visible and enabled.
    pub visibility: super::super::SocketVisibility,
    pub always_randomize_sockets: bool,
    pub is_preview_enabled: bool,
    pub hide_duplicate_reusable_plugs: bool,
    /// This property indicates if the socket type determines whether Emblem icons and nameplates should be overridden by the inserted plug item's icon and nameplate.
    pub overrides_ui_appearance: bool,
    pub avoid_duplicates_on_initialization: bool,
    pub currency_scalars: Vec<SocketTypeScalarMaterialRequirementEntry>,

    #[serde(flatten)]
    pub definition: super::Definition<super::super::SocketTypeHash>,
}

/// Data related to what happens while a plug is being inserted, mostly for UI purposes.
#[derive(Clone, Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct InsertPlugActionDefinition {
    /// How long it takes for the Plugging of the item to be completed once it is initiated, if you care.
    pub action_execute_seconds: i32,
    /// The type of action being performed when you act on this Socket Type. The most common value is "insert plug", but there are others as well (for instance, a "Masterwork" socket may allow for Re-initialization, and an Infusion socket allows for items to be consumed to upgrade the item)
    pub action_type: SocketTypeActionType,
}

/// Indicates the type of actions that can be performed
#[derive(Clone, Serialize, Deserialize, Debug)]
pub enum SocketTypeActionType {
    InsertPlug = 0,
    InfuseItem = 1,
    ReinitializeSocket = 2,
}

/// Defines a plug "Category" that is allowed to be plugged into a socket of this type.
///
/// This should be compared against a given plug item's DestinyInventoryItemDefinition.plug.plugCategoryHash, which indicates the plug item's category.
#[derive(Clone, Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct PlugWhitelistEntryDefinition {
    /// The hash identifier of the Plug Category to compare against the plug item's plug.plugCategoryHash.
    ///
    /// Note that this does NOT relate to any Definition in itself, it is only used for comparison purposes.
    pub category_hash: super::super::PlugCategoryHash,
    /// The string identifier for the category, which is here mostly for debug purposes.
    pub category_identifier: String,
    /// The list of all plug items (DestinyInventoryItemDefinition) that the socket may randomly be populated with when reinitialized.
    ///
    /// Which ones you should actually show are determined by the plug being inserted into the socket, and the socket’s type.
    ///
    /// When you inspect the plug that could go into a Masterwork Socket, look up the socket type of the socket being inspected and find the DestinySocketTypeDefinition.
    ///
    /// Then, look at the Plugs that can fit in that socket. Find the Whitelist in the DestinySocketTypeDefinition that matches the plug item’s categoryhash.
    ///
    /// That whitelist entry will potentially have a new “reinitializationPossiblePlugHashes” property.If it does, that means we know what it will roll if you try to insert this plug into this socket.
    pub reinitialization_possible_plug_hashes: Vec<super::super::ItemHash>,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct SocketTypeScalarMaterialRequirementEntry {
    pub currency_item_hash: super::super::ItemHash,
    pub scalar_value: i32,
}

/// Sockets on an item are organized into Categories visually.
///
/// You can find references to the socket category defined on an item's DestinyInventoryItemDefinition.sockets.socketCategories property.
///
/// This has the display information for rendering the categories' header, and a hint for how the UI should handle showing this category.
///
/// The shitty thing about this, however, is that the socket categories' UI style can be overridden by the item's UI style. For instance, the Socket Category used by Emote Sockets says it's "consumable," but that's a lie: they're all reusable, and overridden by the detail UI pages in ways that we can't easily account for in the API.
///
/// As a result, I will try to compile these rules into the individual sockets on items, and provide the best hint possible there through the plugSources property. In the future, I may attempt to use this information in conjunction with the item to provide a more usable UI hint on the socket layer, but for now improving the consistency of plugSources is the best I have time to provide. (See https://github.com/Bungie-net/api/issues/522 for more info)
#[derive(Clone, Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct SocketCategoryDefinition {
    pub display_properties: super::common::DisplayPropertiesDefinition,
    /// A string hinting to the game's UI system about how the sockets in this category should be displayed.
    ///
    /// BNet doesn't use it: it's up to you to find valid values and make your own special UI if you want to honor this category style.
    pub ui_category_style: super::super::SocketCategoryStyle,
    /// Same as uiCategoryStyle, but in a more usable enumeration form.
    pub category_style: super::super::SocketCategoryStyle,
    #[serde(flatten)]
    pub definition: super::Definition<super::super::SocketCategoryHash>,
}