use crate::destiny::definitions::InventoryItemDefinition;
use crate::destiny::{Hash, ItemHash};
use crate::endpoints::destiny::definitions::DefinitionsError::HashParseError;
use std::collections::HashMap;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum DefinitionsError {
    #[error("Unknown error: {0}")]
    ReqwestError(reqwest::Error),
    #[error("Could not parse hash string: {0}")]
    HashParseError(String),
}

pub async fn get_inventory_item_definitions(
    client: &reqwest::Client,
    url: &str,
) -> Result<HashMap<ItemHash, InventoryItemDefinition>, DefinitionsError> {
    client
        .get(url)
        .send()
        .await
        .map_err(DefinitionsError::ReqwestError)?
        .json::<HashMap<String, InventoryItemDefinition>>()
        .await
        .map_err(DefinitionsError::ReqwestError)
        .and_then(|r| {
            let mut map = HashMap::new();
            for (key, value) in r {
                if let Ok(key) = key.parse() {
                    map.insert(ItemHash(Hash(key)), value);
                } else {
                    return Err(HashParseError(key));
                }
            }
            Ok(map)
        })
}

#[cfg(test)]
mod tests {

    #[tokio::test]
    #[ignore]
    async fn test_get_inventory_item_definitions() {
        let client = reqwest::Client::new();
        // todo: automate updating this url
        let url = format!(
            "{}/common/destiny2_content/json/en/DestinyInventoryItemDefinition-{}.json",
            crate::endpoints::PLATFORM,
            "ed55fd73-3627-4784-9026-96aae1a7b82f"
        );
        let definitions = super::get_inventory_item_definitions(&client, &url).await;
        if let Err(err) = definitions {
            panic!("error: {}", err);
        }
        assert!(definitions.is_ok());
    }
    
}
