

use serde::{Serialize, Deserialize};


/// These definitions represent vendors' locations and relevant display information at different times in the game.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DestinyVendorLocationDefinition {

    /// The hash identifier for a Destination at which this vendor may be located. Each destination where a Vendor may exist will only ever have a single entry.
    pub destination_hash: u32,
    /// The relative path to the background image representing this Vendor at this location, for use in a banner.
    pub background_image_path: String,
}
