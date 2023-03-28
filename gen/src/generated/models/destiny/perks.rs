use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use serde_repr::{Serialize_repr, Deserialize_repr};
use serde_with::{serde_as, DisplayFromStr};

/// The list of perks to display in an item tooltip - and whether or not they have been activated.
/// Perks apply a variety of effects to a character, and are generally either intrinsic to the item or provided in activated talent nodes or sockets.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DestinyPerkReference {

    /// The icon for the perk.
    pub icon_path: String,
    /// Whether this perk is currently active. (We may return perks that you have not actually activated yet: these represent perks that you should show in the item's tooltip, but that the user has not yet activated.)
    pub is_active: bool,
    /// The hash identifier for the perk, which can be used to look up DestinySandboxPerkDefinition if it exists. Be warned, perks frequently do not have user-viewable information. You should examine whether you actually found a name/description in the perk's definition before you show it to the user.
    pub perk_hash: crate::id::Id<crate::generated::models::destiny::definitions::DestinySandboxPerkDefinition>,
    /// Some perks provide benefits, but aren't visible in the UI. This value will let you know if this is perk should be shown in your UI.
    pub visible: bool,
}
