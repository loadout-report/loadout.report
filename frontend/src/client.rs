use log::info;
use rustgie_types::{SingleComponentResponseOfDestinyProfileCollectiblesComponent, SingleComponentResponseOfDestinyProfileTransitoryComponent};
use rustgie_types::destiny::definitions::collectibles::DestinyCollectibleDefinition;
use rustgie_types::destiny::responses::DestinyProfileResponse;
use rustgie_types::user::UserInfoCard;
use serde_derive::{Serialize, Deserialize};

const API_BASE: &str = env!("API_BASE");

#[derive(Clone, Debug)]
pub struct Client {
    pub client: reqwest::Client,
}

impl PartialEq for Client {
    fn eq(&self, _: &Self) -> bool {
        true
    }
}

impl Client {
    pub fn new() -> Self {
        Self {
            client: reqwest::Client::new(),
        }
    }

    pub fn with_api_key(api_key: &str) -> Self {
        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert("X-API-Key", api_key.parse().unwrap());
        let client = reqwest::Client::builder()
            .default_headers(headers)
            .build()
            .unwrap();
        Self { client }
    }

    pub async fn search(&self, search_request: ExactSearchRequest) -> Result<Vec<UserInfoCard>, reqwest::Error> {
        info!("searching for user");
        self.client.post(format!("{}/players/search", API_BASE))
            .json(&search_request)
            .send()
            .await?
            .json::<Vec<UserInfoCard>>()
            .await
    }

    pub async fn get_collectibles(&self) -> Result<Vec<DestinyCollectibleDefinition>, reqwest::Error> {
        info!("getting collectibles");
        self.client.get(format!("{}/collectibles", API_BASE))
            .send()
            .await?
            .json::<Vec<DestinyCollectibleDefinition>>()
            .await
    }

    pub async fn get_profile_collectibles(&self, membership_type: i32, membership_id: i64) -> Result<SingleComponentResponseOfDestinyProfileCollectiblesComponent, reqwest::Error> {
        info!("getting profile");
        self.client.get(&format!("{}/players/{}/{}/collectibles", API_BASE, membership_type, membership_id))
            .send()
            .await?
            .json::<SingleComponentResponseOfDestinyProfileCollectiblesComponent>()
            .await
    }

    pub async fn get_profile_transitive_data(&self, membership_type: i32, membership_id: i64) -> Result<SingleComponentResponseOfDestinyProfileTransitoryComponent, reqwest::Error> {
        info!("getting profile");
        self.client.get(&format!("{}/players/{}/{}/transitive", API_BASE, membership_type, membership_id))
            .send()
            .await?
            .json::<SingleComponentResponseOfDestinyProfileTransitoryComponent>()
            .await
    }

    pub async fn get_profile(&self, membership_type: i32, membership_id: i64) -> Result<DestinyProfileResponse, reqwest::Error> {
        info!("getting profile");
        self.client.get(&format!("{}/players/{}/{}/profile", API_BASE, membership_type, membership_id))
            .send()
            .await?
            .json::<DestinyProfileResponse>()
            .await
    }

}

#[derive(Clone, PartialEq, Eq, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExactSearchRequest {
    #[serde(rename = "displayName")]
    pub name: String,
    #[serde(rename = "displayNameCode")]
    pub code: i16,
}

impl From<&str> for ExactSearchRequest {
    fn from(value: &str) -> Self {
        let mut split = value.split('#');
        let name = split.next().unwrap_or_default().to_string();
        let code = split.next().unwrap_or_default().to_string();
        let code = code.parse().unwrap_or_default();
        Self { name, code }
    }
}
