use rexie::*;

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

