use serde::{Deserialize, Serialize};

use crate::data::prelude::*;
use crate::data_source::prelude::*;

#[derive(Deserialize, Serialize, Clone, PartialEq, Eq, Debug)]
pub struct JoinedUserOutput {
    pub user_query: UserQuery,
    pub action: String,
}


impl JoinedUserOutput {
   pub fn new(user_query: UserQuery, action: String) -> Self {
        Self {
            user_query,
            action
        }
    }
}

#[derive(Deserialize, Serialize)]
pub struct UserInfo {
    pub teller_loc: usize,
    pub user_time: f64,
    pub user_teller_pos: usize,
}

pub fn show_user_waiting_time(prev_time: f64, teller_id: String, queue: &mut QueueStruct, user_pos: usize) -> f64 {
    let teller = find_teller(teller_id);
    match teller {
        Ok(teller_data) => queue.set_up_timer(prev_time, teller_data.service_time as f64, user_pos),
        Err(_) => 0.0,
    }
}
