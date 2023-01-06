use thiserror::Error;

pub mod auth;
pub mod destiny;

const PLATFORM: &str = "http://localhost:5655";

#[derive(Error, Debug)]
pub enum ApiError {
    #[error("Error in request: {0}")]
    ReqwestError(reqwest::Error),
    #[error("Error parsing JSON: {0}")]
    JsonParseError(reqwest::Error),
    #[error("Invalid response")]
    InvalidResponse,
}
