use std::env::args;
use std::str::FromStr;
use anyhow::Context;
use ipnet::IpNet;
use reqwest::Client;
use log::info;

#[cfg(target_os = "windows")]
#[global_allocator]
static ALLOC: snmalloc_rs::SnMalloc = snmalloc_rs::SnMalloc;

#[cfg(not(target_os = "windows"))]
#[global_allocator]
static GLOBAL: jemallocator::Jemalloc = jemallocator::Jemalloc;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    env_logger::init();
    let range = args().nth(1).context("Missing IP range")?;
    let range = IpNet::from_str(&range).context("Invalid IP range")?;
    let host = range.hosts().next().context("IP range exhausted")?;

    info!("creating client");
    let client: Client = reqwest::ClientBuilder::new()
        .local_address(host.clone())
        .freebind(true)
        .build()
        .unwrap();

    info!("sending request to {:?}", host);
    let text = client.get("https://am.i.mullvad.net").send()
        .await?
        .text()
        .await?;

    println!("ip: {}", text);

    Ok(())
}
