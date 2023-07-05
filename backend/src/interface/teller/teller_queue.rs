use crate::prelude::*;
use log::info;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug)]
pub struct TellerQueueStruct {
    pub teller_position: usize,
}

#[derive(Default, Debug, Deserialize, Serialize, Clone)]
pub struct SubQueues {
    tellers: Vec<ServerQueue>,
}

impl SubQueues {
    pub fn tellers_count(&self) -> usize {
        self.tellers.len()
    }
    pub fn add_teller(&mut self, teller: TellerQueueQuery) -> Result<(), &str> {
        match self.tellers_count() < SERVER_COUNT {
            true => {
                self.tellers.push(ServerQueue::new(teller));
                info!("{:?}", self.tellers);
                Ok(())
            }
            false => Err("Server List full"),
        }
    }
    pub fn remove_teller(&mut self, index: usize) -> Result<ServerQueue, &str> {
        if index > 0 || index > self.tellers_count() {
            Ok(self.tellers.remove(index))
        } else {
            Err("Index not in line")
        }
    }
    pub fn search_teller(&self, station: usize) -> Result<&ServerQueue, &str> {
        if station < self.tellers.len() {
            Ok(&self.tellers[station])
        } else {
            Err("No Available Teller")
        }
    }

    fn setup_timer(
        &self,
        service_location: usize,
        national_id: String,
        server_remaining_time: f64,
    ) -> f64 {
        let teller_queue = &self.tellers[service_location];
        let sub_user_queue = &teller_queue.users;
        let user = sub_user_queue
            .iter()
            .find(|user| user.national_id == national_id)
            .unwrap();
        let sub_queue_pos = &user.sub_queue_position.unwrap();
        let service_period = find_teller(teller_queue.teller.server_id.clone())
            .unwrap()
            .service_time as f64;
        match sub_queue_pos {
            0 => 0.0,
            1..=CUSTOMER_COUNT => {
                (service_period * (*sub_queue_pos + 1) as f64) + server_remaining_time
            }
            _ => (service_period * *sub_queue_pos as f64) + server_remaining_time,
        }
    }

    pub fn add_customer(&mut self, mut user: UserQueuePos) -> Result<(), &str> {
        let service_location = user.service_location.unwrap();
        let startup_timer = self.setup_timer(service_location, user.national_id.clone(), 0.0);
        let teller = &mut self.tellers[service_location];
        match teller.users.len() != usize::MAX {
            true => {
                let sub_queue_position = teller.users.len() + 1;
                user.setup_sub(sub_queue_position, startup_timer);
                teller.users.push(user);
                Ok(())
            }
            false => Err("Unable to add customer"),
        }
    }
    pub fn remove_customer(&mut self, user: UserQueuePos) -> Result<(), &str> {
        match user.sub_queue_position {
            Some(index) => {
                let teller = &mut self.tellers[index];
                match teller.users.is_empty() {
                    false => {
                        teller.users.remove(index);
                        // self.reassign_tellers();
                        Ok(())
                    }
                    true => Err("Unable to add customer"),
                }
            }
            None => Err("No Teller Found"),
        }
    }
    pub fn show_users(&mut self, service_location: usize) -> Vec<UserQueuePos> {
        let teller = &mut self.tellers[service_location];
        let mut teller_view: Vec<UserQueuePos> = Vec::new();
        for user in teller.users.clone() {
            teller_view.push(user);
        }
        teller_view
    }
}
