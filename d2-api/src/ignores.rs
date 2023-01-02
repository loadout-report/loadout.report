use enumflags2::{bitflags, BitFlags};
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct IgnoreResponse {
    pub is_ignored: bool,
    #[serde(deserialize_with = "crate::util::serde::zero_as_none")]
    pub ignore_flags: Option<BitFlags<IgnoreStatus>>,
}

#[bitflags]
#[repr(u16)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum IgnoreStatus {
    // NotIgnored = 0, // can we translate this to Option?
    IgnoredUser = 1,
    IgnoredGroup = 2,
    IgnoredByGroup = 4,
    IgnoredPost = 8,
    IgnoredTag = 16,
    IgnoredGlobal = 32,
}
