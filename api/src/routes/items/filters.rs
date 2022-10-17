use warp::Filter;
use crate::routes::items::model::ItemCache;

pub fn items(
    items: ItemCache
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    items_fetch(items)
}

fn items_fetch(
    items: ItemCache
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("items")
        .and(warp::get())
        .and(warp::query::<super::model::GetItemsOptions>())
        .and(with_items(items))
        .and(warp::path::end())
        .and_then(super::handlers::get_items)
}

fn with_items(
    items: ItemCache
) -> impl Filter<Extract=(ItemCache,), Error=std::convert::Infallible> + Clone {
    warp::any().map(move || items.clone())
}
