use serde::{Serialize, Deserialize};
use std::collections::HashMap;

#[derive(Hash, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct Membership (pub i32, pub i64);

impl Membership {
    pub fn new(t: i32, id: i64) -> Self {
        Membership(t, id)
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ProfileStruct {
    pub profile: Option<ComponentResponse<Profile>>,
    pub profile_plug_sets: Option<ComponentResponse<ProfilePlugSets>>,
    pub character_equipment: Option<ComponentResponse<HashMap<String, CharacterEquipment>>>,
    pub character_plug_sets: Option<ComponentResponse<HashMap<String, CharacterPlugSets>>>,
    pub item_components: Option<ItemComponents>,
    pub characters: Option<ComponentResponse<HashMap<String, Character>>>
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct CharacterEquipment {
    pub items: Vec<Item>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Item {
    pub item_hash: i64,
    pub item_instance_id: String,
    pub quantity: i64,
    pub bind_status: i64,
    pub location: i64,
    pub bucket_hash: i64,
    pub transfer_status: i64,
    pub lockable: bool,
    pub state: i64,
    pub dismantle_permission: i64,
    pub is_wrapper: bool,
    pub tooltip_notification_indexes: Vec<i64>,
    pub version_number: Option<i64>,
    pub override_style_item_hash: Option<i64>,
    pub metric_hash: Option<i64>,
    pub metric_objective: Option<Objective>,
}

#[derive(Serialize, Deserialize, Copy, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Objective {
    pub objective_hash: i64,
    pub progress: i64,
    pub completion_value: i64,
    pub complete: bool,
    pub visible: bool,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct CharacterPlugSets {
    pub plugs: HashMap<String, Vec<DatumPlug>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct DatumPlug {
    pub plug_item_hash: i64,
    pub can_insert: bool,
    pub enabled: bool,
    pub insert_fail_indexes: Option<Vec<i64>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct ItemComponents {
    pub instances: ComponentResponse<HashMap<String, Instance>>,
    pub stats: ComponentResponse<HashMap<String, Stats>>,
    pub sockets: Option<ComponentResponse<HashMap<String, Sockets>>>,
    pub perks: ComponentResponse<HashMap<String, Perks>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Instance {
    pub damage_type: i64,
    pub item_level: i64,
    pub quality: i64,
    pub is_equipped: bool,
    pub can_equip: bool,
    pub equip_required_level: i64,
    pub unlock_hashes_required_to_equip: Vec<i64>,
    pub cannot_equip_reason: i64,
    pub energy: Option<Energy>,
    pub damage_type_hash: Option<i64>,
    pub primary_stat: Option<Stat>,
    pub breaker_type: Option<i64>,
    pub breaker_type_hash: Option<i64>,
}

#[derive(Serialize, Deserialize, Copy, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Energy {
    pub energy_type_hash: i64,
    pub energy_type: i64,
    pub energy_capacity: i64,
    pub energy_used: i64,
    pub energy_unused: i64,
}

#[derive(Serialize, Deserialize, Copy, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Stat {
    pub stat_hash: i64,
    pub value: i64,
}


#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Perks {
    pub perks: Vec<Perk>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Perk {
    pub perk_hash: i64,
    pub icon_path: String,
    pub is_active: bool,
    pub visible: bool,
}


#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Sockets {
    pub sockets: Vec<Socket>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Socket {
    pub plug_hash: Option<i64>,
    pub is_enabled: bool,
    pub is_visible: bool,
    pub enable_fail_indexes: Option<Vec<i64>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Stats {
    pub stats: HashMap<String, Stat>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Profile {
    pub user_info: UserInfo,
    pub date_last_played: String,
    pub versions_owned: i64,
    pub character_ids: Vec<String>,
    pub season_hashes: Vec<i64>,
    pub event_card_hashes_owned: Vec<i64>,
    pub current_season_hash: i64,
    pub current_season_reward_power_cap: i64,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct UserInfo {
    pub cross_save_override: i64,
    pub applicable_membership_types: Vec<i64>,
    pub is_public: bool,
    pub membership_type: i64,
    pub membership_id: String,
    pub display_name: String,
    pub bungie_global_display_name: String,
    pub bungie_global_display_name_code: i64,
}


#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct ProfilePlugSets {
    pub plugs: HashMap<String, Vec<DataPlug>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct DataPlug {
    pub plug_item_hash: i64,
    pub can_insert: bool,
    pub enabled: bool,
    pub plug_objectives: Option<Vec<Objective>>,
    pub insert_fail_indexes: Option<Vec<i64>>,
    pub enable_fail_indexes: Option<Vec<i64>>,
}
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct ComponentResponse<T: Clone> {
    data: Option<T>,
    privacy: i64,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Character {
    membership_id: String,
    membership_type: i64,
    character_id: String,
    date_last_played: String,
    minutes_played_this_session: String,
    minutes_played_total: String,
    light: i64,
    stats: HashMap<String, i64>,
    race_hash: i64,
    gender_hash: i64,
    class_hash: i64,
    race_type: i64,
    class_type: i64,
    gender_type: i64,
    emblem_path: String,
    emblem_background_path: String,
    emblem_hash: i64,
    emblem_color: EmblemColor,
    level_progression: HashMap<String, i64>,
    base_character_level: i64,
    percent_to_next_level: f64,
    title_record_hash: i64,
}

#[derive(Serialize, Deserialize, Copy, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct EmblemColor {
    red: i64,
    green: i64,
    blue: i64,
    alpha: i64,
}


#[derive(Clone, PartialEq, Eq, Debug, Serialize, Deserialize)]
pub struct ExactSearchRequest {
    #[serde(rename = "displayName")]
    pub name: String,
    #[serde(rename = "displayNameCode")]
    pub code: i16,
}
