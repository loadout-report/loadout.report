use thiserror::Error;

pub mod auth;
pub mod destiny;

const PLATFORM: &str = "https://www.bungie.net";

#[derive(Error, Debug)]
pub enum ApiError {
    #[error("Error in request: {0}")]
    ReqwestError(reqwest::Error),
    #[error("Error parsing JSON: {0}")]
    JsonParseError(reqwest::Error),
    #[error("Invalid response")]
    InvalidResponse,
}
