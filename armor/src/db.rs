use rexie::*;
use data::api::manifest::model::Hash;
use wasm_bindgen::JsValue;
use log::warn;
use crate::model::{CharacterClass, InventoryArmor, ManifestArmor};
use crate::scheduler::ComputeError;

pub async fn build_database() -> Result<Rexie> {
    Rexie::builder("d2armorpicker-v2")
        .version(190)
        .add_object_store(
            ObjectStore::new("manifestArmor")
                .key_path("id")
                .auto_increment(true)
                .add_index(Index::new("hash", "hash"))
                .add_index(Index::new("isExotic", "isExotic")),
        )
        .add_object_store(
            ObjectStore::new("inventoryArmor")
                .key_path("id")
                .auto_increment(true)
                .add_index(Index::new("itemInstanceId", "itemInstanceId"))
                .add_index(Index::new("isExotic", "isExotic"))
                .add_index(Index::new("hash", "hash"))
                .add_index(Index::new("name", "name"))
                .add_index(Index::new("masterworked", "masterworked"))
                .add_index(Index::new("clazz", "clazz"))
                .add_index(Index::new("slot", "slot"))
        ).build().await
}

pub async fn get_manifest_armor(db: &Rexie, exotic: &Hash) -> std::result::Result<ManifestArmor, ComputeError> {
    let tx = db
        .transaction(&["manifestArmor"], TransactionMode::ReadOnly)
        .map_err(ComputeError::LoadDataError)?;
    let manifest_armor = tx.store("manifestArmor").unwrap();
    let armor = manifest_armor
        .index("hash")
        .unwrap()
        .get(&JsValue::from(*exotic as i32))
        .await
        .map_err(ComputeError::LoadDataError)?;
    let armor: ManifestArmor =
        serde_wasm_bindgen::from_value(armor).map_err(ComputeError::DataParseError)?;
    tx.done().await.map_err(ComputeError::LoadDataError)?;
    Ok(armor)
}

pub async fn get_inventory_armor_for_class(db: Rexie, class: &CharacterClass) -> std::result::Result<Vec<InventoryArmor>, ComputeError> {
    let tx = db
        .transaction(&["inventoryArmor"], TransactionMode::ReadOnly)
        .map_err(ComputeError::LoadDataError)?;
    let inventory_armor = tx
        .store("inventoryArmor")
        .map_err(ComputeError::LoadDataError)?;
    let key_range = KeyRange::only(&(*class as i32).into()).unwrap();
    let armor = inventory_armor.index("clazz").unwrap();
    let armor = armor
        .get_all(Some(&key_range), None, None, None)
        .await
        .map_err(ComputeError::LoadDataError)?;
    let armor = armor
        .iter()
        .into_iter()
        .flat_map(|(_, value)| serde_wasm_bindgen::from_value(value.clone()).inspect_err(|err| warn!("could not parse armor, {}", err.to_string())))
        .collect();
    Ok(armor)
}

