use std::collections::{HashMap, HashSet};
use std::convert::TryInto;
use std::iter::{once, Once};
use std::ops::RangeBounds;
use std::time::{Duration, Instant};
use gloo_worker::{HandlerId, Worker, WorkerScope};
use rexie::{KeyRange, TransactionMode};
use rexie::Error::DomExceptionNotFound;
use serde::{Deserialize, Serialize};
use wasm_bindgen::JsValue;
use itertools::Itertools;
use data::api::manifest::model::Hash;
use crate::db::build_database;
use crate::model::{ArmorInformation, ArmorPerkOrSlot, ArmorSet, ArmorSlot, ArmorStat, CharacterClass, DestinyEnergyType, ExoticChoiceModel, FixableSelection, InventoryArmor, Item, ManifestArmor, ModGroup, ModifierValue, Msg, SimpleArmorStat, SimpleModifierValue, StatMod, StatModifier, StrippedInventoryArmor, ThreadConfig, TierType, WorkerConfig};
use crate::model::stats::{Stats, StatsMod};

pub struct ArmorWorker {
    db: Option<rexie::Rexie>,
    ready: bool,
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Input {
    pub current_class: CharacterClass,
    pub config: WorkerConfig,
    pub thread_split: ThreadConfig,
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ExoticResult {
    icon: String,
    watermark: String,
    name: String,
    hash: Hash,
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ItemResult {
    exotic: Option<ExoticResult>,
    mod_count: usize,
    mod_cost: u8,
    mods: Vec<StatModifier>,
    stats: Stats,
    stats_no_mods: Stats,
    tiers: u8,
    waste: u8,
    items: Vec<Item>,
    class_item: ClassItem,
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ClassItem {
    perk: ArmorPerkOrSlot,
    affinity: DestinyEnergyType
}

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
    maximum_possible_tiers: [u8; 6],
    stat_combo_3x_100: HashSet<u8>,
    stat_combo_4x_100: HashSet<u8>,
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
    runtime: Runtime,
    results: Vec<ItemResult>,
    total: Option<usize>,
    done: bool,
    stats: Option<OutputStats>,
}

impl Worker for ArmorWorker {
    type Message = Msg<()>;
    type Input = Input;
    type Output = Output;

    fn create(scope: &WorkerScope<Self>) -> Self {
        {
            scope.send_future(async {
                let db = build_database().await.unwrap();
                Msg::Ready(db)
            });
        }


        Self {
            db: None,
            ready: false,
        }
    }

    fn update(&mut self, _: &WorkerScope<Self>, msg: Self::Message) {
        if let Msg::Ready(db) = msg {
            self.db = Some(db);
            self.ready = true;
        }
    }

    fn received(&mut self, scope: &WorkerScope<Self>, msg: Self::Input, id: HandlerId) {
        // run job
        if !self.ready {
            // todo: respond with error
            return;
        }

        scope.send_future(async {
            run_job(scope, id, self.db.unwrap(), msg).await.unwrap();
            Msg::Done
        });

    }
}

async fn run_job(scope: &WorkerScope<ArmorWorker>, id: HandlerId, db: rexie::Rexie, data: Input) -> Result<(), Box<dyn std::error::Error>> {
    let start_time = Instant::now();
    let config = data.config;

    let selected_exotic = if let ExoticChoiceModel::Some(exotic) = config.selected_exotic {
        let tx = db.transaction(&["manifestArmor"], TransactionMode::ReadOnly)?;
        let manifest_armor = tx.store("manifestArmor").unwrap();
        let armor = manifest_armor.index("hash").unwrap().get(&JsValue::from(exotic)).await?;
        let armor: ManifestArmor = serde_wasm_bindgen::from_value(armor)?;
        tx.done().await?;
        Some(armor)
    } else {
        None
    };

    // todo: single iteration to split vectors
    let armor: Vec<InventoryArmor> = {
        let tx = db.transaction(&["inventoryArmor"], TransactionMode::ReadOnly)?;
        let inventory_armor = tx.store("inventory_armor")?;
        let key_range = KeyRange::only(&(config.character_class as u32).into()).unwrap();
        let armor = inventory_armor.index("clazz").unwrap();
        let armor = armor.get_all(Some(&key_range), None, None, None).await?;
        armor.iter().into_iter()
            .map(|(_, value)| serde_wasm_bindgen::from_value(value.clone()).unwrap())
            .collect()
    };

    let armor_info: HashMap<Hash, ArmorInformation> = armor.iter()
        .map(|i| (i.hash, Into::into(i.clone())))
        .collect();

    let armor: Vec<StrippedInventoryArmor> = armor.iter()
        .map(|i| Into::into(i.clone()))
        .collect();

    let armor: Vec<_> = armor.iter()
        .filter(|i| i.slot != ArmorSlot::ArmorSlotNone)
        .filter(|i| !config.disabled_items.contains(&i.item_instance_id))
        .filter(|i| config.selected_exotic != ExoticChoiceModel::None || !i.is_exotic)
        .filter(|i| config.selected_exotic == ExoticChoiceModel::All || selected_exotic.is_some_and(|e| e.slot != i.slot || e.hash == i.hash))
        .filter(|i| !config.only_use_masterworked_items || i.masterworked)
        .filter(|i| config.allow_blue_armor_pieces || i.rarity == TierType::Exotic || i.rarity == TierType::Superior)
        .filter(|i| !config.ignore_sunset_armor || !i.is_sunset)
        .filter(|i| {
            let affinity = config.armor_affinities.get(&i.slot).unwrap();
            !affinity.fixed ||
                affinity.value == DestinyEnergyType::Any ||
                (i.masterworked && config.ignore_armor_affinities_on_masterworked_items) ||
                (!i.masterworked && config.ignore_armor_affinities_on_non_masterworked_items) ||
                affinity.value == i.energy_affinity
        })
        .filter(|i| {
            let perks = config.armor_perks.get(&i.slot).unwrap();
            i.is_exotic ||
                !perks.fixed ||
                perks.value == ArmorPerkOrSlot::None ||
                perks.value == i.perk
        })
        .map(|i| {
            if (i.is_exotic && config.assume_exotics_masterworked)
                || config.assume_legendaries_masterworked
                || (i.slot == ArmorSlot::ArmorSlotClass && config.assume_class_item_masterworked) {
                return i.assume_masterwork();
            }
            *i
        })
        .collect();

    let (mut helmets, mut gauntlets, mut chests, mut legs, class_items): (
        Vec<StrippedInventoryArmor>,
        Vec<StrippedInventoryArmor>,
        Vec<StrippedInventoryArmor>,
        Vec<StrippedInventoryArmor>,
        Vec<StrippedInventoryArmor>,
    ) = armor.iter().fold(Default::default(), |mut t, i| {
        match i.slot {
            ArmorSlot::ArmorSlotNone => (),
            ArmorSlot::ArmorSlotHelmet => t.0.push(*i),
            ArmorSlot::ArmorSlotGauntlet => t.1.push(*i),
            ArmorSlot::ArmorSlotChest => t.2.push(*i),
            ArmorSlot::ArmorSlotLegs => t.3.push(*i),
            ArmorSlot::ArmorSlotClass => t.4.push(*i),
        }
        t
    });

    // threading
    if data.thread_split.count > 1 {
        let sets: [usize; 4] = [helmets.len(), gauntlets.len(), chests.len(), legs.len()];
        let max = sets.iter().max().unwrap();
        let index = sets.iter().position(|l| l == max).unwrap();
        let keep_length = max / data.thread_split.count;
        let start_index = keep_length * data.thread_split.current;
        let mut end_index = keep_length * (data.thread_split.current + 1);
        if keep_length * data.thread_split.count != *max && data.thread_split.current == data.thread_split.count - 1 {
            end_index += max - keep_length * data.thread_split.count;
        }

        match index {
            0 => {
                helmets.drain(end_index..);
                helmets.drain(0..start_index);
            }
            1 => {
                gauntlets.drain(end_index..);
                gauntlets.drain(0..start_index);
            }
            2 => {
                chests.drain(end_index..);
                chests.drain(0..start_index);
            }
            3 => {
                legs.drain(end_index..);
                legs.drain(0..start_index);
            }
            _ => unreachable!(),
        }
    }

    let available_class_item_perk_types: HashSet<_> = class_items.iter()
        .map(|i| i.perk)
        .collect();

    let available_class_item_energy_perk_dict = available_class_item_perk_types.iter()
        .fold(HashMap::new(), |mut map, apos| {
            if !map.contains_key(apos) {
                map.insert(apos, HashSet::new());
            }
            if !map.contains_key(&ArmorPerkOrSlot::None) {
                map.insert(&ArmorPerkOrSlot::None, HashSet::new());
            }
            class_items.iter()
                .filter(|i| i.perk == *apos)
                .for_each(|i| {
                    map.get_mut(&ArmorPerkOrSlot::None).unwrap().insert(i.energy_affinity);
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

    for helmet in &helmets {
        for gauntlet in &gauntlets {
            for chest in &chests {
                for leg in &legs {
                    let set = ArmorSet::new(
                        *helmet,
                        *gauntlet,
                        *chest,
                        *leg,
                    );
                    let (ok, required_class_item) = check_slots(
                        &config, modslot_requirement, &available_class_item_energy_perk_dict,
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
                            available_class_item_energy_perk_dict.get(
                                &required_class_item.unwrap_or_default()
                            ).unwrap_or(&HashSet::new()), &set,
                        );
                        if !ok {
                            continue;
                        }
                        required_class_el = req_cel;
                    }

                    let result = handle_permutation(&mut runtime, &config, &set, stat_bonus, available_modslots, do_not_output);
                    match result {
                        Ok(result) => {
                            total_results += 1;

                            let result = ItemResult {
                                exotic: result.exotic.map(|e| {
                                    let info = armor_info.get(&e).unwrap().clone();
                                    ExoticResult {
                                        icon: info.icon,
                                        watermark: info.watermark,
                                        name: info.name,
                                        hash: e
                                    }
                                }),
                                mod_count: result.mod_count,
                                mod_cost: result.mod_cost,
                                mods: result.mods.to_vec(),
                                stats: result.stats,
                                stats_no_mods: result.stats_no_mods,
                                tiers: result.tiers,
                                waste: result.waste,
                                items: result.items.iter().map(|i| {
                                    let armor_info = armor_info.get(&i.hash).unwrap().clone();
                                    Item {
                                        energy: i.energy_affinity,
                                        energyLevel: i.energy_level,
                                        hash: i.hash,
                                        item_instance_id: i.item_instance_id,
                                        name: armor_info.name,
                                        exotic: i.is_exotic,
                                        masterworked: i.masterworked,
                                        may_be_bugged: i.may_be_bugged,
                                        slot: i.slot,
                                        perk: i.perk,
                                        transfer_state: 0,
                                        stats: i.stats
                                    }
                                }).collect(),
                                class_item: ClassItem {
                                    perk: required_class_item.unwrap_or_default(),
                                    affinity: required_class_el
                                }
                            };

                            results.push(result);
                            listed_results += 1;
                            do_not_output = do_not_output || (config.limit_parsed_results && listed_results >= 50_000_usize / data.thread_split.count) || listed_results >= 1_000_000_usize / data.thread_split.count;
                            if results.len() >= 5000 {
                                // todo: respond to caller
                                scope.respond(id, Output {
                                    runtime: runtime.clone(),
                                    results,
                                    total: None,
                                    done: false,
                                    stats: None
                                });
                                results = Vec::with_capacity(5000);
                            }
                        },
                        Err(err) => if let PermutationError::DoNotOutput = err { total_results += 1 },
                    }

                }
            }
        }
    }

    scope.respond(id, Output {
        runtime,
        results,
        total: None,
        done: true,
        stats: Some(OutputStats {
            permutation_count: total_results,
            item_count: armor.len() - class_items.len(),
            total_time: start_time.elapsed(),
        })
    });

    Ok(())
}

#[derive(Copy, Clone)]
pub enum PermutationError {
    Unknown,
    TooManyMods,
    WastedStats,
    DoNotOutput,
}

fn handle_permutation(
    runtime: &mut Runtime, config: &WorkerConfig,
    set: &ArmorSet,
    bonus: StatsMod,
    mut slots: [u8; 5],
    do_not_output: bool,
) -> Result<StrippedItemResult, PermutationError> {

    // todo: verify not checking for masterworks here is okay (we did that near the filter section)
    // todo: we're ignoring the add constant 1 resilience option for now
    let mut base_stats = set.stat_total();

    if !set.chest.is_exotic && config.add_constant_1_resilience {
        base_stats = base_stats + SimpleModifierValue::new(SimpleArmorStat::Resilience, 1);
    }

    let mut stats = base_stats + bonus;

    for (stat, tier) in &config.minimum_stat_tiers {
        if tier.fixed && stats[*stat] > tier.value {
            return Err(PermutationError::TooManyMods);
        }
    }

    let mut required_mods = [
        0.max(config.minimum_stat_tiers.get(&SimpleArmorStat::Mobility).unwrap().value - stats[SimpleArmorStat::Mobility] / 10),
        0.max(config.minimum_stat_tiers.get(&SimpleArmorStat::Resilience).unwrap().value - stats[SimpleArmorStat::Resilience] / 10),
        0.max(config.minimum_stat_tiers.get(&SimpleArmorStat::Recovery).unwrap().value - stats[SimpleArmorStat::Recovery] / 10),
        0.max(config.minimum_stat_tiers.get(&SimpleArmorStat::Discipline).unwrap().value - stats[SimpleArmorStat::Discipline] / 10),
        0.max(config.minimum_stat_tiers.get(&SimpleArmorStat::Intellect).unwrap().value - stats[SimpleArmorStat::Intellect] / 10),
        0.max(config.minimum_stat_tiers.get(&SimpleArmorStat::Strength).unwrap().value - stats[SimpleArmorStat::Strength] / 10),
    ];

    let required_mods_total: u8 = required_mods.iter().sum();
    if required_mods_total > 5 {
        return Err(PermutationError::TooManyMods);
    }

    // todo: there may be a more efficient way of doing this
    let mut available_modslots_count = slots.iter()
        .filter(|i| 0 < **i)
        .count();

    if required_mods_total > available_modslots_count as u8 {
        return Err(PermutationError::TooManyMods);
    }

    // can we order this descending?
    let mut used_mods: heapless::Vec<u8, 5> = heapless::Vec::new();
    // we need mods
    if required_mods_total > 0 {

        // add any necessary mods
        for i in 0..6 { // iterate through stats
            if required_mods[i] == 0 {
                continue;
            }

            // add some minor mods
            let stat_diff = stats[i] % 10;
            if stat_diff >= 5 {
                let x = 1 + (i * 2) as u8;
                used_mods.insert(used_mods.binary_search(&x).unwrap_or_else(|pos| pos), x)
                    .map_err(|_| PermutationError::TooManyMods)?;
                required_mods[i] -= 1;
                stats[i] += 5;
            }

            // fill rest with major mods
            for _ in 0..required_mods[i] {
                let x = 2 + (i * 2) as u8;
                used_mods.insert(used_mods.binary_search(&x).unwrap_or_else(|pos| pos), x)
                    .map_err(|_| PermutationError::TooManyMods)?;
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
                    used_mods.insert(
                        used_mods.binary_search(&minor)
                            .unwrap_or_else(|pos| pos), minor)
                        .map_err(|_| PermutationError::TooManyMods)?;
                    used_mods.insert(
                        used_mods.binary_search(&minor)
                            .unwrap_or_else(|pos| pos), minor)
                        .map_err(|_| PermutationError::TooManyMods)?;
                    // step back to verify we can also place the two minor mods correctly (cost-wise)
                    i -= 1;
                } else {
                    // mod isn't a major mod, but we'd need to replace it. exit.
                    return Err(PermutationError::TooManyMods);
                }
            } else {
                // we can accommodate this mod just fine, let's mark the slot as used.
                slots.iter_mut().filter(|d| **d >= cost).take(1).for_each(|i| *i = 0);
                available_modslots_count -= 1;
            }
            i += 1;
        }
    }

    // if 5+ mods were used return

    // todo: we can likely optimise this a lot
    if config.only_show_results_with_no_wasted_stats {
        if stats.values.iter().any(|i| *i > 100 || *i % 5 != 0) {
            return Err(PermutationError::WastedStats);
        }

        let mut stats_wasted: heapless::Vec<(u8, usize, u8), 6> = stats.values.iter()
            .enumerate()
            .filter(|(i, x)| *x % 10 != 5) // only wasted stats
            .map(|(i, x)| (x % 10, i, *x)) // make into tuple (wasted, statId, total)
            .collect();
        // todo: desc sort
        // stats_wasted.sort_by(|a, b| a.0.cmp(&b.0));
        stats_wasted.sort_by(|a, b| a.0.cmp(&b.0));
        for (waste, stat, _) in &mut stats_wasted {
            if available_modslots_count == 0 { // we can only remove as many as we have mod slots
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
                    used_mods.insert(used_mods.binary_search(&mod_id).unwrap_or_else(|pos| pos), mod_id)
                        .map_err(|_| PermutationError::TooManyMods)?;
                    break;
                }
            }
            if *waste != 0 { // if we just went through all of those without fixing the wasted stat it means it's too expensive and we done goofed
                break;
            }
        }

        if get_waste(stats) > 0 {
            // we have waste.
            return Err(PermutationError::WastedStats);
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
            possible_100.push(Stat(index as u8, 100 - (current_stat as i16))).unwrap();
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

        let combinations: heapless::Vec<_, 20> = possible_100.iter()
            // pair each element with its index
            .enumerate()
            .take_while(|(index, _)| *index < possible_100.len() - 2)
            .map(|(index, i)| (index, ArmorCombination::new().with(0, *i), i.mod_cost()))
            .take_while(|(_, _, cost)| *cost <= slots_count)
            .flat_map(|(index, a, cost)| possible_100.iter()
                .enumerate()
                .skip(index + 1)
                .take_while(|(index, _)| *index < possible_100.len() - 1)
                .map(move |(index, i)| (index, a.with(1, *i), cost + i.mod_cost()))
                .take_while(|(_, _, cost)| *cost <= slots_count)
                .flat_map(|(index, combination, cost)| possible_100.iter()
                    .enumerate()
                    .skip(index + 1)
                    .map(move |(index, i)| (index, combination.with(2, *i), cost + i.mod_cost()))
                    .take_while(|(_, _, cost)| *cost <= slots_count)
                    .flat_map(|(index, combination, cost)| {
                        let mut res: heapless::Vec<_, 20> = possible_100.iter()
                            .enumerate()
                            .skip(index + 1)
                            .map(move |(index, i)| (index, combination.with(3, *i), cost + i.mod_cost()))
                            .take_while(|(_, _, cost)| *cost <= slots_count)
                            .map(|(_, combination, _)| combination)
                            .collect();
                        if res.is_empty() {
                            res.push(combination).unwrap();
                        }
                        res
                    })
                )
            )
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
                for (i, d) in slots
                    .iter()
                    .enumerate() {
                    if used_modslot_index & (1 << i) != 0 && *d >= cost as u8 {
                        continue;
                    }
                    // we can't use i due to the filter condition
                    if counter == cost {
                        break;
                    }
                    used_modslot_index |= 1 << i;
                    cost_amount -= 1;
                    counter += 1;
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
                    runtime.stat_combo_3x_100.insert(combination.id() - (1 << combination.stats[3].unwrap().0));
                    runtime.stat_combo_4x_100.insert(combination.id());
                }
            }
        }
    }

    if do_not_output {
        return Err(PermutationError::DoNotOutput)
    }

    if config.try_limit_wasted_stats && available_modslots_count > 0 {
        // todo: we have modslots remaining, let's see if we can limit wasted stats
    }

    let waste = get_waste(stats);
    if config.only_show_results_with_no_wasted_stats && waste > 0 {
        // todo: do we need this? we have a similar check above
        return Err(PermutationError::WastedStats)
    }

    Ok(StrippedItemResult {
        exotic: set.exotic().map(|e| e.hash),
        mod_count: used_mods.len(),
        mod_cost: used_mods.iter().fold(0_u8, |acc, s| acc + get_stat_mod_cost(num::FromPrimitive::from_u8(*s).unwrap())),
        mods: used_mods.iter().map(|s| num::FromPrimitive::from_u8(*s).unwrap()).collect(),
        stats,
        stats_no_mods: base_stats,
        tiers: get_skill_tier(stats),
        waste,
        items: *set,
    })
}

fn get_skill_tier(stats: Stats) -> u8 {
    stats.values.iter().map(|s| 100.min(*s) / 10).sum()
}

fn get_required_mod_costs(combination: ArmorCombination) -> MultiCounter<6> {
    let mut required_mod_costs: MultiCounter<6> = MultiCounter::new();
    combination.stats.iter()
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
        Self { counters: [0; N], total: 0 }
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
            stats: Default::default()
        }
    }

    pub fn with(mut self, i: usize, s: Stat) -> Self {
        self.stats[i] = Some(s);
        self
    }

    pub fn id(self) -> u8 {
        self.stats.iter().filter(|s| s.is_some()).fold(0, |acc, s| acc + (1 << s.unwrap().0))
    }
}

fn get_waste(stats: Stats) -> u8 {
    stats.values.iter().map(|i| if *i > 100 { i - 100 } else { i % 10 }).sum()
}

fn check_elements(
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

    let mut bad = (requirements
        .iter()
        .skip(1)// skip first element
        .map(|i| 0.max(*i))
        .reduce(|acc, e| acc + e).unwrap() - wildcard) as i16;

    let mut req_class_element = DestinyEnergyType::Any;

    let class_armor_affinity = config.armor_affinities.get(&ArmorSlot::ArmorSlotClass).unwrap();
    if class_armor_affinity.fixed {
        req_class_element = class_armor_affinity.value;
    }

    if bad == 1 && !(class_armor_affinity.fixed && class_armor_affinity.value != DestinyEnergyType::Any) {
        for i in [DestinyEnergyType::Void, DestinyEnergyType::Stasis, DestinyEnergyType::Thermal, DestinyEnergyType::Arc] {
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

fn calc_el_req(requirements: &mut [i16; 7], wildcard: &mut i16, config: &WorkerConfig, item: &StrippedInventoryArmor) {
    if (item.masterworked && config.ignore_armor_affinities_on_masterworked_items)
        || (!item.masterworked && config.ignore_armor_affinities_on_non_masterworked_items) {
        *wildcard += 1;
    } else {
        requirements[item.energy_affinity as usize] -= 1;
    }
}

fn check_slots(
    config: &WorkerConfig,
    requirements: [u8; 12],
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
    if perk.fixed && perk.value != ArmorPerkOrSlot::None && !available_class_item_perk_types.contains_key(&perk.value) {
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
        bad += i.max(0)
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

fn prepare_constant_modslot_requirement(armor_perks: &HashMap<ArmorSlot, FixableSelection<ArmorPerkOrSlot>>) -> [u8; 12] {
    let mut req: [u8; 12] = Default::default();
    req[armor_perks.get(&ArmorSlot::ArmorSlotHelmet).unwrap().value as usize] += 1;
    req[armor_perks.get(&ArmorSlot::ArmorSlotChest).unwrap().value as usize] += 1;
    req[armor_perks.get(&ArmorSlot::ArmorSlotGauntlet).unwrap().value as usize] += 1;
    req[armor_perks.get(&ArmorSlot::ArmorSlotLegs).unwrap().value as usize] += 1;
    req[armor_perks.get(&ArmorSlot::ArmorSlotClass).unwrap().value as usize] += 1;
    req
}

/// returns an array of five elements denoting the available mod slots and their capacity.
/// this is sorted in ascending order so cost optimisation can simply take the first matching element
fn prepare_constant_available_modslots(max_mod_slots: &HashMap<ArmorSlot, FixableSelection<u8>>) -> [u8; 5] {
    let mut req: [u8; 5] = Default::default();
    req[0] = max_mod_slots.get(&ArmorSlot::ArmorSlotHelmet).unwrap().value as u8;
    req[1] = max_mod_slots.get(&ArmorSlot::ArmorSlotChest).unwrap().value as u8;
    req[2] = max_mod_slots.get(&ArmorSlot::ArmorSlotGauntlet).unwrap().value as u8;
    req[3] = max_mod_slots.get(&ArmorSlot::ArmorSlotLegs).unwrap().value as u8;
    req[4] = max_mod_slots.get(&ArmorSlot::ArmorSlotClass).unwrap().value as u8;
    req.sort();
    req
}

fn prepare_constant_stat_bonus(enabled_mods: &Vec<StatMod>, class: CharacterClass) -> StatsMod {
    let mut constant_bonus: StatsMod = Default::default();
    for stat_mod in enabled_mods {
        let modifiers = get_modifiers(class, *stat_mod);
        constant_bonus = constant_bonus + modifiers;
    }
    constant_bonus
}

fn prepare_constant_element_requirement(armor_affinities: &HashMap<ArmorSlot, FixableSelection<DestinyEnergyType>>) -> [u8; 7] {
    let mut cer: [u8; 7] = Default::default();
    cer[armor_affinities.get(&ArmorSlot::ArmorSlotHelmet).unwrap().value as usize] += 1;
    cer[armor_affinities.get(&ArmorSlot::ArmorSlotChest).unwrap().value as usize] += 1;
    cer[armor_affinities.get(&ArmorSlot::ArmorSlotGauntlet).unwrap().value as usize] += 1;
    cer[armor_affinities.get(&ArmorSlot::ArmorSlotLegs).unwrap().value as usize] += 1;

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
