extern crate core;

use rustgie::types::BungieMembershipType;
use rustgie::types::destiny::DestinyComponentType;
use rustgie::types::destiny::responses::DestinyProfileResponse;
use rustgie::types::user::UserInfoCard;
use data::api::{ApiResponse, ComponentType, model::profile::{Membership, ProfileStruct}};
use data::api::model::profile::{ExactSearchRequest, UserInfo};
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

    pub async fn fetch_player(&self, membership: Membership, use_cache: bool) -> Result<DestinyProfileResponse, Box<dyn std::error::Error>> {
        let mut cache = self.cache.profile.lock().await;
        if use_cache {
            if let Some(cached) = cache.get(&membership) {
                return Ok(cached.clone());
            }
        }
        let client = rustgie::RustgieClientBuilder::new().with_api_key(self.api_key.as_str())
            .build().unwrap();
        let response = client.destiny2_get_profile(membership.1, parse_membership_type(membership.0).unwrap(), Some(vec![
            DestinyComponentType::Profiles,
            DestinyComponentType::Characters,
            DestinyComponentType::CharacterEquipment,
            DestinyComponentType::ItemInstances,
            DestinyComponentType::ItemPerks,
            DestinyComponentType::ItemStats,
            DestinyComponentType::Collectibles,
            DestinyComponentType::Transitory,
        ]), None).await?;
        let _ = cache.insert(membership, response.clone());
        Ok(response)
    }


    pub async fn fetch_loadout(&self, membership: Membership, use_cache: bool) -> Result<DestinyProfileResponse, Box<dyn std::error::Error>> {
        let profile = self.fetch_player(membership, use_cache).await?;
        todo!();
        Ok(profile)
    }

    pub async fn search_player(&self, search_player_options: &ExactSearchRequest) -> Result<Vec<UserInfoCard>, Box<dyn std::error::Error>> {
        let request = self.client.post("http://localhost:58989/Platform/Destiny2/SearchDestinyPlayerByBungieName/-1/")
            .header("x-api-key", self.api_key.as_str())
            .json(search_player_options)
            .build()?;
        let response: ApiResponse<Vec<UserInfoCard>> = self.client.execute(request).await?.json().await?;
        let data = Result::from(response)?;
        Ok(data)
    }

}



fn parse_membership_type(t: i32) -> Option<BungieMembershipType> {
    match t {
        0 => Some(BungieMembershipType::None),
        1 => Some(BungieMembershipType::TigerXbox),
        2 => Some(BungieMembershipType::TigerPsn),
        3 => Some(BungieMembershipType::TigerSteam),
        4 => Some(BungieMembershipType::TigerBlizzard),
        5 => Some(BungieMembershipType::TigerStadia),
        10 => Some(BungieMembershipType::TigerDemon),
        254 => Some(BungieMembershipType::BungieNext),
        -1 => Some(BungieMembershipType::All),
        _ => None,
    }
}