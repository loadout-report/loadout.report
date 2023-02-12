

use serde::{Serialize, Deserialize};


/// A response containing all of the components for all requested vendors.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InventoryChangedResponse {

    /// Items that appeared in the inventory possibly as a result of an action.
    pub added_inventory_items: i32,
    /// Items that disappeared from the inventory possibly as a result of an action.
    pub removed_inventory_items: i32,
}

/// The response contract for GetDestinyCharacter, with components that can be returned for character and item-level data.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DestinyCharacterResponse {

    /// Equipped items on the character.
/// COMPONENT TYPE: CharacterEquipment
    pub equipment: i32,
    /// COMPONENT TYPE: PresentationNodes
    pub presentation_nodes: i32,
    /// The set of components belonging to the player's instanced items.
/// COMPONENT TYPE: [See inside the DestinyItemComponentSet contract for component types.]
    pub item_components: i32,
    /// The set of components belonging to the player's UNinstanced items. Because apparently now those too can have information relevant to the character's state.
/// COMPONENT TYPE: [See inside the DestinyItemComponentSet contract for component types.]
    pub uninstanced_item_components: i32,
    /// Items available from Kiosks that are available to this specific character. 
/// COMPONENT TYPE: Kiosks
    pub kiosks: i32,
    /// COMPONENT TYPE: Collectibles
    pub collectibles: i32,
    /// Activity data - info about current activities available to the player.
/// COMPONENT TYPE: CharacterActivities
    pub activities: i32,
    /// Base information about the character in question.
/// COMPONENT TYPE: Characters
    pub character: i32,
    /// When sockets refer to reusable Plug Sets (see DestinyPlugSetDefinition for more info), this is the set of plugs and their states that are scoped to this character.
/// This comes back with ItemSockets, as it is needed for a complete picture of the sockets on requested items.
/// COMPONENT TYPE: ItemSockets
    pub plug_sets: i32,
    /// Character progression data, including Milestones.
/// COMPONENT TYPE: CharacterProgressions
    pub progressions: i32,
    /// Character rendering data - a minimal set of information about equipment and dyes used for rendering.
/// COMPONENT TYPE: CharacterRenderData
    pub render_data: i32,
    /// COMPONENT TYPE: Records
    pub records: i32,
    /// A "lookup" convenience component that can be used to quickly check if the character has access to items that can be used for purchasing.
/// COMPONENT TYPE: CurrencyLookups
    pub currency_lookups: i32,
    /// The character-level non-equipped inventory items.
/// COMPONENT TYPE: CharacterInventories
    pub inventory: i32,
}

/// A response containing all of the components for a vendor.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DestinyVendorResponse {

    /// Item components, keyed by the vendorItemIndex of the active sale items.
/// COMPONENT TYPE: [See inside the DestinyItemComponentSet contract for component types.]
    pub item_components: i32,
    /// A "lookup" convenience component that can be used to quickly check if the character has access to items that can be used for purchasing.
/// COMPONENT TYPE: CurrencyLookups
    pub currency_lookups: i32,
    /// The base properties of the vendor.
/// COMPONENT TYPE: Vendors
    pub vendor: i32,
    /// Sales, keyed by the vendorItemIndex of the item being sold.
/// COMPONENT TYPE: VendorSales
    pub sales: i32,
    /// Categories that the vendor has available, and references to the sales therein.
/// COMPONENT TYPE: VendorCategories
    pub categories: i32,
    /// A map of string variable values by hash for this character context.
/// COMPONENT TYPE: StringVariables
    pub string_variables: i32,
}

/// A response containing all valid components for the public Vendors endpoint.
///  It is a decisively smaller subset of data compared to what we can get when we know the specific user making the request.
///  If you want any of the other data - item details, whether or not you can buy it, etc... you'll have to call in the context of a character. I know, sad but true.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DestinyPublicVendorsResponse {

    /// Categories that the vendor has available, and references to the sales therein. These are keyed by the Vendor Hash, so you will get one Categories Component per vendor returned.
/// COMPONENT TYPE: VendorCategories
    pub categories: i32,
    /// A set of string variable values by hash for a public vendors context.
/// COMPONENT TYPE: StringVariables
    pub string_variables: i32,
    /// Sales, keyed by the vendorItemIndex of the item being sold. These are keyed by the Vendor Hash, so you will get one Sale Item Set Component per vendor returned.
/// Note that within the Sale Item Set component, the sales are themselves keyed by the vendorSaleIndex, so you can relate it to the corrent sale item definition within the Vendor's definition.
/// COMPONENT TYPE: VendorSales
    pub sales: i32,
    /// For Vendors being returned, this will give you the information you need to group them and order them in the same way that the Bungie Companion app performs grouping. It will automatically be returned if you request the Vendors component.
/// COMPONENT TYPE: Vendors
    pub vendor_groups: i32,
    /// The base properties of the vendor. These are keyed by the Vendor Hash, so you will get one Vendor Component per vendor returned.
/// COMPONENT TYPE: Vendors
    pub vendors: i32,
}

/// The response object for retrieving an individual instanced item. None of these components are relevant for an item that doesn't have an "itemInstanceId": for those, get your information from the DestinyInventoryDefinition.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DestinyItemResponse {

    /// Information about how to render the item in 3D.
/// COMPONENT TYPE: ItemRenderData
    pub render_data: i32,
    /// Information specifically about the perks currently active on the item.
/// COMPONENT TYPE: ItemPerks
    pub perks: i32,
    /// Common data for the item relevant to its non-instanced properties.
/// COMPONENT TYPE: ItemCommonData
    pub item: i32,
    /// Information about objectives on Plugs for a given item. See the component's documentation for more info.
/// COMPONENT TYPE: ItemPlugObjectives
    pub plug_objectives: i32,
    /// Information about the Reusable Plugs for sockets on an item. These are plugs that you can insert into the given socket regardless of if you actually own an instance of that plug: they are logic-driven plugs rather than inventory-driven.
///  These may need to be combined with Plug Set component data to get a full picture of available plugs on a given socket.
///  COMPONENT TYPE: ItemReusablePlugs
    pub reusable_plugs: i32,
    /// Information about the sockets of the item: which are currently active, what potential sockets you could have and the stats/abilities/perks you can gain from them.
/// COMPONENT TYPE: ItemSockets
    pub sockets: i32,
    /// Information about the computed stats of the item: power, defense, etc...
/// COMPONENT TYPE: ItemStats
    pub stats: i32,
    /// Information about the talent grid attached to the item. Talent nodes can provide a variety of benefits and abilities, and in Destiny 2 are used almost exclusively for the character's "Builds".
/// COMPONENT TYPE: ItemTalentGrids
    pub talent_grid: i32,
    /// Basic instance data for the item.
/// COMPONENT TYPE: ItemInstances
    pub instance: i32,
    /// Information specifically about the item's objectives.
/// COMPONENT TYPE: ItemObjectives
    pub objectives: i32,
    /// If the item is on a character, this will return the ID of the character that is holding the item.
    #[serde(with = "crate::unfuck_js::nullable_stringified_numbers")]
    pub character_id: Option<i64>,
}

/// No documentation provided.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PersonalDestinyVendorSaleItemSetComponent {

    /// No documentation provided.
    pub sale_items: i32,
}

/// No documentation provided.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PublicDestinyVendorSaleItemSetComponent {

    /// No documentation provided.
    pub sale_items: i32,
}

/// The response for GetDestinyProfile, with components for character and item-level data.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DestinyProfileResponse {

    /// Silver quantities for any platform on which this Profile plays destiny.
///  COMPONENT TYPE: PlatformSilver
    pub platform_silver: i32,
    /// COMPONENT TYPE: Transitory
    pub profile_transitory_data: i32,
    /// COMPONENT TYPE: PresentationNodes
    pub profile_presentation_nodes: i32,
    /// When we have progression information - such as Checklists - that may apply profile-wide, it will be returned here rather than in the per-character progression data.
/// COMPONENT TYPE: ProfileProgression
    pub profile_progression: i32,
    /// Character-level progression data, keyed by the Character's Id.
/// COMPONENT TYPE: CharacterProgressions
    pub character_progressions: i32,
    /// The profile-level currencies owned by the Destiny Profile.
/// COMPONENT TYPE: ProfileCurrencies
    pub profile_currencies: i32,
    /// The basic information about the Destiny Profile (formerly "Account").
/// COMPONENT TYPE: Profiles
    pub profile: i32,
    /// COMPONENT TYPE: Collectibles
    pub profile_collectibles: i32,
    /// Some secondary components are not tracked in the primary response timestamp and have their timestamp tracked here. If your component is any of the following, this field is where you will find your timestamp value:
///  PresentationNodes, Records, Collectibles, Metrics, StringVariables, Craftables, Transitory
///  All other component types may use the primary timestamp property.
    pub secondary_components_minted_timestamp: chrono::DateTime<chrono::Utc>,
    /// The profile-level inventory of the Destiny Profile.
/// COMPONENT TYPE: ProfileInventories
    pub profile_inventory: i32,
    /// Items available from Kiosks that are available Profile-wide (i.e. across all characters)
/// This component returns information about what Kiosk items are available to you on a *Profile* level. It is theoretically possible for Kiosks to have items gated by specific Character as well. If you ever have those, you will find them on the characterKiosks property.
/// COMPONENT TYPE: Kiosks
    pub profile_kiosks: i32,
    /// The character's equipped items, keyed by the Character's Id.
/// COMPONENT TYPE: CharacterEquipment
    pub character_equipment: i32,
    /// Records the timestamp of when most components were last generated from the world server source. Unless the component type is specified in the documentation for secondaryComponentsMintedTimestamp, this value is sufficient to do data freshness.
    pub response_minted_timestamp: chrono::DateTime<chrono::Utc>,
    /// COMPONENT TYPE: StringVariables
    pub character_string_variables: i32,
    /// When sockets refer to reusable Plug Sets (see DestinyPlugSetDefinition for more info), this is the set of plugs and their states that are profile-scoped.
/// This comes back with ItemSockets, as it is needed for a complete picture of the sockets on requested items.
/// COMPONENT TYPE: ItemSockets
    pub profile_plug_sets: i32,
    /// COMPONENT TYPE: Craftables
    pub character_craftables: i32,
    /// Basic information about each character, keyed by the CharacterId.
/// COMPONENT TYPE: Characters
    pub characters: i32,
    /// The character-level non-equipped inventory items, keyed by the Character's Id.
/// COMPONENT TYPE: CharacterInventories
    pub character_inventories: i32,
    /// Character rendering data - a minimal set of info needed to render a character in 3D - keyed by the Character's Id.
/// COMPONENT TYPE: CharacterRenderData
    pub character_render_data: i32,
    /// Do you ever get the feeling that a system was designed *too* flexibly? That it can be used in so many different ways that you end up being unable to provide an easy to use abstraction for the mess that's happening under the surface?
/// Let's talk about character-specific data that might be related to items without instances. These two statements are totally unrelated, I promise.
/// At some point during D2, it was decided that items - such as Bounties - could be given to characters and *not* have instance data, but that *could* display and even use relevant state information on your account and character.
/// Up to now, any item that had meaningful dependencies on character or account state had to be instanced, and thus "itemComponents" was all that you needed: it was keyed by item's instance IDs and provided the stateful information you needed inside.
/// Unfortunately, we don't live in such a magical world anymore. This is information held on a per-character basis about non-instanced items that the characters have in their inventory - or that reference character-specific state information even if it's in Account-level inventory - and the values related to that item's state in relation to the given character.
/// To give a concrete example, look at a Moments of Triumph bounty. They exist in a character's inventory, and show/care about a character's progression toward completing the bounty. But the bounty itself is a non-instanced item, like a mod or a currency. This returns that data for the characters who have the bounty in their inventory.
/// I'm not crying, you're crying Okay we're both crying but it's going to be okay I promise Actually I shouldn't promise that, I don't know if it's going to be okay
    pub character_uninstanced_item_components: i32,
    /// When sockets refer to reusable Plug Sets (see DestinyPlugSetDefinition for more info), this is the set of plugs and their states, per character, that are character-scoped.
/// This comes back with ItemSockets, as it is needed for a complete picture of the sockets on requested items.
/// COMPONENT TYPE: ItemSockets
    pub character_plug_sets: i32,
    /// COMPONENT TYPE: Records
    pub character_records: i32,
    /// COMPONENT TYPE: Metrics
    pub metrics: i32,
    /// COMPONENT TYPE: Records
    pub profile_records: i32,
    /// COMPONENT TYPE: StringVariables
    pub profile_string_variables: i32,
    /// Recent, refundable purchases you have made from vendors. When will you use it? Couldn't say...
/// COMPONENT TYPE: VendorReceipts
    pub vendor_receipts: i32,
    /// COMPONENT TYPE: Collectibles
    pub character_collectibles: i32,
    /// COMPONENT TYPE: PresentationNodes
    pub character_presentation_nodes: i32,
    /// A "lookup" convenience component that can be used to quickly check if the character has access to items that can be used for purchasing.
/// COMPONENT TYPE: CurrencyLookups
    pub character_currency_lookups: i32,
    /// Character activity data - the activities available to this character and its status, keyed by the Character's Id.
/// COMPONENT TYPE: CharacterActivities
    pub character_activities: i32,
    /// Items available from Kiosks that are available to a specific character as opposed to the account as a whole. It must be combined with data from the profileKiosks property to get a full picture of the character's available items to check out of a kiosk.
/// This component returns information about what Kiosk items are available to you on a *Character* level. Usually, kiosk items will be earned for the entire Profile (all characters) at once. To find those, look in the profileKiosks property.
/// COMPONENT TYPE: Kiosks
    pub character_kiosks: i32,
    /// Information about instanced items across all returned characters, keyed by the item's instance ID.
/// COMPONENT TYPE: [See inside the DestinyItemComponentSet contract for component types.]
    pub item_components: i32,
}

/// No documentation provided.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DestinyProfileUserInfoCard {

    /// A platform specific additional display name - ex: psn Real Name, bnet Unique Name, etc.
    pub supplemental_display_name: String,
    /// If there is a cross save override in effect, this value will tell you the type that is overridding this one.
    pub cross_save_override: i32,
    /// The list of Membership Types indicating the platforms on which this Membership can be used.
///  Not in Cross Save = its original membership type. Cross Save Primary = Any membership types it is overridding, and its original membership type Cross Save Overridden = Empty list
    pub applicable_membership_types: i32,
    /// No documentation provided.
    pub date_last_played: chrono::DateTime<chrono::Utc>,
    /// The bungie global display name code, if set.
    pub bungie_global_display_name_code: Option<i16>,
    /// URL the Icon if available.
    pub icon_path: String,
    /// If this profile is being overridden/obscured by Cross Save, this will be set to true. We will still return the profile for display purposes where users need to know the info: it is up to any given area of the app/site to determine if this profile should still be shown.
    pub is_overridden: bool,
    /// Display Name the player has chosen for themselves. The display name is optional when the data type is used as input to a platform API.
    pub display_name: String,
    /// This is the silver available on this Profile across any platforms on which they have purchased silver.
///  This is only available if you are requesting yourself.
    pub platform_silver: i32,
    /// The bungie global display name, if set.
    pub bungie_global_display_name: String,
    /// Type of the membership. Not necessarily the native type.
    pub membership_type: i32,
    /// If True, this is a public user membership.
    pub is_public: bool,
    /// Membership ID as they user is known in the Accounts service
    #[serde(with = "crate::unfuck_js::stringified_numbers")]
    pub membership_id: i64,
    /// If this profile is not in a cross save pairing, this will return the game versions that we believe this profile has access to.
///  For the time being, we will not return this information for any membership that is in a cross save pairing. The gist is that, once the pairing occurs, we do not currently have a consistent way to get that information for the profile's original Platform, and thus gameVersions would be too inconsistent (based on the last platform they happened to play on) for the info to be useful.
///  If we ever can get this data, this field will be deprecated and replaced with data on the DestinyLinkedProfileResponse itself, with game versions per linked Platform. But since we can't get that, we have this as a stop-gap measure for getting the data in the only situation that we currently need it.
    pub unpaired_game_versions: Option<i32>,
    /// If true, this account is hooked up as the "Primary" cross save account for one or more platforms.
    pub is_cross_save_primary: bool,
}

/// If a Destiny Profile can't be returned, but we're pretty certain it's a valid Destiny account, this will contain as much info as we can get about the profile for your use.
/// Assume that the most you'll get is the Error Code, the Membership Type and the Membership ID.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DestinyErrorProfile {

    /// Basic info about the account that failed. Don't expect anything other than membership ID, Membership Type, and displayName to be populated.
    pub info_card: i32,
    /// The error that we encountered. You should be able to look up localized text to show to the user for these failures.
    pub error_code: i32,
}

/// A response containing all of the components for all requested vendors.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DestinyVendorsResponse {

    /// Sales, keyed by the vendorItemIndex of the item being sold. These are keyed by the Vendor Hash, so you will get one Sale Item Set Component per vendor returned.
/// Note that within the Sale Item Set component, the sales are themselves keyed by the vendorSaleIndex, so you can relate it to the corrent sale item definition within the Vendor's definition.
/// COMPONENT TYPE: VendorSales
    pub sales: i32,
    /// For Vendors being returned, this will give you the information you need to group them and order them in the same way that the Bungie Companion app performs grouping. It will automatically be returned if you request the Vendors component.
/// COMPONENT TYPE: Vendors
    pub vendor_groups: i32,
    /// The set of item detail components, one set of item components per Vendor. These are keyed by the Vendor Hash, so you will get one Item Component Set per vendor returned.
/// The components contained inside are themselves keyed by the vendorSaleIndex, and will have whatever item-level components you requested (Sockets, Stats, Instance data etc...) per item being sold by the vendor.
    pub item_components: i32,
    /// Categories that the vendor has available, and references to the sales therein. These are keyed by the Vendor Hash, so you will get one Categories Component per vendor returned.
/// COMPONENT TYPE: VendorCategories
    pub categories: i32,
    /// The base properties of the vendor. These are keyed by the Vendor Hash, so you will get one Vendor Component per vendor returned.
/// COMPONENT TYPE: Vendors
    pub vendors: i32,
    /// A "lookup" convenience component that can be used to quickly check if the character has access to items that can be used for purchasing.
/// COMPONENT TYPE: CurrencyLookups
    pub currency_lookups: i32,
    /// A map of string variable values by hash for this character context.
/// COMPONENT TYPE: StringVariables
    pub string_variables: i32,
}

/// I know what you seek. You seek linked accounts. Found them, you have.
/// This contract returns a minimal amount of data about Destiny Accounts that are linked through your Bungie.Net account. We will not return accounts in this response whose
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DestinyLinkedProfilesResponse {

    /// If the requested membership had a linked Bungie.Net membership ID, this is the basic information about that BNet account.
/// I know, Tetron; I know this is mixing UserServices concerns with DestinyServices concerns. But it's so damn convenient! https://www.youtube.com/watch?v=X5R-bB-gKVI
    pub bnet_membership: i32,
    /// Any Destiny account for whom we could successfully pull characters will be returned here, as the Platform-level summary of user data. (no character data, no Destiny account data other than the Membership ID and Type so you can make further queries)
    pub profiles: i32,
    /// This is brief summary info for profiles that we believe have valid Destiny info, but who failed to return data for some other reason and thus we know that subsequent calls for their info will also fail.
    pub profiles_with_errors: i32,
}

/// No documentation provided.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DestinyItemChangeResponse {

    /// Items that appeared in the inventory possibly as a result of an action.
    pub added_inventory_items: i32,
    /// Items that disappeared from the inventory possibly as a result of an action.
    pub removed_inventory_items: i32,
    /// No documentation provided.
    pub item: i32,
}

/// Returns the detailed information about a Collectible Presentation Node and any Collectibles that are direct descendants.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DestinyCollectibleNodeDetailResponse {

    /// COMPONENT TYPE: Collectibles
    pub collectibles: i32,
    /// Item components, keyed by the item hash of the items pointed at collectibles found under the requested Presentation Node.
/// NOTE: I had a lot of hemming and hawing about whether these should be keyed by collectible hash or item hash... but ultimately having it be keyed by item hash meant that UI that already uses DestinyItemComponentSet data wouldn't have to have a special override to do the collectible -> item lookup once you delve into an item's details, and it also meant that you didn't have to remember that the Hash being used as the key for plugSets was different from the Hash being used for the other Dictionaries. As a result, using the Item Hash felt like the least crappy solution.
/// We may all come to regret this decision. We will see.
/// COMPONENT TYPE: [See inside the DestinyItemComponentSet contract for component types.]
    pub collectible_item_components: i32,
}
