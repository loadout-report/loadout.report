use std::sync::Arc;
use serde::Deserialize;
use tokio::sync::Mutex;
use data::api::manifest::model::{Hash, Item};

pub type ItemCache = Arc<Mutex<Vec<Item>>>;

#[derive(Deserialize, Clone)]
pub struct GetItemsOptions {
    pub rarity: Option<i32>,
    pub category: Option<Hash>
}
