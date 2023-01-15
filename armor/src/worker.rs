use std::collections::{HashMap, HashSet};
use std::convert::TryInto;
use std::time::{Duration, Instant};

use gloo_worker::{HandlerId, Worker, WorkerScope};
use itertools::Itertools;

use serde::{Deserialize, Serialize};

use crate::armory::Armory;
use data::api::manifest::model::Hash;

use crate::model::stats::{Stats, StatsMod, Waste};
use crate::model::{
    ArmorPerkOrSlot, ArmorSet, ArmorSlot, CharacterClass, DestinyEnergyType, ExoticChoiceModel,
    FixableSelection, Item, ModGroup, Msg, SimpleArmorStat, SimpleModifierValue, StatMod,
    StatModifier, StrippedInventoryArmor, WorkerConfig,
};

pub struct ArmorWorker {}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Input {
    pub config: WorkerConfig,
    pub armory: Armory,
    pub n: usize,
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ExoticResult {
    pub icon: String,
    pub watermark: String,
    pub name: String,
    pub hash: Hash,
}

/// This is the ItemResult returned by the rust application to the javascript
/// interface for interoperability
#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SanitisedItemResult {
    pub exotic: Option<ExoticResult>,
    pub mod_count: usize,
    pub mod_cost: u8,
    pub mods: Vec<StatModifier>,
    pub stats: Stats,
    pub stats_no_mods: Stats,
    pub tiers: u8,
    pub waste: u8,
    pub items: Vec<Item>,
    pub class_item: ClassItem,
}

/// This is the ItemResult returned by the worker to the calling rust framework
/// As opposed to the Stripped version, this uses a regular vector so we can
/// serialise it
#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ItemResult {
    pub exotic: Option<Hash>,
    pub mod_count: usize,
    pub mod_cost: u8,
    pub mods: Vec<StatModifier>,
    pub stats: Stats,
    pub stats_no_mods: Stats,
    pub tiers: u8,
    pub waste: u8,
    pub items: ArmorSet,
    pub class_item: ClassItem,
}

impl From<(StrippedItemResult, ClassItem)> for ItemResult {
    fn from((item, class_item): (StrippedItemResult, ClassItem)) -> Self {
        Self {
            exotic: item.exotic,
            mod_count: item.mod_count,
            mod_cost: item.mod_cost,
            mods: item.mods.to_vec(),
            stats: item.stats,
            stats_no_mods: item.stats_no_mods,
            tiers: item.tiers,
            waste: item.waste,
            items: item.items,
            class_item,
        }
    }
}

#[derive(Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ClassItem {
    perk: ArmorPerkOrSlot,
    affinity: DestinyEnergyType,
}

/// This ItemResult is used internally in the worker for extra speed. It implements
/// Clone should be a very inexpensive operation on this struct
#[derive(Clone)]
pub struct StrippedItemResult {
    exotic: Option<Hash>,
    mod_count: usize,
    mod_cost: u8,
    mods: heapless::Vec<StatModifier, 5>,
    stats: Stats,
    stats_no_mods: Stats,
    tiers: u8,
    waste: u8,
    items: ArmorSet,
}

#[derive(Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Runtime {
    pub maximum_possible_tiers: [u8; 6],
    pub stat_combo_3x_100: HashSet<u64>,
    pub stat_combo_4x_100: HashSet<u64>,
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct OutputStats {
    permutation_count: usize,
    item_count: usize,
    total_time: Duration,
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Output {
    pub runtime: Runtime,
    pub results: Vec<ItemResult>,
    pub total: Option<usize>,
    pub done: bool,
    pub stats: Option<OutputStats>,
}

impl Worker for ArmorWorker {
    type Message = Msg<()>;
    type Input = Input;
    type Output = Output;

    fn create(_: &WorkerScope<Self>) -> Self {
        Self {}
    }

    fn update(&mut self, _: &WorkerScope<Self>, _: Self::Message) {}

    fn received(&mut self, scope: &WorkerScope<Self>, msg: Self::Input, id: HandlerId) {
        // run job
        run_job(scope, id, msg.armory, msg.config, msg.n);
    }
}

fn run_job(
    scope: &WorkerScope<ArmorWorker>,
    id: HandlerId,
    armory: Armory,
    config: WorkerConfig,
    n: usize,
) {
    let start_time = Instant::now();

    let available_class_item_perk_types: HashSet<_> =
        armory.class_items.iter().map(|i| i.perk).collect();

    let available_class_item_energy_perk_dict =
        available_class_item_perk_types
            .iter()
            .fold(HashMap::new(), |mut map, apos| {
                if !map.contains_key(apos) {
                    map.insert(apos, HashSet::new());
                }
                if !map.contains_key(&ArmorPerkOrSlot::None) {
                    map.insert(&ArmorPerkOrSlot::None, HashSet::new());
                }
                armory
                    .class_items
                    .iter()
                    .filter(|i| i.perk == *apos)
                    .for_each(|i| {
                        map.get_mut(&ArmorPerkOrSlot::None)
                            .unwrap()
                            .insert(i.energy_affinity);
                        map.get_mut(apos).unwrap().insert(i.energy_affinity);
                    });
                map
            });

    // console.debug

    let mut runtime: Runtime = Runtime {
        maximum_possible_tiers: [0, 0, 0, 0, 0, 0],
        stat_combo_3x_100: Default::default(),
        stat_combo_4x_100: Default::default(),
    };

    // todo: evaluate if moving by value or ref is faster here
    let stat_bonus = prepare_constant_stat_bonus(&config.enabled_mods, config.character_class);
    let element_requirement = prepare_constant_element_requirement(&config.armor_affinities);
    let modslot_requirement = prepare_constant_modslot_requirement(&config.armor_perks);
    let available_modslots = prepare_constant_available_modslots(&config.maximum_mod_slots);
    let must_check_element_req = element_requirement[0] < 5;

    let mut results: Vec<ItemResult> = Vec::with_capacity(5000);

    let mut listed_results = 0;
    let mut total_results = 0;
    let mut do_not_output = false;

    for set in armory.clone() {
        let (ok, required_class_item) = check_slots(
            &config,
            &modslot_requirement,
            &available_class_item_energy_perk_dict,
            &set,
        );
        if !ok {
            continue;
        }

        let mut required_class_el = DestinyEnergyType::Any;
        if must_check_element_req {
            let (ok, req_cel) = check_elements(
                &config,
                element_requirement,
                available_class_item_energy_perk_dict
                    .get(&required_class_item.unwrap_or_default())
                    .unwrap_or(&HashSet::new()),
                &set,
            );
            if !ok {
                continue;
            }
            required_class_el = req_cel;
        }

        let (runtime, result) = handle_permutation(
            // &mut runtime,
            &config,
            &set,
            stat_bonus,
            available_modslots,
            do_not_output, // this can exit early if we don't want to output the item
            // do we maybe want to calc output separately?
        );
        match result {
            Ok(result) => {
                total_results += 1;

                let result = ItemResult::from((result, ClassItem {
                    perk: required_class_item.unwrap_or_default(),
                    affinity: required_class_el
                }));

                results.push(result);
                listed_results += 1;

                // don't output any more if we've reached the limit (divided by chunk count)
                do_not_output = do_not_output
                    || (config.limit_parsed_results && listed_results >= 50_000_usize / n)
                    || listed_results >= 1_000_000_usize / n;
                if results.len() == 5000 {
                    scope.respond(
                        id,
                        Output {
                            runtime: runtime.clone(),
                            results,
                            total: None,
                            done: false,
                            stats: None,
                        },
                    );
                    results = Vec::with_capacity(5000);
                }
            }
            Err(err) => {
                if let PermutationError::DoNotOutput = err {
                    total_results += 1
                }
                // ignored errors
                // Unknown,
                // TooManyMods,
                // WastedStats,
            }
        }
    }

    scope.respond(
        id,
        Output {
            runtime,
            results,
            total: None,
            done: true,
            stats: Some(OutputStats {
                permutation_count: total_results,
                item_count: armory.len() - armory.class_items.len(),
                total_time: start_time.elapsed(),
            }),
        },
    );
}

#[derive(Copy, Clone)]
pub enum PermutationError {
    Unknown,
    ConstraintsExceeded,
    TooManyMods,
    WastedStats,
    DoNotOutput,
}

pub(crate) fn handle_permutation(
    // runtime: &mut Runtime,
    config: &WorkerConfig,
    set: &ArmorSet,
    bonus: StatsMod,
    mut slots: [u8; 5],
    do_not_output: bool,
) -> (Runtime, Result<StrippedItemResult, PermutationError>) {
    // todo: verify not checking for masterworks here is okay (we did that near the filter section)
    // todo: we're ignoring the add constant 1 resilience option for now
    let mut base_stats = set.stat_total();

    let mut runtime = Runtime {
        maximum_possible_tiers: [0, 0, 0, 0, 0, 0],
        stat_combo_3x_100: Default::default(),
        stat_combo_4x_100: Default::default(),
    };

    // add 1 resilience if chest isn't exotic and the config flag is given
    base_stats.values[1] += (!set.chest.is_exotic && config.add_constant_1_resilience) as u8;

    let mut stats = base_stats + bonus;

    for (stat, tier) in &config.minimum_stat_tiers {
        if tier.fixed && stats[*stat] > tier.value {
            return (runtime, Err(PermutationError::TooManyMods));
        }
    }

    let mut required_mods = [
        0.max(
            config.minimum_stat_tiers
                .get(&SimpleArmorStat::Mobility).unwrap().value
                - stats[SimpleArmorStat::Mobility] / 10,
        ),
        0.max(
            config.minimum_stat_tiers
                .get(&SimpleArmorStat::Resilience).unwrap().value
                - stats[SimpleArmorStat::Resilience] / 10,
        ),
        0.max(
            config.minimum_stat_tiers
                .get(&SimpleArmorStat::Recovery).unwrap().value
                - stats[SimpleArmorStat::Recovery] / 10,
        ),
        0.max(
            config.minimum_stat_tiers
                .get(&SimpleArmorStat::Discipline).unwrap().value
                - stats[SimpleArmorStat::Discipline] / 10,
        ),
        0.max(
            config.minimum_stat_tiers
                .get(&SimpleArmorStat::Intellect).unwrap().value
                - stats[SimpleArmorStat::Intellect] / 10,
        ),
        0.max(
            config.minimum_stat_tiers
                .get(&SimpleArmorStat::Strength).unwrap().value
                - stats[SimpleArmorStat::Strength] / 10,
        ),
    ];

    let required_mods_total: u8 = required_mods.iter().sum();
    if required_mods_total > 5 {
        return (runtime, Err(PermutationError::TooManyMods));
    }

    // todo: there may be a more efficient way of doing this
    let mut available_modslots_count = slots.iter().filter(|i| 0 < **i).count();

    if required_mods_total > available_modslots_count as u8 {
        return (runtime, Err(PermutationError::TooManyMods));
    }

    // can we order this descending?
    let mut used_mods: heapless::Vec<u8, 5> = heapless::Vec::new();
    // we need mods
    if required_mods_total > 0 {
        // add any necessary mods
        for i in 0..6 {
            // iterate through stats
            if required_mods[i] == 0 {
                continue;
            }

            // add some minor mods
            let stat_diff = stats[i] % 10;
            if stat_diff >= 5 {
                let x = 1 + (i * 2) as u8;
                let res = used_mods
                    .insert(used_mods.binary_search(&x).unwrap_or_else(|pos| pos), x)
                    .map_err(|_| PermutationError::TooManyMods);
                if res.is_err() {
                    return (runtime, Err(PermutationError::TooManyMods));
                }
                required_mods[i] -= 1;
                stats[i] += 5;
            }

            // fill rest with major mods
            for _ in 0..required_mods[i] {
                let x = 2 + (i * 2) as u8;
                let res = used_mods
                    .insert(used_mods.binary_search(&x).unwrap_or_else(|pos| pos), x)
                    .map_err(|_| PermutationError::TooManyMods);
                if res.is_err() {
                    return (runtime, Err(PermutationError::TooManyMods));
                }
                stats[i] += 10;
            }
        }

        // optimise modslots
        // todo: this entire section needs a massive overhaul
        // start with coming up with a better datastructure for work items
        // iterate through our selected mod slots
        let mut i = 0;
        loop {
            // verify this should be 5 or 6
            if i == used_mods.len() || used_mods.len() == 5 {
                break;
            }

            // definitely exists as we are within used_mod length
            // mod to check
            let curr = *used_mods.get(i).unwrap();
            // how much does this mod cost?
            // todo: can we optimise this by calling it earlier?
            let cost = get_stat_mod_cost(num::FromPrimitive::from_u8(curr).unwrap());
            // how many slots do we have that can accommodate this mod cost? if 0:
            // todo: likely more efficient with a breakable for loop since we only ever need the first match
            // todo: major = cost > 2
            // todo: minor_cost = major_cost / 2 (lossy)
            // todo: might save us some lookups to use the above
            if slots.iter().filter(|d| **d >= cost).count() == 0 {
                // let's try replacing the mod with two minor mods - is this mod a major mod?
                if curr % 2 == 0 {
                    // let's just remove the thing and try adding two minor versions
                    // worst case this doesn't work and we return an error
                    used_mods.remove(i);
                    let minor = curr - 1;
                    let res = used_mods
                        .insert(
                            used_mods.binary_search(&minor).unwrap_or_else(|pos| pos),
                            minor,
                        )
                        .map_err(|_| PermutationError::TooManyMods);
                    if res.is_err() {
                        return (runtime, Err(PermutationError::TooManyMods));
                    }
                    let res = used_mods
                        .insert(
                            used_mods.binary_search(&minor).unwrap_or_else(|pos| pos),
                            minor,
                        )
                        .map_err(|_| PermutationError::TooManyMods);
                    if res.is_err() {
                        return (runtime, Err(PermutationError::TooManyMods));
                    }
                    // step back to verify we can also place the two minor mods correctly (cost-wise)
                    i -= 1;
                } else {
                    // mod isn't a major mod, but we'd need to replace it. exit.
                    return (runtime, Err(PermutationError::TooManyMods));
                }
            } else {
                // we can accommodate this mod just fine, let's mark the slot as used.
                slots
                    .iter_mut()
                    .filter(|d| **d >= cost)
                    .take(1)
                    .for_each(|i| *i = 0);
                available_modslots_count -= 1;
            }
            i += &1;
        }
    }

    // if 5+ mods were used return

    // todo: we can likely optimise this a lot
    if config.only_show_results_with_no_wasted_stats {
        if stats.values.iter().any(|i| *i > 100 || *i % 5 != 0) {
            return (runtime, Err(PermutationError::WastedStats));
        }

        let mut stats_wasted: heapless::Vec<(u8, usize, u8), 6> = stats
            .values
            .iter()
            .enumerate()
            .filter(|(i, x)| *x % 10 != 5) // only wasted stats
            .map(|(i, x)| (x % 10, i, *x)) // make into tuple (wasted, statId, total)
            .collect();
        // todo: desc sort
        // stats_wasted.sort_by(|a, b| a.0.cmp(&b.0));
        stats_wasted.sort_by(|a, b| a.0.cmp(&b.0));
        for (waste, stat, _) in &mut stats_wasted {
            if available_modslots_count == 0 {
                // we can only remove as many as we have mod slots
                break;
            }
            for capacity in &mut slots {
                let mod_id = 1 + (*stat * 2);
                if *capacity >= get_stat_mod_cost(num::FromPrimitive::from_usize(mod_id).unwrap()) {
                    *capacity = 0;
                    available_modslots_count -= 1;
                    stats[*stat] += 5;
                    *waste -= 5;
                    let mod_id = mod_id as u8;
                    let res  = used_mods
                        .insert(
                            used_mods.binary_search(&mod_id).unwrap_or_else(|pos| pos),
                            mod_id,
                        )
                        .map_err(|_| PermutationError::TooManyMods);
                    if res.is_err() {
                        return (runtime, Err(PermutationError::TooManyMods));
                    }
                    break;
                }
            }
            if *waste != 0 {
                // if we just went through all of those without fixing the wasted stat it means it's too expensive and we done goofed
                break;
            }
        }

        if stats.waste_sum() > 0 {
            // we have waste.
            return (runtime, Err(PermutationError::WastedStats));
        }
    }

    // get maximum possible stats and write them into the runtime
    let max_bonus = 10 * available_modslots_count as u8;
    let minimum_needed_for_max = 100 - max_bonus;

    // heapless vector of possible (statId, required to make 100)
    let mut possible_100: heapless::Vec<Stat, 6> = Default::default();
    for (index, current_stat) in stats.values.iter().enumerate() {
        let mut current_stat = *current_stat;
        if current_stat >= minimum_needed_for_max {
            // todo: WARNING CURRENT STAT COULD BE OVER 100
            possible_100
                .push(Stat(index as u8, 100 - (current_stat.max(100) as i16)))
                .unwrap();
        }

        if current_stat + max_bonus >= runtime.maximum_possible_tiers[index] {
            let minor = get_stat_mod_cost(num::FromPrimitive::from_usize(1 + (index * 2)).unwrap());
            let major = get_stat_mod_cost(num::FromPrimitive::from_usize(2 + (index * 2)).unwrap());

            for slot in slots {
                if current_stat >= 100 {
                    break;
                }
                if slot >= major {
                    current_stat += 10;
                } else if slot >= minor {
                    current_stat += 5;
                }
            }
            if current_stat > runtime.maximum_possible_tiers[index] {
                runtime.maximum_possible_tiers[index] = current_stat;
            }
        }
    }

    if available_modslots_count > 0 && possible_100.len() >= 3 {
        // a triple 100 or quad 100 build might be doable
        possible_100.sort_by(|a, b| a.1.cmp(&b.1));
        let slots_count = available_modslots_count as u8;

        let combinations: heapless::Vec<_, 20> = possible_100
            .iter()
            // pair each element with its index
            .enumerate()
            .take_while(|(index, _)| *index < possible_100.len() - 2)
            .map(|(index, i)| (index, ArmorCombination::new().with(0, *i), i.mod_cost()))
            .take_while(|(_, _, cost)| *cost <= slots_count)
            .flat_map(|(index, a, cost)| {
                possible_100
                    .iter()
                    .enumerate()
                    .skip(index + 1)
                    .take_while(|(index, _)| *index < possible_100.len() - 1)
                    .map(move |(index, i)| (index, a.with(1, *i), cost + i.mod_cost()))
                    .take_while(|(_, _, cost)| *cost <= slots_count)
                    .flat_map(|(index, combination, cost)| {
                        possible_100
                            .iter()
                            .enumerate()
                            .skip(index + 1)
                            .map(move |(index, i)| {
                                (index, combination.with(2, *i), cost + i.mod_cost())
                            })
                            .take_while(|(_, _, cost)| *cost <= slots_count)
                            .flat_map(|(index, combination, cost)| {
                                let mut res: heapless::Vec<_, 20> = possible_100
                                    .iter()
                                    .enumerate()
                                    .skip(index + 1)
                                    .map(move |(index, i)| {
                                        (index, combination.with(3, *i), cost + i.mod_cost())
                                    })
                                    .take_while(|(_, _, cost)| *cost <= slots_count)
                                    .map(|(_, combination, _)| combination)
                                    .collect();
                                if res.is_empty() {
                                    res.push(combination).unwrap();
                                }
                                res
                            })
                    })
            })
            .collect();

        // every potential combination that can get to triple or quad 100 for this armor set
        for combination in combinations {
            // todo: could we theoretically do this in one step and make it more performant?
            // get the theoretical mod cost we need to get to t3 / t4
            let mut costs = get_required_mod_costs(combination);

            // combination is more expensive than we can afford
            // lets just continue
            if costs.total > slots_count {
                continue;
            }

            let mut used_modslot_index: u8 = 0;

            // iterate backwards because we want to check for the most expensive mods first
            // we only need to check index 5, 4, 3 because less expensive mods are handled differently
            for cost in (3..6).rev() {
                // how many mods of this cost do we need?
                let mut cost_amount = costs.counters[cost];
                // if we don't need any, simply continue
                if cost_amount == 0 {
                    continue;
                }

                let mut counter = 0;
                for (i, d) in slots.iter().enumerate() {
                    if used_modslot_index & (1 << i) != 0 && *d >= cost as u8 {
                        continue;
                    }
                    // we can't use i due to the filter condition
                    if counter == cost {
                        break;
                    }
                    used_modslot_index |= 1 << i;
                    cost_amount -= 1;
                    counter += &1;
                }

                costs.decrement_by(cost, cost_amount);
                costs.increment_by(cost / 2, cost_amount);
                if costs.total > slots_count {
                    break;
                }
            }

            if costs.total <= available_modslots_count as u8 {
                if combination.stats[3].is_none() {
                    // triplet
                    runtime.stat_combo_3x_100.insert(combination.id());
                } else {
                    runtime
                        .stat_combo_3x_100
                        .insert(combination.id() - (1 << combination.stats[3].unwrap().0));
                    runtime.stat_combo_4x_100.insert(combination.id());
                }
            }
        }
    }

    if do_not_output {
        return (runtime, Err(PermutationError::DoNotOutput));
    }

    // calculate output
    if config.try_limit_wasted_stats && available_modslots_count > 0 {
        // todo: we have modslots remaining, let's see if we can limit wasted stats
        let mut waste: heapless::Vec<(u8, u8, u8), 6> = Waste::from(stats)
            .values
            .iter()
            .enumerate()
            .map(|(a, (w, v))| (a as u8, *w, *v))
            .filter(|(_, waste, value)| *waste >= 5 && *value < 100)
            .collect();
        waste.sort_by(|a, b| b.cmp(a));

        let mut id = 0;
        while id < available_modslots_count {
            let result = waste
                .iter()
                .filter(|(stat_id, waste, _)| {
                    slots
                        .iter()
                        .filter(|i| {
                            **i >= get_stat_mod_cost(
                                num::FromPrimitive::from_u8(1 + (stat_id * 2)).unwrap(),
                            )
                        })
                        .count()
                        > 0
                })
                .sorted_unstable_by(|(_, a, _), (_, b, _)| a.cmp(b))
                .next();
            let Some((stat_id, _, _)) = result else {
                break
            };
            let stat_id = usize::from(*stat_id);

            let tier = config
                .minimum_stat_tiers
                .get(&num::FromPrimitive::from_usize(stat_id).unwrap())
                .unwrap();
            if tier.fixed && (stats.values[stat_id] + 5) / 10 >= tier.value + 1 {
                waste[stat_id].1 -= 5;
                continue;
            }

            *slots
                .iter_mut()
                .find(|slot| {
                    **slot
                        >= get_stat_mod_cost(
                            num::FromPrimitive::from_usize(1 + (stat_id * 2)).unwrap(),
                        )
                })
                .unwrap() = 0;

            available_modslots_count -= 1;
            stats.values[stat_id] += 5;
            waste[stat_id].1 -= 5;
            used_mods.push(1 + 2 * stat_id as u8).unwrap();
            id += &1;
        }
    }

    let waste = stats.waste_sum();
    if config.only_show_results_with_no_wasted_stats && waste > 0 {
        // todo: do we need this? we have a similar check above
        return (runtime, Err(PermutationError::WastedStats));
    }

    (runtime, Ok(StrippedItemResult {
        exotic: set.exotic().map(|e| e.hash),
        mod_count: used_mods.len(),
        mod_cost: used_mods.iter().fold(0_u8, |acc, s| {
            acc + get_stat_mod_cost(num::FromPrimitive::from_u8(*s).unwrap())
        }),
        mods: used_mods
            .iter()
            .map(|s| num::FromPrimitive::from_u8(*s).unwrap())
            .collect(),
        stats,
        stats_no_mods: base_stats,
        tiers: get_skill_tier(stats),
        waste,
        items: *set,
    }))
}

fn get_skill_tier(stats: Stats) -> u8 {
    stats.values.iter().map(|s| 100.min(*s) / 10).sum()
}

fn get_required_mod_costs(combination: ArmorCombination) -> MultiCounter<6> {
    let mut required_mod_costs: MultiCounter<6> = MultiCounter::new();
    combination
        .stats
        .iter()
        .flatten()
        .filter(|s| s.1 > 0)
        .for_each(|s| {
            let id = s.0;
            let cost = s.1;
            let id = 1 + (id * 2);
            let minor = get_stat_mod_cost(num::FromPrimitive::from_u8(id).unwrap());
            let major = get_stat_mod_cost(num::FromPrimitive::from_u8(id + 1).unwrap());

            let cost = 0.max(cost);
            // how many major mods do we need
            let mut major_count = cost / 10; // integer floor division
                                             // do we need a minor mod?
            let rest = cost % 10;
            if rest > 5 {
                major_count += 1;
            } else if rest > 0 {
                required_mod_costs.increment(minor as usize);
            }

            required_mod_costs.increment_by(major as usize, major_count as u8);
        });

    required_mod_costs
}

pub struct MultiCounter<const N: usize> {
    pub counters: [u8; N],
    total: u8,
}

impl<const N: usize> MultiCounter<N> {
    pub fn new() -> Self {
        Self {
            counters: [0; N],
            total: 0,
        }
    }

    pub fn increment(&mut self, index: usize) {
        self.counters[index] += 1; // todo: use checked add
        self.total += 1;
    }

    pub fn increment_by(&mut self, index: usize, n: u8) {
        self.counters[index] += n; // todo: use checked add
        self.total += n;
    }

    pub fn decrement(&mut self, index: usize) {
        self.counters[index] -= 1;
        self.total = self.total.checked_sub(1).unwrap() // todo: pass result up
    }

    pub fn decrement_by(&mut self, index: usize, n: u8) {
        self.counters[index] -= n;
        self.total = self.total.checked_sub(n).unwrap() // todo: pass result up
    }
}

/// A stat is a tuple of a stat id and a modifier value (which can be negative)
#[derive(Debug, Copy, Clone, Default)]
pub struct Stat(u8, i16);

impl Stat {
    fn mod_cost(self) -> u8 {
        ((self.1.max(0) + 9) / 10) as u8
    }
}

#[derive(Debug, Copy, Clone, Default)]
pub struct ArmorCombination {
    stats: [Option<Stat>; 4],
}

impl ArmorCombination {
    pub fn new() -> Self {
        ArmorCombination {
            stats: Default::default(),
        }
    }

    pub fn with(mut self, i: usize, s: Stat) -> Self {
        self.stats[i] = Some(s);
        self
    }

    pub fn id(self) -> u64 {
        self.stats
            .iter()
            .filter(|s| s.is_some())
            .fold(0, |acc, s| acc + (1 << s.unwrap().0))
    }
}

/// ensure that the elemental affinities on the armor set fit into the requirements
pub(crate) fn check_elements(
    config: &WorkerConfig,
    requirements: [u8; 7], // todo: we never use ghost / subclass so this could be reduced to 5
    available_class_elements: &HashSet<DestinyEnergyType>,
    set: &ArmorSet,
) -> (bool, DestinyEnergyType) {
    let mut requirements = requirements.map(|i| i as i16);

    let mut wildcard = requirements[0];

    // todo: find a better solution for this (enums?)
    calc_el_req(&mut requirements, &mut wildcard, config, &set.helmet);
    calc_el_req(&mut requirements, &mut wildcard, config, &set.gauntlets);
    calc_el_req(&mut requirements, &mut wildcard, config, &set.chest);
    calc_el_req(&mut requirements, &mut wildcard, config, &set.legs);

    let mut bad: i16 = requirements
        .iter()
        .skip(1) // skip first element
        .map(|i| 0.max(*i))
        .reduce(|acc, e| acc + e)
        .unwrap()
        - wildcard;

    let mut req_class_element = DestinyEnergyType::Any;

    let class_armor_affinity = config
        .armor_affinities
        .get(&ArmorSlot::ArmorSlotClass)
        .unwrap();
    if class_armor_affinity.fixed {
        req_class_element = class_armor_affinity.value;
    }

    if bad == 1
        && !(class_armor_affinity.fixed && class_armor_affinity.value != DestinyEnergyType::Any)
    {
        for i in [
            DestinyEnergyType::Void,
            DestinyEnergyType::Stasis,
            DestinyEnergyType::Thermal,
            DestinyEnergyType::Arc,
        ] {
            if requirements[i as usize] <= 0 {
                continue;
            }
            if available_class_elements.contains(&i) {
                req_class_element = i;
                bad -= 1;
                break;
            }
        }
    }

    (bad <= 0, req_class_element)
}

fn calc_el_req(
    requirements: &mut [i16; 7],
    wildcard: &mut i16,
    config: &WorkerConfig,
    item: &StrippedInventoryArmor,
) {
    if (item.masterworked && config.ignore_armor_affinities_on_masterworked_items)
        || (!item.masterworked && config.ignore_armor_affinities_on_non_masterworked_items)
    {
        *wildcard += 1;
    } else {
        requirements[item.energy_affinity as usize] -= 1;
    }
}

/// ensure we can fit our mod slots into this set
pub fn check_slots(
    config: &WorkerConfig,
    requirements: &[u8; 12],
    available_class_item_perk_types: &HashMap<&ArmorPerkOrSlot, HashSet<DestinyEnergyType>>,
    set: &ArmorSet,
) -> (bool, Option<ArmorPerkOrSlot>) {
    let mut requirements = requirements.map(|i| i as i16);
    let exotic = config.selected_exotic;
    if !exotic.is(set.helmet.hash) && !is_item_applicable_to_slot(config, &set.helmet) {
        return (false, None);
    }

    if !exotic.is(set.gauntlets.hash) && !is_item_applicable_to_slot(config, &set.gauntlets) {
        return (false, None);
    }

    if !exotic.is(set.chest.hash) && !is_item_applicable_to_slot(config, &set.chest) {
        return (false, None);
    }

    if !exotic.is(set.legs.hash) && !is_item_applicable_to_slot(config, &set.legs) {
        return (false, None);
    }

    let perk = config.armor_perks.get(&ArmorSlot::ArmorSlotClass).unwrap();
    if perk.fixed
        && perk.value != ArmorPerkOrSlot::None
        && !available_class_item_perk_types.contains_key(&perk.value)
    {
        return (false, None);
    }

    requirements[set.helmet.perk as usize] -= 1;
    requirements[set.gauntlets.perk as usize] -= 1;
    requirements[set.chest.perk as usize] -= 1;
    requirements[set.legs.perk as usize] -= 1;

    if let ExoticChoiceModel::Some(hash) = exotic {
        if set.helmet.hash == hash {
            requirements[config.armor_perks.get(&set.helmet.slot).unwrap().value as usize] -= 1;
        } else if set.gauntlets.hash == hash {
            requirements[config.armor_perks.get(&set.gauntlets.slot).unwrap().value as usize] -= 1;
        } else if set.chest.hash == hash {
            requirements[config.armor_perks.get(&set.chest.slot).unwrap().value as usize] -= 1;
        } else if set.legs.hash == hash {
            requirements[config.armor_perks.get(&set.legs.slot).unwrap().value as usize] -= 1;
        }
    }

    let mut bad = 0;
    for i in requirements {
        bad += 0.max(i)
    }

    let mut required_class_item = ArmorPerkOrSlot::None;
    if bad == 1 {
        for i in 0..12 {
            if requirements[i] <= 0 {
                continue;
            }
            let perk: ArmorPerkOrSlot = i.try_into().unwrap();
            if available_class_item_perk_types.contains_key(&perk) {
                bad -= 1;
                required_class_item = perk;
                break;
            }
        }
    } else {
        let perk = config.armor_perks.get(&ArmorSlot::ArmorSlotClass).unwrap();
        if perk.fixed {
            required_class_item = perk.value;
        }
    }
    (bad <= 0, Some(required_class_item))
}

// todo: fixable selection as enum
fn is_item_applicable_to_slot(config: &WorkerConfig, item: &StrippedInventoryArmor) -> bool {
    let perk = config.armor_perks.get(&item.slot).unwrap();
    !(perk.fixed && perk.value != ArmorPerkOrSlot::None && perk.value != item.perk)
}

pub fn prepare_constant_modslot_requirement(
    armor_perks: &HashMap<ArmorSlot, FixableSelection<ArmorPerkOrSlot>>,
) -> [u8; 12] {
    let mut req: [u8; 12] = Default::default();
    req[armor_perks.get(&ArmorSlot::ArmorSlotHelmet).unwrap().value as usize] += 1;
    req[armor_perks.get(&ArmorSlot::ArmorSlotChest).unwrap().value as usize] += 1;
    req[armor_perks
        .get(&ArmorSlot::ArmorSlotGauntlet)
        .unwrap()
        .value as usize] += 1;
    req[armor_perks.get(&ArmorSlot::ArmorSlotLegs).unwrap().value as usize] += 1;
    req[armor_perks.get(&ArmorSlot::ArmorSlotClass).unwrap().value as usize] += 1;
    req
}

/// returns an array of five elements denoting the available mod slots and their capacity.
/// this is sorted in ascending order so cost optimisation can simply take the first matching element
pub fn prepare_constant_available_modslots(
    max_mod_slots: &HashMap<ArmorSlot, FixableSelection<u8>>,
) -> [u8; 5] {
    let mut req: [u8; 5] = Default::default();
    req[0] = max_mod_slots
        .get(&ArmorSlot::ArmorSlotHelmet)
        .unwrap()
        .value;
    req[1] = max_mod_slots.get(&ArmorSlot::ArmorSlotChest).unwrap().value;
    req[2] = max_mod_slots
        .get(&ArmorSlot::ArmorSlotGauntlet)
        .unwrap()
        .value;
    req[3] = max_mod_slots.get(&ArmorSlot::ArmorSlotLegs).unwrap().value;
    req[4] = max_mod_slots.get(&ArmorSlot::ArmorSlotClass).unwrap().value;
    req.sort();
    req
}

pub fn prepare_constant_stat_bonus(enabled_mods: &Vec<StatMod>, class: CharacterClass) -> StatsMod {
    let mut constant_bonus: StatsMod = Default::default();
    for stat_mod in enabled_mods {
        let modifiers = get_modifiers(class, *stat_mod);
        constant_bonus = constant_bonus + modifiers;
    }
    constant_bonus
}

pub fn prepare_constant_element_requirement(
    armor_affinities: &HashMap<ArmorSlot, FixableSelection<DestinyEnergyType>>,
) -> [u8; 7] {
    let mut cer: [u8; 7] = Default::default();
    cer[armor_affinities
        .get(&ArmorSlot::ArmorSlotHelmet)
        .unwrap()
        .value as usize] += 1;
    cer[armor_affinities
        .get(&ArmorSlot::ArmorSlotChest)
        .unwrap()
        .value as usize] += 1;
    cer[armor_affinities
        .get(&ArmorSlot::ArmorSlotGauntlet)
        .unwrap()
        .value as usize] += 1;
    cer[armor_affinities
        .get(&ArmorSlot::ArmorSlotLegs)
        .unwrap()
        .value as usize] += 1;

    let class_item_affinity = armor_affinities.get(&ArmorSlot::ArmorSlotClass).unwrap();
    if !class_item_affinity.fixed {
        cer[class_item_affinity.value as usize] += 1;
    }
    cer[0] = 0;
    cer
}

/// bonus, cost, mod hash
fn get_stat_mod_cost(index: StatModifier) -> u8 {
    match index {
        StatModifier::None => unreachable!("passed StatModifier::None to stat mod cost query"),
        StatModifier::MinorMobility => 1,
        StatModifier::MajorMobility => 3,
        StatModifier::MinorResilience => 1,
        StatModifier::MajorResilience => 3,
        StatModifier::MinorRecovery => 2,
        StatModifier::MajorRecovery => 4,
        StatModifier::MinorDiscipline => 1,
        StatModifier::MajorDiscipline => 3,
        StatModifier::MinorIntellect => 2,
        StatModifier::MajorIntellect => 5,
        StatModifier::MinorStrength => 1,
        StatModifier::MajorStrength => 3,
    }
}

fn get_modifiers(class: CharacterClass, moa: StatMod) -> ModGroup {
    match moa {
        StatMod::PowerfulFriends => ModGroup::single(SimpleArmorStat::Mobility, 20),
        StatMod::RadiantLight => ModGroup::single(SimpleArmorStat::Strength, 20),
        StatMod::ProtectiveLight => ModGroup::single(SimpleArmorStat::Strength, -10),
        StatMod::ExtraReserves => ModGroup::single(SimpleArmorStat::Intellect, -10),
        StatMod::PreciselyCharged => ModGroup::single(SimpleArmorStat::Discipline, -10),
        StatMod::StacksOnStacks => ModGroup::single(SimpleArmorStat::Recovery, -10),
        StatMod::PrecisionCharge => ModGroup::single(SimpleArmorStat::Strength, -10),
        StatMod::SurpriseAttack => ModGroup::single(SimpleArmorStat::Intellect, -10),
        StatMod::EnergyConverter => ModGroup::single(SimpleArmorStat::Discipline, -10),
        StatMod::ChargeHarvester => ModGroup::single(SimpleArmorStat::for_class(class), -10),
        // stasis mods
        StatMod::WhisperOfDurance => ModGroup::single(SimpleArmorStat::Strength, 10),
        StatMod::WhisperOfChains => ModGroup::single(SimpleArmorStat::Recovery, 10),
        StatMod::WhisperOfShards => ModGroup::single(SimpleArmorStat::Resilience, 10),
        StatMod::WhisperOfConduction => ModGroup::Double(
            SimpleModifierValue::new(SimpleArmorStat::Resilience, 10),
            SimpleModifierValue::new(SimpleArmorStat::Intellect, 10),
        ),
        StatMod::WhisperOfBonds => ModGroup::Double(
            SimpleModifierValue::new(SimpleArmorStat::Discipline, -10),
            SimpleModifierValue::new(SimpleArmorStat::Intellect, -10),
        ),
        StatMod::WhisperOfHedrons => ModGroup::single(SimpleArmorStat::Strength, -10),
        StatMod::WhisperOfFractures => ModGroup::single(SimpleArmorStat::Discipline, -10),
        StatMod::WhisperOfHunger => ModGroup::Double(
            SimpleModifierValue::new(SimpleArmorStat::Mobility, -10),
            SimpleModifierValue::new(SimpleArmorStat::Recovery, -10),
        ),
        StatMod::EchoOfExpulsion => ModGroup::single(SimpleArmorStat::Intellect, -10),
        StatMod::EchoOfProvision => ModGroup::single(SimpleArmorStat::Strength, -10),
        StatMod::EchoOfPersistence => ModGroup::single(SimpleArmorStat::for_class(class), -10),
        StatMod::EchoOfLeeching => ModGroup::single(SimpleArmorStat::Resilience, 10),
        StatMod::EchoOfDomineering => ModGroup::single(SimpleArmorStat::Discipline, 10),
        StatMod::EchoOfDilation => ModGroup::Double(
            SimpleModifierValue::new(SimpleArmorStat::Mobility, 10),
            SimpleModifierValue::new(SimpleArmorStat::Intellect, 10),
        ),
        StatMod::EchoOfUndermining => ModGroup::single(SimpleArmorStat::Discipline, -20),
        StatMod::EchoOfInstability => ModGroup::single(SimpleArmorStat::Strength, 10),
        StatMod::EchoOfHarvest => ModGroup::single(SimpleArmorStat::Intellect, -10),
        StatMod::EchoOfObscurity => ModGroup::single(SimpleArmorStat::Recovery, 10),
        StatMod::EchoOfStarvation => ModGroup::single(SimpleArmorStat::Recovery, -10),
        StatMod::EmberOfBenelovence => ModGroup::single(SimpleArmorStat::Discipline, -10),
        StatMod::EmberOfBeams => ModGroup::single(SimpleArmorStat::Intellect, 10),
        StatMod::EmberOfEmpyrean => ModGroup::single(SimpleArmorStat::Resilience, -10),
        StatMod::EmberOfCombustion => ModGroup::single(SimpleArmorStat::Strength, 10),
        StatMod::EmberOfChar => ModGroup::single(SimpleArmorStat::Discipline, 10),
        StatMod::EmberOfTempering => ModGroup::single(SimpleArmorStat::Recovery, -10),
        StatMod::EmberOfEruption => ModGroup::single(SimpleArmorStat::Strength, 10),
        StatMod::EmberOfWonder => ModGroup::single(SimpleArmorStat::Resilience, 10),
        StatMod::EmberOfSearing => ModGroup::single(SimpleArmorStat::Recovery, 10),
        StatMod::SparkOfBrilliance => ModGroup::single(SimpleArmorStat::Intellect, 10),
        StatMod::SparkOfFeedback => ModGroup::single(SimpleArmorStat::Resilience, 10),
        StatMod::SparkOfDischarge => ModGroup::single(SimpleArmorStat::Strength, -10),
        StatMod::SparkOfFocus => ModGroup::single(SimpleArmorStat::for_class(class), -10),
        StatMod::SparkOfVolts => ModGroup::single(SimpleArmorStat::Recovery, 10),
        StatMod::SparkOfResistance => ModGroup::single(SimpleArmorStat::Strength, 10),
        StatMod::SparkOfShock => ModGroup::single(SimpleArmorStat::Discipline, -10),
    }
}
