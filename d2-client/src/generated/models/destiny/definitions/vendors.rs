use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use serde_repr::{Serialize_repr, Deserialize_repr};
use serde_with::{serde_as, DisplayFromStr};

/// These definitions represent vendors' locations and relevant display information at different times in the game.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DestinyVendorLocationDefinition {

    /// The relative path to the background image representing this Vendor at this location, for use in a banner.
    pub background_image_path: String,
    /// The hash identifier for a Destination at which this vendor may be located. Each destination where a Vendor may exist will only ever have a single entry.
    pub destination_hash: crate::id::Id<u32, crate::generated::models::destiny::definitions::DestinyDestinationDefinition>,
}
