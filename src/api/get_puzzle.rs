use crate::domain::{is_door_allowed_to_be_open, Puzzle, PuzzleControl, PuzzleDescription};
use crate::store::Store;
use crate::utils::{tv_shows, with_store};
use serde::{Deserialize, Serialize};
use std::convert::Infallible;
use std::vec;
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
        3 => Puzzle {
            description: PuzzleDescription::Text(String::from("🎛🥩️➡️🌡️️️🕒⬅️")),
            controls: vec![
                PuzzleControl::NumberPicker(0),
                PuzzleControl::NumberPicker(0),
            ],
        },
        4 => Puzzle {
            description: PuzzleDescription::Text(String::from("👺❌🦨💨👃✅")),
            controls: vec![PuzzleControl::TextSelection(String::new(), tv_shows())],
        },
        5 => Puzzle {
            description: PuzzleDescription::Text(String::from("5 12 24 36 ?")),
            controls: vec![PuzzleControl::NumberPicker(0)],
        },
        6 => Puzzle {
            description: PuzzleDescription::Text(String::from("Inspect me! 👀")),
            controls: vec![
                PuzzleControl::InvisibleText(String::from("You did well meow")),
                PuzzleControl::TextInput(String::new()),
            ],
        },
        7 => Puzzle {
            description: PuzzleDescription::Text(String::from("👩‍🦰🗡️🗡️🗡️🌀🌀👃🏽😈🍏")),
            controls: vec![PuzzleControl::TextSelection(String::new(), tv_shows())],
        },
        8 => Puzzle {
            description: PuzzleDescription::Image(String::from("/assests/images/puzzle_8.png")),
            controls: vec![
                PuzzleControl::NumberPicker(0),
                PuzzleControl::NumberPicker(0),
                PuzzleControl::NumberPicker(0),
                PuzzleControl::NumberPicker(0),
            ],
        },
        9 => Puzzle {
            description: PuzzleDescription::Text(String::from("1 0 1 -1 2 -3 ?")),
            controls: vec![PuzzleControl::NumberPicker(0)],
        },
        10 => Puzzle {
            description: PuzzleDescription::Text(String::from("🛼⭐️➡️🛼➖")),
            controls: vec![PuzzleControl::TextInput(String::new())],
        },
        11 => Puzzle {
            description: PuzzleDescription::Text(String::from("🏎️🍌🐢➡️🌈🛣️⬅️")),
            controls: vec![PuzzleControl::TextInput(String::new())],
        },
        12 => Puzzle {
            description: PuzzleDescription::Text(String::from("🇲🟨💭💤😊")),
            controls: vec![PuzzleControl::TextSelection(String::new(), tv_shows())],
        },
        13 => Puzzle {
            description: PuzzleDescription::Text(String::from("/assests/images/puzzle_13.png")),
            controls: vec![PuzzleControl::TextInput(String::new())],
        },
        14 => Puzzle {
            description: PuzzleDescription::Image(String::from("/assests/images/puzzle_14.png")),
            controls: vec![PuzzleControl::NumberPicker(0)],
        },
        15 => Puzzle {
            description: PuzzleDescription::Image(String::from("/assests/images/puzzle_15.png")),
            controls: vec![PuzzleControl::TextSelection(String::new(), tv_shows())],
        },
        16 => Puzzle {
            description: PuzzleDescription::Text(String::from("Iye nsn govv woooyg!")),
            controls: vec![PuzzleControl::TextInput(String::new())],
        },
        17 => Puzzle {
            description: PuzzleDescription::Text(String::from("M D M D ?")),
            controls: vec![PuzzleControl::TextInput(String::new())],
        },
        22 => Puzzle {
            description: PuzzleDescription::Text(String::from("heute")),
            controls: vec![PuzzleControl::TextInput(String::new())],
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
