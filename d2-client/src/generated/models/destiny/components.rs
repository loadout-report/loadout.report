use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use serde_repr::{Serialize_repr, Deserialize_repr};
use serde_with::{serde_as, DisplayFromStr};
pub mod collectibles;
pub mod craftables;
pub mod inventory;
pub mod items;
pub mod kiosks;
pub mod loadouts;
pub mod metrics;
pub mod plug_sets;
pub mod presentation;
pub mod profiles;
pub mod records;
pub mod social;
pub mod string_variables;
pub mod vendors;