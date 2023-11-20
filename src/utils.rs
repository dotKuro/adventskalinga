use crate::store::Store;
use std::convert::Infallible;
use warp::Filter;

pub fn with_store(store: Store) -> impl Filter<Extract = (Store,), Error = Infallible> + Clone {
    warp::any().map(move || store.clone())
}
