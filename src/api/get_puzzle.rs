use crate::domain::{is_door_allowed_to_be_open, Puzzle, PuzzleControl, PuzzleDescription};
use crate::store::Store;
use crate::utils::with_store;
use serde::{Deserialize, Serialize};
use std::convert::Infallible;
use warp::Filter;

#[derive(Deserialize)]
struct GetPuzzleOptions {
    session_id: String,
    number: u8,
}

#[derive(Serialize)]
#[serde(tag = "type")]
enum GetPuzzleResponse {
    Success { puzzle: Puzzle },
    Error { error: String },
}

async fn handle_get_puzzle(
    options: GetPuzzleOptions,
    store: Store,
) -> Result<impl warp::Reply, Infallible> {
    log::info!(
        "get-puzzle request for for number {} and sessionId {}",
        options.number,
        options.session_id
    );

    if options.number > 24 {
        return Ok(warp::reply::json(&GetPuzzleResponse::Error {
            error: String::from(
                "Du hast einen Tag angefragt, der gar kein Türchen hat. Hier wird nicht gehackt. >:(",
            ),
        }));
    }

    if is_door_allowed_to_be_open(options.number) {
        return Ok(warp::reply::json(&GetPuzzleResponse::Error {
            error: String::from(
                "Dieses Türchen bleibt noch geschlossen. Sei nicht so neugierig. OwO",
            ),
        }));
    };

    store.open_door(options.session_id, options.number);

    let puzzle = match options.number {
        1 => Puzzle {
            description: PuzzleDescription::Image(String::from(
                "https://m.media-amazon.com/images/I/81en3yalOyL.jpg",
            )),
            controls: vec![
                PuzzleControl::ColorPicker(None),
                PuzzleControl::ColorPicker(None),
                PuzzleControl::ColorPicker(None),
                PuzzleControl::ColorPicker(None),
                PuzzleControl::ColorPicker(None),
                PuzzleControl::ColorPicker(None),
                PuzzleControl::ColorPicker(None),
                PuzzleControl::ColorPicker(None),
                PuzzleControl::ColorPicker(None),
            ],
        },
        2 => Puzzle {
            description: PuzzleDescription::Image(String::from("/assests/images/puzzle_2.png")),
            controls: vec![
                PuzzleControl::NumberPicker(0),
                PuzzleControl::NumberPicker(0),
            ],
        },
        _ => panic!("unreachable"),
    };

    Ok(warp::reply::json(&GetPuzzleResponse::Success { puzzle }))
}

pub fn get_puzzle(
    store: Store,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    warp::path!("api" / "get-puzzle")
        .and(warp::post())
        .and(warp::body::json::<GetPuzzleOptions>())
        .and(with_store(store))
        .and_then(handle_get_puzzle)
}
