use serde::Deserialize;
use std::convert::Infallible;
use warp::Filter;

use crate::store::Store;
use crate::utils::with_store;

#[derive(Deserialize)]
struct ListDoorsOptions {
    session_id: String,
}

async fn handle_list_doors(
    options: ListDoorsOptions,
    store: Store,
) -> Result<impl warp::Reply, Infallible> {
    log::info!("list-doors request for sessionId {}", options.session_id);

    let doors = store.list_doors(options.session_id);

    Ok(warp::reply::json(&doors))
}

pub fn list_doors(store: Store) -> impl Filter<Extract = impl warp::Reply> + Clone {
    warp::path!("list-doors")
        .and(warp::post())
        .and(warp::body::json::<ListDoorsOptions>())
        .and(with_store(store))
        .and_then(handle_list_doors)
}
