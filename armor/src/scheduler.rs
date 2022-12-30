use crate::armory::Armory;
use crate::db::build_database;
use crate::model::{
    ArmorInformation, ArmorPerkOrSlot, ArmorSlot, DestinyEnergyType, ExoticChoiceModel,
    InventoryArmor, ManifestArmor, StrippedInventoryArmor, TierType, WorkerConfig,
};
use crate::worker::{ArmorWorker, Input, Output};
use data::api::manifest::model::Hash;
use gloo_worker::Spawnable;
use rexie::{KeyRange, TransactionMode};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen::JsValue;

#[derive(Serialize, Deserialize)]
pub enum ComputeError {
    Unknown,
    LoadDataError,
}

impl Into<JsValue> for ComputeError {
    fn into(self) -> JsValue {
        match self {
            ComputeError::Unknown => JsValue::from_str("unknown error"),
            ComputeError::LoadDataError => JsValue::from_str("could not load source data"),
        }
    }
}

#[wasm_bindgen]
pub async fn compute_results(config: JsValue, n: usize) -> Result<(), ComputeError> {
    let config: WorkerConfig = serde_wasm_bindgen::from_value(config).unwrap();

    let db = build_database().await.unwrap();

    let selected_exotic = if let ExoticChoiceModel::Some(exotic) = config.selected_exotic {
        let tx = db
            .transaction(&["manifestArmor"], TransactionMode::ReadOnly)
            .map_err(|_| ComputeError::LoadDataError)?;
        let manifest_armor = tx.store("manifestArmor").unwrap();
        let armor = manifest_armor
            .index("hash")
            .unwrap()
            .get(&JsValue::from(exotic))
            .await
            .map_err(|_| ComputeError::LoadDataError)?;
        let armor: ManifestArmor =
            serde_wasm_bindgen::from_value(armor).map_err(|e| ComputeError::LoadDataError)?;
        tx.done().await.map_err(|e| ComputeError::LoadDataError)?;
        Some(armor)
    } else {
        None
    };

    // todo: single iteration to split vectors
    let armor: Vec<InventoryArmor> = {
        let tx = db
            .transaction(&["inventoryArmor"], TransactionMode::ReadOnly)
            .map_err(|e| ComputeError::LoadDataError)?;
        let inventory_armor = tx
            .store("inventory_armor")
            .map_err(|e| ComputeError::LoadDataError)?;
        let key_range = KeyRange::only(&(config.character_class as u32).into()).unwrap();
        let armor = inventory_armor.index("clazz").unwrap();
        let armor = armor
            .get_all(Some(&key_range), None, None, None)
            .await
            .map_err(|e| ComputeError::LoadDataError)?;
        armor
            .iter()
            .into_iter()
            .map(|(_, value)| serde_wasm_bindgen::from_value(value.clone()).unwrap())
            .collect()
    };

    let armor_info: HashMap<Hash, ArmorInformation> = armor
        .iter()
        .map(|i| (i.hash, Into::into(i.clone())))
        .collect();

    let armor: Vec<StrippedInventoryArmor> = armor.iter().map(|i| Into::into(i.clone())).collect();

    let armor: Vec<_> = armor
        .iter()
        .filter(|i| i.slot != ArmorSlot::ArmorSlotNone)
        .filter(|i| !config.disabled_items.contains(&i.item_instance_id))
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

    let armory: Armory = Into::into(armor);

    for armory in armory.chunk(n) {
        let worker = ArmorWorker::spawner()
            .callback(|msg: Output| {
                // todo: use output
                // rebuild SanitisedItemResult from ItemResult
                // reconcile threads
            })
            .spawn("/assets/worker.js");

        worker.send(Input {
            config: config.clone(),
            armory,
            n,
        })
    }

    Ok(())
}
