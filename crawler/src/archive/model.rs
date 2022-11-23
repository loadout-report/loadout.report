use serde::de::Visitor;
use serde::{de, Deserialize, Deserializer, Serialize};
use std::collections::HashMap;
use std::fmt;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Values1 {
    // pub average_score_per_kill: Option<f64>,
    // pub average_score_per_life: Option<f64>,
    // pub standing: Option<i64>,
    // pub team: Option<i64>,
    pub activity_duration_seconds: i64,
    // pub assists: i64,
    pub completed: f64,
    pub completion_reason: f64,
    // pub deaths: i64,
    // pub efficiency: f64,
    // pub fireteam_id: f64,
    // pub kills: i64,
    // pub kills_deaths_assists: f64,
    // pub kills_deaths_ratio: f64,
    // pub opponents_defeated: i64,
    // pub player_count: i64,
    pub score: i64,
    // pub start_seconds: i64,
    // pub team_score: i64,
    pub time_played_seconds: i64,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DestinyUserInfo {
    // pub display_name: Option<String>,
    // pub icon_path: Option<String>,
    pub membership_id: Id,
    pub membership_type: i64,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BungieNetUserInfo {
    // pub display_name: String,
    // pub icon_path: String,
    pub membership_id: Id,
    pub membership_type: i64,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Player {
    pub bungie_net_user_info: Option<BungieNetUserInfo>,
    // pub character_class: Option<String>,
    pub character_level: i64,
    pub class_hash: i64,
    pub destiny_user_info: DestinyUserInfo,
    pub emblem_hash: i64,
    pub gender_hash: i64,
    pub light_level: i64,
    pub race_hash: i64,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct Values {
    pub medal_multi_2x: Option<i64>,
    pub medal_streak_5x: Option<i64>,
    pub medal_streak_10x: Option<i64>,
    pub medal_streak_team: Option<i64>,
    pub medal_streak_shutdown: Option<i64>,
    pub medal_payback: Option<i64>,
    pub medal_avenger: Option<i64>,
    pub medal_super_shutdown: Option<i64>,
    pub medal_streak_combined: Option<i64>,
    pub precision_kills: i64,
    pub weapon_kills_ability: i64,
    pub weapon_kills_grenade: i64,
    pub weapon_kills_melee: i64,
    pub weapon_kills_super: i64,
}

#[derive(Debug)]
pub struct Id(pub u64);

impl<'de> Deserialize<'de> for Id {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct IdVisitor;

        impl<'de> Visitor<'de> for IdVisitor {
            type Value = Id;

            fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
                f.write_str("ID as a number or string")
            }

            fn visit_i32<E>(self, id: i32) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(Id(id as u64))
            }

            fn visit_u64<E>(self, id: u64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(Id(id))
            }

            fn visit_str<E>(self, id: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                id.parse().map(Id).map_err(de::Error::custom)
            }
        }

        deserializer.deserialize_any(IdVisitor)
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct Extended {
    // Values
    pub values: HashMap<kstring::KString, f64>,
    pub weapons: Option<Vec<Weapon>>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Entry {
    pub character_id: Id,
    // pub extended: Extended,
    pub player: Player,
    // pub score: i64,
    // pub standing: i64,
    // Values1
    // pub values: HashMap<kstring::KString, f64>,
    pub values: Values1,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ActivityDetails {
    // pub director_activity_hash: i64,
    pub instance_id: Id,
    // pub is_private: bool,
    // pub mode: i64,
    pub modes: Vec<i64>,
    pub reference_id: i64,
    // pub membership_type: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Team {
    pub team_id: i64,
    pub standing: i64,
    pub score: i64,
    // pub team_name: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ArchivedReport {
    // #[serde(rename = "_id")]
    // pub id: Id,
    pub activity_details: ActivityDetails,
    // pub archived: chrono::DateTime<chrono::Utc>,
    pub entries: Vec<Entry>,
    pub period: chrono::DateTime<chrono::Utc>,
    // pub starting_phase_index: i64,
    // pub teams: Vec<Team>,
}

impl ArchivedReport {
    pub fn is_gm(&self) -> bool {
        crate::constants::GRANDMASTERS
            .binary_search(&self.activity_details.reference_id)
            .is_ok()
    }

    pub fn strike_slug(&self) -> i64 {
        self.activity_details.reference_id
    }

    pub fn season(&self) -> i32 {
        let id = self.activity_details.instance_id.0;

        if (6_110_577_052..6_369_174_187).contains(&id) {
            // Worthy
            return 10;
        } else if (6_652_421_602..7_132_690_261).contains(&id) {
            // Arrivals
            return 11;
        } else if (7_618_446_979..7_927_738_348).contains(&id) {
            // Hunt
            return 12;
        } else if (8_157_068_252..8_418_736_467).contains(&id) {
            // Chosen
            return 13;
        } else if (8_700_774_175..9_028_999_691).contains(&id) {
            // Splicer
            return 14;
        } else if (9_370_573_998..10_179_519_298).contains(&id) {
            // Lost
            return 15;
        } else if (10_549_012_758..10_800_796_510).contains(&id) {
            // Risen
            return 16;
        } else if (11_087_977_829..11_369_612_788).contains(&id) {
            // Haunted
            return 17;
        } else if id >= 11_734_575_369 {
            // Plunder
            return 18;
        }

        0
    }

    pub fn is_completed(&self) -> bool {
        self.entries
            .iter()
            .any(|e: &Entry| e.values.completion_reason < 0.5)
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct WeaponStats {
    pub unique_weapon_kills: i64,
    pub unique_weapon_precision_kills: i64,
    pub unique_weapon_kills_precision_kills: f64,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct Weapon {
    pub reference_id: i64,
    pub values: WeaponStats,
}
