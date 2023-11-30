use chrono::{DateTime, Local};
use rand::seq::SliceRandom;
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Debug)]
pub struct Door {
    pub number: u8,
    pub open: bool,
}

pub fn generate_doors_for_new_session() -> Vec<Door> {
    let mut rng = rand::thread_rng();

    let mut doors: Vec<Door> = (1u8..=24)
        .map(|number| Door {
            number,
            open: false,
        })
        .collect();
    doors.shuffle(&mut rng);

    doors
}

fn get_date_for_number(number: u8) -> DateTime<Local> {
    let date_string = format!("2023-12-{:0>2}T00:00:00-00:00", number);
    DateTime::parse_from_rfc3339(&date_string).unwrap().into()
}

pub fn is_door_allowed_to_be_open(number: u8) -> bool {
    let today = Local::now();
    today < get_date_for_number(number)
}

#[derive(Serialize)]
pub struct Puzzle {
    pub description: PuzzleDescription,
    pub controls: Vec<PuzzleControl>,
}

#[derive(Serialize)]
#[serde(tag = "type", content = "value")]
pub enum PuzzleDescription {
    Text(String),
    Image(String),
}

#[derive(Deserialize, Serialize, PartialEq, Debug)]
#[serde(tag = "type", content = "value")]
pub enum PuzzleControl {
    ColorPicker(Option<Color>),
    NumberPicker,
}

#[derive(Deserialize, Serialize, PartialEq, Debug)]
#[serde(tag = "type")]
pub enum Color {
    Red,
    Orange,
    Brown,
    Blue,
    Purple,
    Yellow,
    Green,
    White,
    Black,
}
