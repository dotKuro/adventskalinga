use std::convert::Infallible;
use warp::Filter;

use crate::store::Store;
use crate::utils::with_store;

async fn handle_backup(store: Store) -> Result<impl warp::Reply, Infallible> {
    log::info!("backup request");

    store.backup();

    Ok(warp::reply())
}

pub fn backup(
    store: Store,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    warp::path!("api" / "backup")
        .and(warp::post())
        .and(with_store(store))
        .and_then(handle_backup)
}
