use std::sync::Arc;
use serde::Deserialize;
use tokio::sync::Mutex;
use data::api::manifest::model::{Category};

pub type CategoryCache = Arc<Mutex<Vec<Category>>>;

#[derive(Deserialize, Clone)]
pub struct GetCategoriesOptions {
}
