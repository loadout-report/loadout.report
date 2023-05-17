use std::sync::Arc;
use serde::Deserialize;
use tokio::sync::Mutex;

#[derive(Deserialize, Clone)]
pub struct GetCategoriesOptions {
}
