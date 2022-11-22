use indicatif::{
    HumanDuration, MultiProgress, ParallelProgressIterator, ProgressBar, ProgressBarIter,
    ProgressIterator, ProgressState, ProgressStyle,
};
use kstring::KString;
use rayon::iter::{FilterMap, IntoParallelRefIterator, IterBridge, ParallelIterator};
use rayon::prelude::*;
use serde::de::Visitor;
use serde::{de, Deserialize, Deserializer, Serialize};
use std::collections::{HashMap, HashSet};
use std::fmt::Write;
use std::fs::{read_dir, DirEntry, File, ReadDir};
use std::hash::Hash;
use std::io::{BufRead, BufReader, Lines, Read};
use std::iter::Sum;
use std::ops::{Add, Div};
use std::time::{Duration, Instant};
use std::{fmt, io};
use zstd::Decoder;

const CHUNK_SIZE: u64 = 10_000_000;

pub fn crawl_files(dir: &str) {
    let files = read_dir(dir).expect("unable to open directory").count();
    let multi = MultiProgress::new();
    let bar = multi.add(ProgressBar::new(files as u64));
    let sty = ProgressStyle::with_template(
        "[{elapsed_precise}] {bar:40.cyan/blue} {percent:>3}% {smoothed_per_sec} {smoothed_eta} {pos:>7}/{len:7} {msg}",
    )
    .unwrap()
        .with_key(
        "smoothed_eta",
        |s: &ProgressState, w: &mut dyn Write| match (s.pos(), s.len()) {
            (pos, Some(len)) => write!(
                w,
                "{:#}",
                HumanDuration(Duration::from_millis(
                    (s.elapsed().as_millis() * (len as u128 - pos as u128) / (pos as u128))
                        as u64
                ))
            )
                .unwrap(),
            _ => write!(w, "-").unwrap(),
        },
    )
    .with_key(
        "smoothed_per_sec",
        |s: &ProgressState, w: &mut dyn Write| match (s.pos(), s.elapsed().as_millis()) {
            (pos, elapsed_ms) if elapsed_ms > 0 => {
                write!(w, "{:.2}/s", pos as f64 * 1000_f64 / elapsed_ms as f64).unwrap()
            }
            _ => write!(w, "-").unwrap(),
        },
    )
    .progress_chars("##-");
    bar.set_style(sty.clone());

    let time = Instant::now();
    let map: HashMap<String, u32> = read_dir(dir)
        .expect("unable to open directory")
        .flatten()
        .progress_with(bar)
        .map(|dir_entry| {
            let pb = multi.add(ProgressBar::new(CHUNK_SIZE));
            let file_name = dir_entry.file_name();
            pb.set_message(file_name.to_str().unwrap().to_string()); // end me
            pb.set_style(sty.clone());
            let file = File::open(dir_entry.path()).expect("couldn't open archive file");
            let decoder = zstd::stream::Decoder::new(file).expect("couldn't open decoder");
            let reader = BufReader::new(decoder);
            let parallel_line_reader = reader
                .lines()
                .filter_map(|l| l.ok())
                .par_bridge()
                .progress_with(pb);
            let json_reader = parse_json(parallel_line_reader);
            extract_gms(json_reader)
        })
        .reduce(merge_sum)
        .unwrap();

    let elapsed = time.elapsed();
    let time_per_entry = elapsed / (files * CHUNK_SIZE as usize) as u32;
    println!(
        "elapsed: {:?}, time per entry: {:?}",
        elapsed, time_per_entry,
    );

    let output_file = File::create("out.json.zst").unwrap();
    let writer = zstd::Encoder::new(output_file, 0).unwrap();
    serde_json::to_writer(writer, &map).unwrap();
}

#[inline]
fn parse_json(
    parallel_line_reader: impl ParallelIterator<Item = String>,
) -> impl ParallelIterator<Item = ArchivedReport> {
    parallel_line_reader.filter_map(|l: String| unsafe {
        // simd_json::from_str::<ArchivedReport>(l.clone().as_mut_str())
        serde_json::from_str(&l)
            .inspect_err(|err| println!("{}, {}", err, l))
            .ok()
    })
}

fn merge_sum(mut a: HashMap<String, u32>, b: HashMap<String, u32>) -> HashMap<String, u32> {
    for (k, v) in b {
        *a.entry(k).or_default() += &v;
    }
    a
}

fn extract_gms(json_reader: impl ParallelIterator<Item = ArchivedReport>) -> HashMap<String, u32> {
    json_reader
        .filter(|pcgr| pcgr.is_gm())
        .fold(
            HashMap::new,
            |mut a: HashMap<String, u32>, b: ArchivedReport| {
                let k = format!("strike/{}/season/{}", b.strike_slug(), b.season());
                *a.entry(format!("{}/a", k)).or_default() += &1;
                if b.is_completed() {
                    *a.entry(format!("{}/c", k)).or_default() += &1;
                }
                for entry in &b.entries {
                    let k = format!(
                        "player/{}/{}/{}",
                        entry.player.destiny_user_info.membership_type,
                        entry.player.destiny_user_info.membership_id.0,
                        k
                    );
                    *a.entry(format!("{}/a", k)).or_default() += &1;
                    if entry.values.completed > 0.5 {
                        *a.entry(format!("{}/c", k)).or_default() += &1;
                    }
                }
                a
            },
        )
        .reduce(HashMap::new, merge_sum)
}

// #[inline]
// fn extract_values_entries(
//     json_reader: impl ParallelIterator<Item = ArchivedReport>,
// ) -> (HashSet<KString>, HashSet<KString>) {
//     let values_entries: (HashSet<kstring::KString>, HashSet<kstring::KString>) = json_reader
//         .flat_map(|pgcr: ArchivedReport| pgcr.entries.into_iter().par_bridge())
//         .map(|entry: Entry| (entry.extended.values.into_keys(), entry.values.into_keys()))
//         .fold(
//             || (HashSet::new(), HashSet::new()),
//             |mut a, b| {
//                 a.0.extend(b.0);
//                 a.1.extend(b.1);
//                 a
//             },
//         )
//         .reduce(
//             || (HashSet::new(), HashSet::new()),
//             |a, b| {
//                 (
//                     a.0.union(&b.0).cloned().collect::<HashSet<_>>(),
//                     a.1.union(&b.1).cloned().collect::<HashSet<_>>(),
//                 )
//             },
//         );
//     values_entries
// }

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Values1 {
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
struct DestinyUserInfo {
    // pub display_name: Option<String>,
    // pub icon_path: Option<String>,
    pub membership_id: Id,
    pub membership_type: i64,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct BungieNetUserInfo {
    // pub display_name: String,
    // pub icon_path: String,
    pub membership_id: Id,
    pub membership_type: i64,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Player {
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

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
struct Values {
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
struct Id(u64);

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
struct Extended {
    // Values
    pub values: HashMap<kstring::KString, f64>,
    pub weapons: Option<Vec<Weapon>>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Entry {
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
struct ActivityDetails {
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
struct Team {
    pub team_id: i64,
    pub standing: i64,
    pub score: i64,
    // pub team_name: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct ArchivedReport {
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
    fn is_gm(&self) -> bool {
        GRANDMASTERS
            .binary_search(&self.activity_details.reference_id)
            .is_ok()
    }

    fn strike_slug(&self) -> i64 {
        self.activity_details.reference_id
    }

    fn season(&self) -> i32 {
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

    fn is_completed(&self) -> bool {
        self.entries
            .iter()
            .any(|e: &Entry| e.values.completion_reason < 0.5)
    }
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

static GRANDMASTERS: &[i64] = &[
    41222998, 54961125, 68611398, 89113250, 135872558, 207226563, 245243710, 265186825, 281497220,
    283725097, 380956401, 554830595, 557845334, 676886831, 707920309, 766116576, 967120713,
    968885838, 1203950592, 1302909043, 1358381372, 1446478334, 1473557543, 1495545956, 1561733170,
    1753547901, 1964120205, 2023667984, 2103025315, 2112435491, 2136458560, 2168858559, 2416314393,
    2533203708, 2599001919, 2660931443, 2694576755, 2766844306, 3014390952, 3029388704, 3100302962,
    3109193568, 3200108048, 3233498454, 3293630132, 3354105309, 3381711459, 3418624832, 3449817631,
    3455414851, 3597372938, 3726640183, 3812135451, 3849697860, 3871967157, 3879949581, 3883876601,
    3919254032, 4196944364, 4197461112,
];
