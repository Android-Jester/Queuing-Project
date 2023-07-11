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
        if index > 1 && index < self.tellers_count() {
            self.tellers[index - 1].users.append(&mut self.tellers[index].users);

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

    fn sub_queue_setup(
        teller: &mut ServerQueue,
        user_queue_data: &mut UserQueuePos,
    )  {
        let sub_queue_position = teller.users.len() + 1;
        let service_period = find_teller(teller.teller.server_id.clone())
            .unwrap()
            .service_time as f64;


        let timer = match sub_queue_position {
            1 => service_period,
            2..=CUSTOMER_COUNT => {
                service_period
            }
            _ => {
                let remaining_time = teller.users.first().unwrap().startup_timer;
                (service_period * (sub_queue_position as f64 + 1.0)) + remaining_time
            },
        };

        user_queue_data.setup_sub(sub_queue_position, timer)
    }

    pub fn add_customer(&mut self, mut user: &mut UserQueuePos) -> Result<(), &str> {
        let service_location = user.service_location;
        let teller = &mut self.tellers[service_location];
        match teller.users.len() != usize::MAX {
            true => {
                Self::sub_queue_setup(teller, &mut user);
                info!("User: {:?}", user);
                teller.users.push(user.clone());
                Ok(())
            }
            false => Err("Unable to add customer"),
        }
    }
    pub fn remove_customer(&mut self, user: UserQueuePos) {
        let removed_user = self.tellers[user.service_location].users.remove(user.sub_queue_position);
        self.sub_queue_realign(removed_user.service_location, removed_user.sub_queue_position);
    }

    fn sub_queue_realign(&mut self, service_location: usize, old_sub_queue_position: usize) {
        //TODO: Change the sub_queue_position of all users after the removed user
        for (position, user) in self.tellers[service_location].users.iter_mut().enumerate() {
            Self::sub_queue_setup(&mut self.tellers[service_location], user);
            if user.sub_queue_position > old_sub_queue_position {
                user.sub_queue_position = position
            }
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
