#![feature(result_option_inspect)]


use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::net::IpAddr;
use std::str::FromStr;

pub mod archive;
pub mod constants;
pub mod raw;

const PGCR_URL: &str = "https://bungie.net/Platform/Destiny2/Stats/PostGameCarnageReport/";
const IP_CHECK_URL: &str = "https://api.ipify.org";

#[derive(Clone, Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct BungieApiResponse {
    // pub response: Option<T>,
    pub error_code: i32,
    pub throttle_seconds: i32,
    pub error_status: String,
    pub message: String,
    pub message_data: HashMap<String, String>,
    pub detailed_error_trace: Option<String>,
}

pub async fn get_own_ip(client: reqwest::Client) -> Result<IpAddr, Box<dyn std::error::Error>> {
    let ip: String = client.get(IP_CHECK_URL).send().await?.text().await?;
    let ip: IpAddr = IpAddr::from_str(ip.as_str())?;
    Ok(ip)
}

pub async fn fetch_pcgr(
    client: reqwest::Client,
    id: u64,
) -> Result<BungieApiResponse, Box<dyn std::error::Error>> {
    let report: BungieApiResponse = client
        .get(format!("{}{}", PGCR_URL, id))
        .send()
        .await?
        .json()
        .await?;
    Ok(report)
}

#[cfg(test)]
mod tests {
    use crate::BungieApiResponse;
    use reqwest::header::{HeaderMap, HeaderValue};
    use tokio::join;
    use tokio_stream::StreamExt;

    #[tokio::test]
    async fn test() {}
}
