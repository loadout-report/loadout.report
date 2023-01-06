use crate::pgcr::archive::{ArchivedReport, DestinyUserInfo, Entry, Id};
use rayon::prelude::*;
use serde::Serialize;
use std::collections::{HashMap, HashSet};
use std::hash::Hash;
use std::sync::Arc;

#[derive(Eq, PartialEq, Copy, Clone, Hash, Serialize)]
pub struct Membership(u8, Id);

pub fn extract_members(
    json_reader: impl ParallelIterator<Item = ArchivedReport>,
) -> HashSet<Membership> {
    json_reader
        .map(|pgcr: ArchivedReport| {
            pgcr.entries
                .iter()
                .map(|e: &Entry| &e.player.destiny_user_info)
                .map(|e: &DestinyUserInfo| Membership(e.membership_type, e.membership_id))
                .collect()
        })
        .reduce(HashSet::new, merge)
}

pub fn extract_gms(
    json_reader: impl ParallelIterator<Item = ArchivedReport>,
) -> HashMap<String, u32> {
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
                    if entry.values.get("completed").is_some_and(|c| *c > 0.5) {
                        *a.entry(format!("{}/c", k)).or_default() += &1;
                    }
                }
                a
            },
        )
        .reduce(HashMap::new, merge_sum)
}

pub fn merge_sum(mut a: HashMap<String, u32>, b: HashMap<String, u32>) -> HashMap<String, u32> {
    for (k, v) in b {
        *a.entry(k).or_default() += &v;
    }
    a
}

pub fn merge<T: Eq + Hash + Copy>(mut a: HashSet<T>, b: HashSet<T>) -> HashSet<T> {
    for v in b {
        a.insert(v);
    }
    // a.extend(b);
    return a;
    // a.union(&b).map(|x| x.to_owned()).collect()
}
