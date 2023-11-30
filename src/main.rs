use advent_calendar_backend::api::{get_puzzle, guess_answer, list_doors};
use advent_calendar_backend::store::Store;
use std::env;
use warp::Filter;

#[tokio::main]
async fn main() {
    if env::var_os("RUST_LOG").is_none() {
        env::set_var("RUST_LOG", "info");
    }
    pretty_env_logger::init();

    advent_calendar_backend::domain::generate_doors_for_new_session();

    let store = Store::new();

    let api = list_doors(store.clone())
        .or(get_puzzle(store.clone()))
        .or(guess_answer(store.clone()));

    warp::serve(api).run(([0, 0, 0, 0], 3000)).await;
}
