

use serde::{Serialize, Deserialize};


/// Properties of a DestinyInventoryItemDefinition that store all of the information we were able to discern about how the item spawns, and where you can find the item.
/// Items will have many of these sources, one per level at which it spawns, to try and give more granular data about where items spawn for specific level ranges.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DestinyItemSourceDefinition {

    /// The maximum quality at which the item spawns for this level.
    pub max_quality: i32,
    /// The stats computed for this level/quality range.
    pub computed_stats: i32,
    /// The minimum Character Level required for equipping the item when the item spawns at the item level defined on this DestinyItemSourceDefinition, as far as we saw in our processing.
    pub min_level_required: i32,
    /// The minimum Quality at which the item spawns for this level. Examine DestinyInventoryItemDefinition for more information about what Quality means. Just don't ask Phaedrus about it, he'll never stop talking and you'll have to write a book about it.
    pub min_quality: i32,
    /// The level at which the item spawns. Essentially the Primary Key for this source data: there will be multiple of these source entries per item that has source data, grouped by the level at which the item spawns.
    pub level: i32,
    /// The DestinyRewardSourceDefinitions found that can spawn the item at this level.
    pub source_hashes: i32,
    /// The maximum Character Level required for equipping the item when the item spawns at the item level defined on this DestinyItemSourceDefinition, as far as we saw in our processing.
    pub max_level_required: i32,
}
