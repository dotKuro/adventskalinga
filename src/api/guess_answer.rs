use crate::domain::is_door_allowed_to_be_open;
use crate::domain::{Color, PuzzleControl};
use crate::store::Store;
use crate::utils::{tv_shows, with_store};
use serde::{Deserialize, Serialize};
use std::convert::Infallible;
use warp::Filter;

#[derive(Deserialize)]
struct GuessAnswerOptions {
    session_id: String,
    number: u8,
    answer: Vec<PuzzleControl>,
}

#[derive(Serialize)]
#[serde(tag = "type", content = "value")]
enum GuessAnswerResponse {
    Correct { code: String },
    Wrong,
    Error { error: String },
}

async fn handle_get_puzzle(
    options: GuessAnswerOptions,
    store: Store,
) -> Result<impl warp::Reply, Infallible> {
    log::info!(
        "guess-answer request for for number {} and sessionId {}",
        options.number,
        options.session_id
    );

    if options.number > 24 {
        return Ok(warp::reply::json(&GuessAnswerResponse::Error {
            error: String::from(
                "Du hast einen Tag angefragt, der gar kein Türchen hat. Hier wird nicht gehackt. >:(",
            ),
        }));
    }

    if is_door_allowed_to_be_open(options.number) {
        return Ok(warp::reply::json(&GuessAnswerResponse::Error {
            error: String::from(
                "Dieses Türchen bleibt noch geschlossen. Sei nicht so neugierig. OwO",
            ),
        }));
    };

    if !store.is_door_open(options.session_id, options.number) {
        return Ok(warp::reply::json(&GuessAnswerResponse::Error {
            error: String::from(
                "Eine Frage beantworten für ein geschlossenes Türchen ist kinda sus. 🔴",
            ),
        }));
    }

    let (answer, code) = match options.number {
        1 => (
            vec![
                PuzzleControl::ColorPicker(Some(Color::Green)),
                PuzzleControl::ColorPicker(Some(Color::Orange)),
                PuzzleControl::ColorPicker(Some(Color::Blue)),
                PuzzleControl::ColorPicker(Some(Color::Red)),
                PuzzleControl::ColorPicker(Some(Color::Green)),
                PuzzleControl::ColorPicker(Some(Color::Orange)),
                PuzzleControl::ColorPicker(Some(Color::Blue)),
                PuzzleControl::ColorPicker(Some(Color::Red)),
                PuzzleControl::ColorPicker(Some(Color::Green)),
            ],
            "3726",
        ),
        2 => (
            vec![
                PuzzleControl::NumberPicker(101),
                PuzzleControl::NumberPicker(7),
            ],
            "0608",
        ),
        3 => (
            vec![
                PuzzleControl::NumberPicker(205),
                PuzzleControl::NumberPicker(6),
            ],
            "8941",
        ),
        4 => (
            vec![PuzzleControl::TextSelection(
                String::from("Spongebob Squarepants"),
                tv_shows(),
            )],
            "7203",
        ),
        5 => (vec![PuzzleControl::NumberPicker(52)], "5274"),
        6 => (
            vec![
                PuzzleControl::InvisibleText(String::from("You did well meow")),
                PuzzleControl::TextInput(String::from("You did well meow")),
            ],
            "4348",
        ),
        7 => (
            vec![PuzzleControl::TextSelection(
                String::from("One Piece"),
                tv_shows(),
            )],
            "7334",
        ),
        8 => (
            vec![
                PuzzleControl::NumberPicker(8),
                PuzzleControl::NumberPicker(15),
                PuzzleControl::NumberPicker(25),
                PuzzleControl::NumberPicker(25),
            ],
            "2316",
        ),
        9 => (vec![PuzzleControl::NumberPicker(5)], "7571"),
        10 => (
            vec![PuzzleControl::TextInput(String::from("pivot"))],
            "9472",
        ),
        11 => (
            vec![PuzzleControl::TextInput(String::from("Rainbow Road"))],
            "2446",
        ),
        12 => (
            vec![PuzzleControl::TextSelection(
                String::from("Gray's Anatomy"),
                tv_shows(),
            )],
            "9218",
        ),
        13 => (
            vec![PuzzleControl::TextInput(String::from(
                "You did well meeow!",
            ))],
            "8627",
        ),
        14 => (vec![PuzzleControl::NumberPicker(15)], "9132"),
        15 => (
            vec![PuzzleControl::TextSelection(
                String::from("Gilmore Girls"),
                tv_shows(),
            )],
            "2041",
        ),
        16 => (
            vec![PuzzleControl::TextInput(String::from(
                "You did well meeeow!",
            ))],
            "8725",
        ),
        17 => (vec![PuzzleControl::TextInput(String::from("F"))], "9790"),
        18 => (
            vec![PuzzleControl::TextInput(String::from("BieneMajazuHause!"))],
            "7280",
        ),
        19 => (vec![PuzzleControl::NumberPicker(18)], "9306"),
        20 => (
            vec![PuzzleControl::TextInput(String::from("Pikachu"))],
            "6209",
        ),
        21 => (
            vec![PuzzleControl::TextSelection(
                String::from("How I met your Mother"),
                tv_shows(),
            )],
            "6886",
        ),
        22 => (
            vec![PuzzleControl::TextInput(String::from("Wintersonnenwende"))],
            "3478",
        ),
        23 => (
            vec![
                PuzzleControl::TextInput(String::from("011011")),
                PuzzleControl::TextInput(String::from("000100")),
            ],
            "0101",
        ),
        24 => (
            vec![PuzzleControl::TextInput(String::from(
                "Ich hoffe dir hat der Kalendar gefallen <3",
            ))],
            "4838",
        ),
        _ => panic!("unreachable"),
    };

    let response = {
        if answer == options.answer {
            GuessAnswerResponse::Correct {
                code: String::from(code),
            }
        } else {
            GuessAnswerResponse::Wrong
        }
    };
    Ok(warp::reply::json(&response))
}

pub fn guess_answer(
    store: Store,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    warp::path!("api" / "guess-answer")
        .and(warp::post())
        .and(warp::body::json::<GuessAnswerOptions>())
        .and(with_store(store))
        .and_then(handle_get_puzzle)
}
