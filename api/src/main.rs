use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;

use log::info;
use serde_derive::{Deserialize, Serialize};
// use tokio::signal::{unix::signal, unix::SignalKind};
use tokio::sync::Mutex;
use tokio_graceful_shutdown::{SubsystemHandle, Toplevel};
use warp::http::StatusCode;
use warp::Filter;

use d2_client::D2Api;
use data::api::{
    manifest::model::{self, Hash},
    ApiResponse,
};

// Use Jemalloc only for musl-64 bits platforms
#[cfg(all(target_env = "musl", target_pointer_width = "64"))]
#[global_allocator]
static ALLOC: jemallocator::Jemalloc = jemallocator::Jemalloc;

#[derive(Deserialize, Serialize)]
struct GetItemsQuery {
    rarity: Option<i32>,
    category: Option<Hash>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    init_logger();

    info!("loading manifest");
    let items = fetch_items().await?;
    let items = Arc::new(Mutex::new(items));
    info!("loaded items");
    let categories = fetch_categories().await?;
    let categories = Arc::new(Mutex::new(categories));
    info!("loaded categories");
    info!("loaded manifest");

    let cors = warp::cors().allow_any_origin()
        .allow_header("x-api-key")
        .allow_header("content-type")
        .allow_methods(vec!["GET", "POST"]);
    let root = warp::get().and(warp::path::end()).map(|| "API root");

    let client = D2Api::new(get_api_key().as_str());
    let routes = root
        .or(api::routes::items(items))
        .or(api::routes::categories(categories))
        .or(api::routes::players(client).with(warp::log("players")))
        .with(cors)
        .recover(|err: warp::Rejection| async {
            if let Some(api::ApiError::Unknown) = err.find() {
                Ok(StatusCode::INTERNAL_SERVER_ERROR)
            } else {
                Err(err)
            }
        });

    let (_, server) =
        warp::serve(routes).bind_with_graceful_shutdown(([0, 0, 0, 0], 58080), async move {
            info!("waiting for shutdown signal");
            tokio::signal::ctrl_c()
                .await
                .expect("failed to listen to shutdown signal");
            info!("got shutdown signal");
        });
    match tokio::join!(tokio::task::spawn(server)).0 {
        Ok(()) => info!("served"),
        Err(e) => log::error!("ERROR: Thread join error {}", e),
    }

    info!("shutting down");
    Ok(())
}

fn init_logger() {
    if std::env::var_os("RUST_LOG").is_none() {
        // Set `RUST_LOG=todos=debug` to see debug logs,
        // this only shows access logs.
        std::env::set_var("RUST_LOG", "info");
    }
    env_logger::init();
}

async fn fetch_categories() -> Result<Vec<model::Category>, Box<dyn std::error::Error>> {
    let url = format!(
        "https://www.bungie.net{}",
        fetch_content_paths()
            .await?
            .destiny_item_category_definition
    );

    let mut items: HashMap<String, model::CategoryDefinition> =
        reqwest::get(url).await?.json().await?;
    let categories: Vec<_> = items
        .values_mut()
        .map(|i| i.clone())
        .map(model::Category::from)
        .collect();
    Ok(categories)
}

fn get_api_key() -> String {
    std::env::var("D2_API_KEY").unwrap()
}

async fn fetch_items() -> Result<Vec<model::Item>, Box<dyn std::error::Error>> {
    let url = format!(
        "https://www.bungie.net{}",
        fetch_content_paths()
            .await?
            .destiny_inventory_item_definition
    );
    let mut items: HashMap<String, model::InventoryItem> = reqwest::get(url).await?.json().await?;
    let mut items: Vec<_> = items
        .values_mut()
        .filter(|i| {
            i.item_category_hashes.is_none()
                || !i
                    .item_category_hashes
                    .as_ref()
                    .unwrap()
                    .contains(&3109687656)
        })
        .map(|i| i.clone())
        .map(model::Item::from)
        .collect();
    items.sort_by_key(|i| i.label.clone());
    Ok(items)
}

async fn fetch_content_paths() -> Result<data::api::manifest::Components, Box<dyn std::error::Error>>
{
    let client = reqwest::Client::new();
    let request = client
        .get("https://www.bungie.net/Platform/Destiny2/Manifest/")
        .header("X-API-Key", get_api_key())
        .build()
        .unwrap();
    let response = client.execute(request).await?;
    let body: ApiResponse<data::api::manifest::Manifest> = response.json().await?;
    Ok(Result::from(body)?.json_world_component_content_paths.en)
}
