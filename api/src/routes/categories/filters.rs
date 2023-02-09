use super::model::CategoryCache;
use warp::Filter;

pub fn categories(
    items: CategoryCache,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    categories_fetch(items)
}

fn categories_fetch(
    categories: CategoryCache,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("categories")
        .and(warp::get())
        .and(warp::query::<super::model::GetCategoriesOptions>())
        .and(with_categories(categories))
        .and(warp::path::end())
        .and_then(super::handlers::get_categories)
}

fn with_categories(
    items: CategoryCache,
) -> impl Filter<Extract = (CategoryCache,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || items.clone())
}
