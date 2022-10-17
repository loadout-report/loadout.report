use crate::routes::items::model::{ItemCache, GetItemsOptions};

pub async fn get_items(
    query: GetItemsOptions,
    cache: ItemCache,
) -> Result<impl warp::Reply, warp::Rejection> {
    let items = cache.lock().await;
    let items: Vec<_> = items.iter()
        .filter(|item| query.rarity.is_none() || item.rarity == query.rarity.unwrap())
        .filter(|item| query.category.is_none() || item.categories.contains(&query.category.unwrap()))
        .collect();
    Ok(warp::reply::json(&items))
}
