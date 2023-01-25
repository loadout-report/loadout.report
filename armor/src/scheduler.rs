use crate::armory::Armory;
use crate::db::build_database;
use crate::model::{ArmorInformation, ArmorPerkOrSlot, ArmorSlot, CharacterClass, DestinyEnergyType, ExoticChoiceModel, InventoryArmor, ManifestArmor, StrippedInventoryArmor, TierType, WorkerConfig};
use crate::worker::{ArmorWorker, check_slots, Input, ItemResult, Output, PermutationError, prepare_constant_available_modslots, prepare_constant_element_requirement, prepare_constant_modslot_requirement, prepare_constant_stat_bonus, Runtime};
use gloo_worker::Spawnable;
use rexie::{KeyRange, Rexie, TransactionMode};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::str::FromStr;
use log::{debug, info, warn};
use num_traits::FromPrimitive;
use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen::JsValue;

#[cfg(feature = "parallel")]
use rayon::prelude::*;
use crate::db;

#[derive(Debug)]
pub enum ComputeError {
    Unknown,
    ConfigParseError(serde_wasm_bindgen::Error),
    DataParseError(serde_wasm_bindgen::Error),
    LoadDataError(rexie::Error),
}

impl Into<JsValue> for ComputeError {
    fn into(self) -> JsValue {
        match self {
            ComputeError::Unknown => JsValue::from_str("unknown error"),
            ComputeError::ConfigParseError(err) => {
                JsValue::from_str(format!("config parse error, {}", err.to_string()).as_str())
            }
            ComputeError::LoadDataError(err) => {
                JsValue::from_str(format!("could not load source data, {}", err.to_string()).as_str())
            }
            ComputeError::DataParseError(err) => {
                JsValue::from_str(format!("could not parse source data, {}", err.to_string()).as_str())
            }
        }
    }
}


#[cfg(feature = "parallel")]
pub async fn compute_results(config: WorkerConfig, armor: Vec<InventoryArmor>, selected_exotic: &Option<ManifestArmor>) -> Result<usize, ComputeError> {

    debug!("selected exotic: {:?}", selected_exotic);
    debug!("loaded armor pieces: {}", armor.len());

    let armor: Vec<StrippedInventoryArmor> = armor.into_iter().map(|i| Into::into(i)).collect();

    let armory: Armory = armor
        .iter()
        .filter(|i| i.slot != ArmorSlot::ArmorSlotNone)
        .filter(|i| !config.disabled_items.contains(&i.item_instance_id))
        .filter(|i| !config.use_fotl_armor || i.slot != ArmorSlot::ArmorSlotHelmet || i.is_fotl_mask())
        .filter(|i| config.selected_exotic != ExoticChoiceModel::None || !i.is_exotic)
        .filter(|i| {
            config.selected_exotic == ExoticChoiceModel::All
                || selected_exotic.is_some_and(|e| e.slot != i.slot || e.hash == i.hash)
        })
        .filter(|i| !config.only_use_masterworked_items || i.masterworked)
        .filter(|i| {
            config.allow_blue_armor_pieces
                || i.rarity == TierType::Exotic
                || i.rarity == TierType::Superior
        })
        .filter(|i| !config.ignore_sunset_armor || !i.is_sunset)
        .filter(|i| {
            let affinity = config.armor_affinities.get(&i.slot).unwrap();
            !affinity.fixed
                || affinity.value == DestinyEnergyType::Any
                || (i.masterworked && config.ignore_armor_affinities_on_masterworked_items)
                || (!i.masterworked && config.ignore_armor_affinities_on_non_masterworked_items)
                || affinity.value == i.energy_affinity
        })
        .filter(|i| {
            let perks = config.armor_perks.get(&i.slot).unwrap();
            i.is_exotic
                || !perks.fixed
                || perks.value == ArmorPerkOrSlot::None
                || perks.value == i.perk
        })
        .map(|i| {
            if (i.is_exotic && config.assume_exotics_masterworked)
                || config.assume_legendaries_masterworked
                || (i.slot == ArmorSlot::ArmorSlotClass && config.assume_class_item_masterworked)
            {
                return i.assume_masterwork();
            }
            *i
        })
        .collect();


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

    let mut runtime = Runtime {
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

    // let mut results: Vec<ItemResult> = Vec::with_capacity(5000);

    let mut listed_results = 0;
    let mut total_results = 0;
    let mut do_not_output = false;


    let count = armory.into_iter()
        .take(1_000_000) // max 1 million for now
        .par_bridge()
        .flat_map(|set| {
            // can we fit our mod slots into this set?
            let (ok, required_class_item) = check_slots(
                &config,
                &modslot_requirement,
                &available_class_item_energy_perk_dict,
                &set,
            );
            if !ok {
                return Err(PermutationError::ConstraintsExceeded);
            }
            // can we fit our element requirements into this set?
            let mut required_class_el = DestinyEnergyType::Any;
            if must_check_element_req {
                let (ok, req_cel) = crate::worker::check_elements(
                    &config, element_requirement, available_class_item_energy_perk_dict.get(&required_class_item.unwrap_or_default())
                        .unwrap_or(&HashSet::new()), &set);
                if !ok {
                    return Err(PermutationError::ConstraintsExceeded);
                }
                required_class_el = req_cel;
            }
            // calculate the permutation
            let (runtime, result) = crate::worker::handle_permutation(
                &config, &set,
                stat_bonus, available_modslots, do_not_output,
            );
            return result.map(|r| (runtime, r, required_class_item, required_class_el));
        })
        .count();

    // let vec: Vec<()> = Vec::new();
    Ok(count)
}

#[cfg(not(feature = "parallel"))]
pub async fn compute_results(config: WorkerConfig) -> Result<Vec<()>, ComputeError> {
    let vec: Vec<()> = Vec::new();
    Ok(vec)
}
