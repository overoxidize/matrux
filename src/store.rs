use std::collections::HashMap;

use crate::room::Room;

pub trait Storer<T: Clone> {
    fn save_filter_id(&mut self, user_id: String, filter_id: String);
    fn load_filter_id(self, user_id: String) -> String;
    fn save_next_batch(&mut self, user_id: String, next_batch_token: String);
    fn load_next_batch(self, user_id: String) -> String;
    fn save_room(&mut self, room: Room<T>);
    fn load_room(self, room_id: String) -> Room<T>;
}

struct InMemoryStore<T: Clone> {
    filters: HashMap<String, String>,
    next_batch: HashMap<String, String>,
    rooms: HashMap<String, Room<T>>,
}

impl<T: Clone> Storer<T> for InMemoryStore<T> {
    fn save_filter_id(&mut self, user_id: String, filter_id: String) {
        self.filters.insert(user_id, filter_id);
    }

    fn load_filter_id(self, user_id: String) -> String {
        self.filters.get(&user_id).unwrap().to_owned()
    }

    fn save_next_batch(&mut self, user_id: String, next_batch_token: String) {
        self.next_batch.insert(user_id, next_batch_token);
    }

    fn load_next_batch(self, user_id: String) -> String {
        self.next_batch.get(&user_id).unwrap().to_owned()
    }

    fn save_room(&mut self, room: Room<T>) {
        let room_id = &room.id;
        self.rooms.insert(room_id.to_string(), room);
    }

    fn load_room(self, room_id: String) -> Room<T> {
        self.rooms.get(&room_id).unwrap().to_owned().clone()
    }
}

impl<T: Clone> InMemoryStore<T> {
    pub fn new() -> Self {
        Self {
            filters: HashMap::new(),
            next_batch: HashMap::new(),
            rooms: HashMap::new(),
        }
    }
}
