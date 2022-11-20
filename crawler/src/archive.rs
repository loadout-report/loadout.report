use crate::raw;
use crate::raw::PostGameCarnageReport;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

pub fn stream(path: &str) {
    let mut file = File::open(path).expect("couldn't open archive file");
    let decoder = zstd::stream::Decoder::new(file).expect("couldn't open decoder");
    let reader = BufReader::new(decoder);
    for pgcr in reader
        .lines()
        .filter_map(|l| l.ok())
        .filter_map(|l| {
            serde_json::from_str::<ArchivedReport>(l.as_str())
                .inspect_err(|err| println!("{}, {}", err, l))
                .err()
        })
        .take(1)
    {
        println!("{:?}", pgcr);
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
struct Values1 {
    pub activity_duration_seconds: i64,
    pub assists: i64,
    pub completed: i64,
    pub completion_reason: i64,
    pub deaths: i64,
    pub efficiency: f64,
    pub fireteam_id: f64,
    pub kills: i64,
    pub kills_deaths_assists: f64,
    pub kills_deaths_ratio: f64,
    pub opponents_defeated: i64,
    pub player_count: i64,
    pub score: i64,
    pub start_seconds: i64,
    pub team_score: i64,
    pub time_played_seconds: i64,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
struct DestinyUserInfo {
    pub display_name: String,
    pub icon_path: String,
    pub membership_id: String,
    pub membership_type: i64,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
struct BungieNetUserInfo {
    pub display_name: String,
    pub icon_path: String,
    pub membership_id: i64,
    pub membership_type: i64,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
struct Player {
    pub bungie_net_user_info: Option<BungieNetUserInfo>,
    pub character_class: Option<String>,
    pub character_level: i64,
    pub class_hash: i64,
    pub destiny_user_info: DestinyUserInfo,
    pub emblem_hash: i64,
    pub gender_hash: i64,
    pub light_level: i64,
    pub race_hash: i64,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
struct Values {
    pub precision_kills: i64,
    pub weapon_kills_ability: i64,
    pub weapon_kills_grenade: i64,
    pub weapon_kills_melee: i64,
    pub weapon_kills_super: i64,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
struct Extended {
    pub values: Values,
    pub weapons: Option<Vec<Weapon>>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
struct Struct {
    pub character_id: String,
    pub extended: Extended,
    pub player: Player,
    pub score: i64,
    pub standing: i64,
    pub values: Values1,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
struct ActivityDetails {
    pub director_activity_hash: i64,
    pub instance_id: i64,
    pub is_private: bool,
    pub mode: i64,
    pub modes: Vec<i64>,
    pub reference_id: i64,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
struct ArchivedReport {
    #[serde(rename = "_id")]
    pub id: i64,
    pub activity_details: ActivityDetails,
    pub archived: String,
    pub entries: Vec<Struct>,
    pub period: String,
    pub starting_phase_index: i64,
    pub teams: Vec<()>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
struct WeaponStats {
    pub unique_weapon_kills: i64,
    pub unique_weapon_precision_kills: i64,
    pub unique_weapon_kills_precision_kills: f64,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
struct Weapon {
    pub reference_id: i64,
    pub values: WeaponStats,
}
