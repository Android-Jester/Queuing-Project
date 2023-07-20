use crate::prelude::*;

#[derive(Deserialize, Debug)]
pub struct UserInputData {
    pub national_id: String,
    pub activity: String,
}

#[derive(Default, Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct UserQueuePos {
    pub name: String,
    pub national_id: String,
    pub activity: String,
    pub position: usize,
    pub sub_queue_position: usize,
    pub service_location: usize,
    pub startup_timer: usize,
}

impl UserQueuePos {
    pub fn new(user_input: UserInputData, name: String, service_location: usize) -> Self {
        Self {
            name,
            position: 0,
            sub_queue_position: 0,
            service_location,
            startup_timer: 0,
            activity: user_input.activity,
            national_id: user_input.national_id,
        }
    }

    pub fn setup_main(&mut self, position: usize) {
        self.position = position;
    }

    pub fn setup_sub(&mut self, sub_queue_position: usize, startup_timer: usize) {
        self.sub_queue_position = sub_queue_position;
        self.startup_timer = startup_timer;
        warn!("AFTER SETUP: {:?}", self);
    }
}
