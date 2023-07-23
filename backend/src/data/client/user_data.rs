use crate::prelude::*;

#[derive(Serialize, Deserialize, Clone)]
pub struct ClientLoginData {
    pub account_number: String,
    pub password: String,
}

#[derive(Deserialize, Debug)]
pub struct ClientInputData {
    pub national_id: String,
    pub activity: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ClientDataQuery {
    pub name: String,
    pub account_number: String,
    pub national_id: String,
}

impl ClientDataQuery {
    pub fn new(name: String, account_number: String, national_id: String) -> Self {
        Self {
            name,
            account_number,
            national_id,
        }
    }
}

#[derive(Default, Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct ClientQueueData {
    pub name: String,
    pub national_id: String,
    pub activity: String,
    pub position: usize,
    pub sub_queue_position: usize,
    pub service_location: usize,
    pub startup_timer: usize,
}

impl ClientQueueData {
    pub fn new(user_input: ClientInputData, name: String, service_location: usize) -> Self {
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
    pub fn new_fill(user: ClientQueueData) -> Self {
        Self { ..user }
    }

    pub fn replace_fill(&mut self, user: ClientQueueData) {
        info!("CALLED REPLACED");
        *self = user;
        // self.activity = user.activity;
        // self.name = user.name;
        // self.position = user.position;
        // self.sub_queue_position = user.sub_queue_position;
        // self.startup_timer = user.startup_timer;
        // self.national_id = user.national_id;
    }

    pub fn setup_main(&mut self, position: usize, servers: &SubQueues) {
        self.position = position;
        SubQueues::customer_sub_queue_setup(servers, self);
    }

    pub fn setup_sub(&mut self, sub_queue_position: usize, startup_timer: usize) {
        self.sub_queue_position = sub_queue_position;
        self.startup_timer = startup_timer;
    }
}
