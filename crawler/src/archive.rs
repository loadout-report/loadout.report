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
use std::io::{BufRead, BufReader, Lines, Read};
use std::ops::Div;
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
    let (extended, values): (HashSet<kstring::KString>, HashSet<kstring::KString>) = read_dir(dir)
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
        .reduce(
            |a: (HashSet<KString>, HashSet<KString>), b: (HashSet<KString>, HashSet<KString>)| {
                (
                    a.0.union(&b.0).cloned().collect::<HashSet<_>>(),
                    a.1.union(&b.1).cloned().collect::<HashSet<_>>(),
                )
            },
        )
        .unwrap();

    let elapsed = time.elapsed();
    let time_per_entry = elapsed / (files * CHUNK_SIZE as usize) as u32;
    println!(
        "elapsed: {:?}, time per entry: {:?}",
        elapsed, time_per_entry,
    );
    println!(
        "processed file, got {} values and {} extended values",
        values.len(),
        extended.len(),
    );
    println!("values: {:?}", values);
    println!("extended: {:?}", extended);
}

#[inline]
fn parse_json(parallel_line_reader: impl ParallelIterator<Item=String>) -> impl ParallelIterator<Item=ArchivedReport> {
    parallel_line_reader.filter_map(|l: String| unsafe {
        // simd_json::from_str::<ArchivedReport>(l.clone().as_mut_str())
        serde_json::from_str(&l)
            .inspect_err(|err| println!("{}, {}", err, l))
            .ok()
    })
}

fn extract_gms(
    json_reader: impl ParallelIterator<Item = ArchivedReport>,
) -> HashMap<String, u32> {
    json_reader
        .filter(|pcgr| pcgr.is_gm())
        .fold(
            || HashMap::new(),
            |a, b| {

            }
        )
        .reduce(
            || HashMap::new(),
            |a, b| {

            })
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
    pub completed: i64,
    pub completion_reason: i64,
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
    #[serde(rename = "_id")]
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
        self.activity_details.modes.iter().any(|mode| GRANDMASTERS.contains(mode))
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

static GRANDMASTERS: Vec<i64> = vec![2136458560, 3381711459, 1203950592, 2112435491, 3293630132, 676886831, 2416314393, 3100302962, 1753547901, 1446478334, 3109193568, 207226563, 3812135451, 4197461112, 1495545956, 3449817631, 3029388704, 554830595, 2599001919, 281497220, 3233498454, 707920309, 2103025315, 3418624832, 4196944364, 1473557543, 1964120205, 968885838, 2766844306, 967120713, 3014390952, 265186825, 89113250, 557845334, 3871967157, 1561733170, 283725097, 3849697860, 2168858559, 2533203708, 3883876601, 1358381372, 3726640183, 2660931443, 2023667984, 3200108048, 2694576755, 68611398, 54961125, 135872558, 3879949581, 380956401, 3597372938, 41222998, 1302909043, 3919254032, 245243710, 3354105309, 766116576, 3455414851];
