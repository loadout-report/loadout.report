use std::collections::HashMap;
use std::str::FromStr;
use chrono::{DateTime, NaiveDateTime, Utc};
use reqwest::header::IF_MODIFIED_SINCE;
use reqwest::{header, StatusCode};
use data::api::manifest::model;

const LAST_MODIFIED_FORMAT: &str = "%a, %d %b %Y %H:%M:%S GMT";

pub struct ManifestApi {
    pub(crate) client: super::D2Api,
}

impl ManifestApi {

    async fn fetch_item_definitions(&self) -> Result<Vec<model::Item>, Box<dyn std::error::Error>> {
        let url = format!("https://www.bungie.net{}", self.fetch_content_paths().await?.destiny_inventory_item_definition);
        let mut items: HashMap<String, model::InventoryItem> = reqwest::get(url).await?
            .json()
            .await?;
        let mut items: Vec<_> = items.values_mut()
            .filter(|i| i.item_category_hashes.is_none() || !i.item_category_hashes.as_ref().unwrap().contains(&3109687656))
            .map(|i| i.clone())
            .map(model::Item::from)
            .collect();
        items.sort_by_key(|i| i.label.clone());
        Ok(items)
    }

    async fn fetch_content_paths(&self) -> Result<data::api::manifest::Components, Box<dyn std::error::Error>> {
        let mut builder = self.client.client.get("https://www.bungie.net/Platform/Destiny2/Manifest/");
        if let Some(last_modified) = self.client.cache.manifest.metadata.lock().await.last_modified {
            builder = builder.header(IF_MODIFIED_SINCE, format!("{}", last_modified.format(LAST_MODIFIED_FORMAT)))
        }
        let request = builder.build()
            .unwrap();
        let response = self.client.client.execute(request).await;
        match response {
            Ok(response) => {
                if response.status() == StatusCode::NOT_MODIFIED {
                    // return cached result
                    log::debug!("using cached manifest");
                    let paths = self.client.cache.manifest.content_paths.lock().await.clone().unwrap();
                    return Ok(paths);
                }
                if !response.status().is_success() {
                    // log error
                    log::error!("unknown response error. trying cache or panicking: {}", response.status());
                    let guard = self.client.cache.manifest.content_paths.lock().await;
                    if let Some(cached) = guard.clone() {
                        log::debug!("using cached manifest");
                        return Ok(cached);
                    }
                    panic!("no cache available");
                }
                let last_modified = response.headers().get(header::LAST_MODIFIED)
                    .and_then(|h| h.to_str().ok())
                    .map(|s| NaiveDateTime::parse_from_str(s, LAST_MODIFIED_FORMAT))
                    .and_then(Result::ok)
                    .map(|d| DateTime::from_utc(d, Utc));
                let body: data::api::ApiResponse<data::api::manifest::Manifest> = response.json().await?;
                let components = Result::from(body)?.json_world_component_content_paths.en;
                let mut components_cache = self.client.cache.manifest.content_paths.lock().await;
                self.client.cache.manifest.metadata.lock().await.last_modified = last_modified;
                let components = components_cache.insert(components);
                Ok(components.clone())
            },
            Err(err) => {
                // log error
                log::error!("unknown unhandled error. trying cache: {}", err);
                let guard = self.client.cache.manifest.content_paths.lock().await;
                if let Some(cached) = guard.clone() {
                    log::debug!("using cached manifest");
                    return Ok(cached);
                }
                Err(err)?
            }
        }
    }

}

#[cfg(test)]
mod tests {
    use tokio_test::assert_ok;
    macro_rules! aw {
        ($e:expr) => {
            tokio_test::block_on($e)
        };
    }

    #[test]
    fn test_fetch_content_paths() {
        let client = crate::D2Api::new("");
        for _ in 1..2 {
            let manifest = tokio_test::block_on(client.manifest().fetch_content_paths());
            assert_ok!(manifest);
        }

    }

}
