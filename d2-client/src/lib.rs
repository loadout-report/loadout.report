extern crate core;

use data::api::{ApiResponse, model::profile::{Membership, ProfileStruct}};
use crate::cache::Cache;

pub mod cache;
pub mod manifest;

#[derive(Clone)]
pub struct D2Api {
    client: reqwest::Client,
    api_key: String,
    cache: Cache,
}

fn create_reqwest_client() -> reqwest::Client {
    reqwest::ClientBuilder::new()
        .cookie_store(true)
        .build()
        .unwrap()
}

impl D2Api {
    pub fn new(api_key: &str) -> Self {
        Self {
            client: create_reqwest_client(),
            api_key: String::from(api_key),
            cache: Cache::new(),
        }
    }
    
    pub fn players() {
        
    }
    
    pub fn manifest(&self) -> manifest::ManifestApi {
        manifest::ManifestApi { client: self.clone() }
    }

    pub fn load_manifest(&self) -> Result<(), Box<dyn std::error::Error>> {
        todo!()
    }

    pub fn fetch_manifest(&self) {
        todo!()
    }

    pub async fn fetch_player(&self, membership: Membership, use_cache: bool) -> Result<ProfileStruct, Box<dyn std::error::Error>> {
        let mut cache = self.cache.profile.lock().await;
        if use_cache {
            if let Some(cached) = cache.get(&membership) {
                return Ok(cached.clone());
            }
        }
        let url = format!("https://bungie.net/Platform/Destiny2/{}/Profile/{}/?components=100,200,205,300,302,304", membership.0, membership.1);
        let request = self.client.get(url)
            .header("x-api-key", self.api_key.as_str())
            .build()?;
        let response: ApiResponse<ProfileStruct> = self.client.execute(request).await?.json().await?;
        let data = Result::from(response)?;
        let _ = cache.insert(membership, data.clone());
        Ok(data)
    }

    pub async fn fetch_loadout(&self, membership: Membership, use_cache: bool) -> Result<ProfileStruct, Box<dyn std::error::Error>> {
        let profile = self.fetch_player(membership, use_cache).await?;
        todo!();
        Ok(profile)
    }

}


