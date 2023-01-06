use thiserror::Error;
use crate::destiny::config::Manifest;
use crate::destiny::Hash;
use crate::endpoints::ApiError;

#[derive(Error, Debug)]
pub enum DestinyEndpointError {

}

pub async fn get_destiny_manifest(client: &reqwest::Client) -> Result<Manifest, ApiError> {
    client.get("https://www.bungie.net/Platform/Destiny2/Manifest/")
        .send().await
        .map_err(ApiError::ReqwestError)?
        .json()
        .await
        .map_err(ApiError::JsonParseError)
}
