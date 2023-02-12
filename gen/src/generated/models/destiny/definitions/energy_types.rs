

use serde::{Serialize, Deserialize};


/// Represents types of Energy that can be used for costs and payments related to Armor 2.0 mods.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DestinyEnergyTypeDefinition {

    /// A variant of the icon that is transparent and colorless.
    pub transparent_icon_path: String,
    /// The unique identifier for this entity. Guaranteed to be unique for the type of entity, but not globally.
/// When entities refer to each other in Destiny content, it is this hash that they are referring to.
    pub hash: u32,
    /// If this is true, then there is an entity with this identifier/type combination, but BNet is not yet allowed to show it. Sorry!
    pub redacted: bool,
    /// We have an enumeration for Energy types for quick reference. This is the current definition's Energy type enum value.
    pub enum_value: i32,
    /// If this Energy Type can be used for determining the Type of Energy that an item can consume, this is the hash for the DestinyInvestmentStatDefinition that represents the stat which holds the Capacity for that energy type. (Note that this is optional because "Any" is a valid cost, but not valid for Capacity - an Armor must have a specific Energy Type for determining the energy type that the Armor is restricted to use)
    pub capacity_stat_hash: Option<u32>,
    /// The index of the entity as it was found in the investment tables.
    pub index: i32,
    /// The description of the energy type, icon etc...
    pub display_properties: i32,
    /// If TRUE, the game shows this Energy type's icon. Otherwise, it doesn't. Whether you show it or not is up to you.
    pub show_icon: bool,
    /// If this Energy Type can be used as a cost to pay for socketing Armor 2.0 items, this is the hash for the DestinyInvestmentStatDefinition that stores the plug's raw cost.
    pub cost_stat_hash: u32,
}
