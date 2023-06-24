use crate::data::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug)]
pub struct TellerQueueStruct {
    pub teller_position: usize,
}

#[derive(Serialize, Deserialize)]
pub struct ServerTeller {
    pub name: String,
    pub account_number: String,
    pub action: String,
    pub national_id: String,
}

#[derive(Debug)]
pub struct TellersQueue {
    tellers: Vec<TellerQueueQuery>,
}

impl Default for TellersQueue {
    fn default() -> Self {
        Self {
            tellers: Vec::with_capacity(SERVER_COUNT),
        }
    }
}

impl TellersQueue {
    pub fn add_teller(&mut self, teller_id: TellerQueueQuery) {
        self.tellers.push(teller_id);
    }
    pub fn remove_teller(&mut self, index: usize) -> TellerQueueQuery {
        self.tellers.remove(index)
    }
    pub fn find_teller(&self, index: usize) -> Result<TellerQueueQuery, &str> {
        if index < self.tellers.len() {
            Ok(self.tellers[index].clone())
        } else {
            Err("No Available Teller")
        }
    }
}
