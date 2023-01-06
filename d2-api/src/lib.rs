#[allow(deprecated)]
use serde::{Deserialize, Serialize};

pub mod app;
pub mod config;
pub mod content;
pub mod destiny;
#[cfg(feature = "endpoints")]
pub mod endpoints;
pub mod forums;
pub mod groupsv2;
pub mod ignores;
pub mod links;
pub mod queries;
pub mod user;
mod util;
mod wrapper;

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum BungieCredentialType {
    None = 0,
    Xuid = 1,
    Psnid = 2,
    Wlid = 3,
    Fake = 4,
    Facebook = 5,
    Google = 8,
    Windows = 9,
    DemonId = 10,
    SteamId = 12,
    BattleNetId = 14,
    StadiaId = 16,
    TwitchId = 18,
    EgsId = 20,
}
