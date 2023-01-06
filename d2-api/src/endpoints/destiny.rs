pub mod definitions;

use crate::destiny::config::Manifest;
use crate::destiny::Hash;
use crate::endpoints::{ApiError, PLATFORM};
use crate::wrapper::ApiResponse;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum DestinyEndpointError {}

pub async fn get_destiny_manifest(client: &reqwest::Client) -> Result<Manifest, ApiError> {
    client
        .get(format!("{}/Platform/Destiny2/Manifest/", PLATFORM))
        .send()
        .await
        .map_err(ApiError::ReqwestError)?
        .json::<ApiResponse<Manifest>>()
        .await
        .map_err(ApiError::JsonParseError)
        .and_then(|r| r.response.ok_or(ApiError::InvalidResponse))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_get_destiny_manifest() {
        let client = reqwest::Client::new();
        let manifest = get_destiny_manifest(&client).await;
        if let Err(err) = manifest {
            panic!("error: {}", err);
        }
        assert!(manifest.is_ok());
    }
}
