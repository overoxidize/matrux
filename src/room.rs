use std::collections::HashMap;

use crate::events::Event;

#[derive(Clone)]
pub struct Room<T: Clone> {
    pub id: String,
    state: HashMap<String, HashMap<String, Event<T>>>,
}

struct PublicRoom {
    canonical_alias: String,
    name: String,
    world_readable: bool,
    topic: String,
    num_joined_members: u32,
    avatar_url: String,
    room_id: String,
    guest_can_join: bool,
    aliases: Vec<String>,
}

impl<T: 'static + Clone> Room<T>
where
    String: for<'a> From<&'a T>,
{
    pub fn new(id: String) -> Self {
        let state = HashMap::new();
        Self { id, state }
    }

    fn update_state(&mut self, mut event: Event<T>) {
        let exists = self.state.contains_key(&event.etype);

        if !exists {
            self.state.insert(event.etype.to_string(), HashMap::new());
        }

        let inner_map = self.state.entry(event.etype.clone()).and_modify(|value| {
            let _event = event.clone();
            value.entry(event.state_key.clone()).and_modify(|inner| {
                let mut inner_val = inner;
                inner_val = &mut event;
            });
        });
    }

    fn get_state_event(&self, event_type: String, state_key: String) -> Option<&Event<T>> {
        let mut exists = false;

        let state_event_map = self
            .state
            .get(&event_type)
            .expect("event type should exist in state");

        state_event_map.get(&state_key)
    }

    fn get_membership_state(&self, user_id: String) -> String {
        let state = "leave".to_string();
        let event_type = "m.room.member".to_string();

        let event = self.get_state_event(event_type, user_id);

        match event {
            Some(event) => {
                let membership_state = event.content.get("membership").unwrap();

                String::from(membership_state)
            }

            None => state,
        }
    }
}
