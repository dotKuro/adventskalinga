use advent_calendar_backend::api::list_doors;
use advent_calendar_backend::store::Store;
use std::env;

#[tokio::main]
async fn main() {
    if env::var_os("RUST_LOG").is_none() {
        env::set_var("RUST_LOG", "info");
    }
    pretty_env_logger::init();

    advent_calendar_backend::domain::generate_doors_for_new_session();

    let store = Store::new();

    let api = list_doors(store.clone());

    warp::serve(api).run(([127, 0, 0, 1], 3000)).await;
}
