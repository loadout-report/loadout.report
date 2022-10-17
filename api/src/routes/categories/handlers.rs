use super::model::{CategoryCache, GetCategoriesOptions};

pub async fn get_categories(
    _: GetCategoriesOptions,
    cache: CategoryCache,
) -> Result<impl warp::Reply, warp::Rejection> {
    let categories = cache.lock().await;
    let categories: Vec<_> = categories.iter()
        .collect();
    Ok(warp::reply::json(&categories))
}
