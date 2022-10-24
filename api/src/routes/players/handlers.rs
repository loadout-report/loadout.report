use log::warn;
use data::api::model::profile::Membership;
use d2_client::D2Api;
use crate::ApiError;
use crate::routes::players::model::{FetchLoadoutOptions, FetchPlayerOptions};

pub async fn get_player(
    membership_type: i32,
    membership_id: i64,
    query: FetchPlayerOptions,
    client: D2Api,
) -> Result<impl warp::Reply, warp::Rejection> {
    let membership = Membership::new(membership_type, membership_id);
    client.fetch_loadout(membership, query.cache.unwrap_or(true)).await
        .map(|loadout| warp::reply::json(&loadout))
        .map_err(|err| {
            warn!("got error loading profile: {}", err);
            warp::reject::custom(ApiError::Unknown)
        })
}

pub async fn get_loadout(
    membership_type: i32,
    membership_id: i64,
    query: FetchLoadoutOptions,
    client: D2Api,
) -> Result<impl warp::Reply, warp::Rejection> {
    let membership = Membership::new(membership_type, membership_id);
    client.fetch_loadout(membership, query.cache.unwrap_or(true)).await
        .map(|loadout| warp::reply::json(&loadout))
        .map_err(|err| {
            warn!("got error loading profile: {}", err);
            warp::reject::custom(ApiError::Unknown)
        })
}
