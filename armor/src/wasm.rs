use std::collections::HashMap;
use futures::future::OptionFuture;
use num_traits::FromPrimitive;
use wasm_bindgen::JsValue;
use crate::model::{WorkerConfig, InventoryArmor, ManifestArmor};
use crate::scheduler::ComputeError;
use crate::db::{build_database, get_inventory_armor_for_class, get_manifest_armor};

#[cfg(feature = "wasm")]
use wasm_bindgen::prelude::*;

#[cfg(feature = "wasm-parallel")]
pub use wasm_bindgen_rayon::init_thread_pool;

#[cfg(feature = "wasm")]
#[wasm_bindgen]
pub fn init_log() {
    wasm_logger::init(wasm_logger::Config::default());
}

#[cfg(feature = "wasm")]
#[wasm_bindgen]
pub async fn compute_results(config: JsValue) -> Result<JsValue, ComputeError> {
    let mut config: WorkerConfig = serde_wasm_bindgen::from_value(config).map_err(ComputeError::ConfigParseError)?;
    config.armor_perks = coerce_string_hashmap(config.armor_perks_.clone());
    config.minimum_stat_tiers = coerce_string_hashmap(config.minimum_stat_tiers_.clone());
    config.maximum_mod_slots = coerce_string_hashmap(config.maximum_mod_slots_.clone());
    config.armor_affinities = coerce_string_hashmap(config.armor_affinities_.clone());

    let db = build_database().await.map_err(ComputeError::LoadDataError)?;

    let selected_exotic = match Option::from(config.selected_exotic) {
        Some(exotic) => Some(get_manifest_armor(&db, &exotic).await?),
        None => None,
    };

    let armor: Vec<InventoryArmor> = get_inventory_armor_for_class(db, &config.character_class).await?;

    crate::scheduler::compute_results(config, armor, &selected_exotic)
        .await
        .map(|results| serde_wasm_bindgen::to_value(&results).unwrap())
}

fn coerce_string_hashmap<K, V>(data: HashMap<String, V>) -> HashMap<K, V>
    where K: FromPrimitive + std::hash::Hash + Eq,
{
    data
        .into_iter()
        .filter_map(|(key, value)| {
            let key = K::from_i64(key.parse::<i64>().unwrap());
            key.map(|key| (key, value))
        })
        .collect()
}
