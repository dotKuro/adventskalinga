use crate::domain::{generate_doors_for_new_session, Door};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

#[derive(Clone)]
pub struct Store {
    hash_map_mutex: Arc<Mutex<HashMap<String, Vec<Door>>>>,
}

impl Store {
    pub fn new() -> Self {
        Self {
            hash_map_mutex: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub fn list_doors(&self, session_id: String) -> Vec<Door> {
        let mut hash_map = self.hash_map_mutex.lock().unwrap();

        let doors_result = hash_map.get(&session_id);

        match doors_result {
            Some(doors) => (*doors).clone(),
            None => {
                let doors = generate_doors_for_new_session();
                hash_map.insert(session_id, doors.clone());
                doors
            }
        }
    }
}
