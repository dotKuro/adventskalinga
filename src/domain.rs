use rand::seq::SliceRandom;
use serde::Serialize;

#[derive(Clone, Serialize, Debug)]
pub struct Door {
    number: u8,
    open: bool,
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
