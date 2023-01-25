use std::convert::Infallible;
use warp::Filter;
use d2_client::D2Api;
use data::api::model::profile::ExactSearchRequest;


pub fn players(
    client: D2Api
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    fetch_player(client.clone())
        .or(search(client.clone()))
        .or(fetch_loadout(client))
}

pub fn search(
    client: D2Api
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("players" / "search")
        .and(warp::post())
        .and(warp::body::json::<ExactSearchRequest>())
        .and(with_client(client))
        .and_then(super::handlers::search)
}

pub fn fetch_player(
    client: D2Api
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("players" / i32 / i64)
        .and(warp::get())
        .and(warp::query::<super::model::FetchPlayerOptions>())
        .and(with_client(client))
        .and_then(super::handlers::get_player)
}

pub fn fetch_loadout(
    client: D2Api
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("players" / i32 / i64 / "loadout")
        .and(warp::get())
        .and(warp::query::<super::model::FetchLoadoutOptions>())
        .and(with_client(client))
        .and_then(super::handlers::get_loadout)
}

fn with_client(
    client: D2Api
) -> impl Filter<Extract=(D2Api,), Error=Infallible> + Clone {
    warp::any().map(move || client.clone())
}
