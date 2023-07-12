use crate::prelude::*;
use log::info;
use serde::{Deserialize, Serialize};




#[derive(Default, Debug, Deserialize, Serialize, Clone)]
pub struct SubQueues {
    tellers: Vec<ServerQueue>,
}

impl SubQueues {
    pub fn teller_count(&self) -> usize {
        self.tellers.len()
    }
    pub fn teller_add(&mut self, teller: TellerQueueQuery) -> Result<(), &str> {
        match self.teller_count() < SERVER_COUNT {
            true => {
                self.tellers.push(ServerQueue::new(teller));
                info!("{:?}", self.tellers);
                Ok(())
            }
            false => Err("Server List full"),
        }
    }
    pub fn teller_remove(&mut self, index: usize) -> Result<ServerQueue, &str> {
        if index > 1 && index < self.teller_count() {
            // self.tellers[index - 1].users.append(&mut self.tellers[index].users);

            Ok(self.tellers.remove(index))
        } else {
            Err("Index not in line")
        }
    }
    pub fn teller_search(&self, station: usize) -> Result<&ServerQueue, &str> {
        if station < self.tellers.len() {
            Ok(&self.tellers[station])
        } else {
            Err("No Available Teller")
        }
    }
    pub fn teller_show_queue(&mut self, service_location: usize) -> Vec<UserQueuePos> {
        let teller = &mut self.tellers[service_location];
        let mut teller_view: Vec<UserQueuePos> = Vec::new();
        for user in teller.users.clone() {
            teller_view.push(user);
        }
        teller_view
    }
    pub fn teller_queue_hold(&mut self, teller_position: usize) -> TellerState {
        let teller_form = &mut self.tellers[teller_position];
        if teller_form.users.is_empty() {
            self.teller_remove(teller_position).expect("Unable to remove teller");
            TellerState::InActive
        } else  {
            TellerState::PendingRelease
        }
    }
}


impl SubQueues {
    fn customer_sub_queue_setup(
        teller: &ServerQueue,
        user_queue_data: &mut UserQueuePos,
        service_location: usize
    )  {
        // let teller = &self.tellers[service_location];
        let sub_queue_position = teller.users.len() + 1;



        let timer = match sub_queue_position {
            1 => 0.0,
            2..=CUSTOMER_COUNT => {
                let remaining_time = teller.users.first().unwrap().startup_timer;
                (teller.teller.service_time * (sub_queue_position as f64 + 1.0)) + remaining_time
            }
            _ => {
                teller.teller.service_time
            }
        };

        user_queue_data.setup_sub(sub_queue_position, service_location, timer)
    }
    fn sub_queue_realign(teller_data: &mut ServerQueue, old_sub_queue_position: usize, remaining_time: f64) {

        // self.tellers[service_location].users.iter_mut()
        // let teller_data = &mut self.tellers[service_location];
        //TODO: Change the sub_queue_position of all users after the removed user
        for (position, user) in teller_data.users.iter_mut().enumerate() {

            if user.sub_queue_position > old_sub_queue_position {
                user.startup_timer = (teller_data.teller.service_time * (position as f64 + 1.0)) + remaining_time;
                user.sub_queue_position = position;
            }
        }
    }


    pub fn customer_add(&mut self, user: &mut UserQueuePos) -> Result<(), &str> {
        let service_loc = user.position % self.teller_count();
        let teller = &mut self.tellers[user.service_location];
        match teller.teller.teller_state {
            TellerState::Active => {
                Self::customer_sub_queue_setup(teller, user, service_loc);
                match teller.users.len() != usize::MAX {
                    true => {

                        info!("User: {:?}", user);
                        teller.users.push(user.clone());
                        Ok(())
                    }
                    false => Err("Unable to add customer"),
                }
            }
            TellerState::InActive => {
                let service_location = user.position % self.teller_count() + 1;
                let teller = &mut self.tellers[service_location];
                let teller_state = &mut teller.teller.teller_state;
                if *teller_state == TellerState::Active {
                    Self::customer_sub_queue_setup(teller,user, service_location);
                    match teller.users.len() != usize::MAX {
                        true => {

                            info!("User: {:?}", user);
                            teller.users.push(user.clone());
                            Ok(())
                        }
                        false => Err("Unable to add customer"),
                    }
                }
                else {
                    Err("Cannot add user")
                }
            }
            TellerState::PendingRelease => {
                let service_location = user.position % self.teller_count() + 1;
                let teller = &mut self.tellers[service_location];
                let teller_state = &mut teller.teller.teller_state;
                if *teller_state == TellerState::Active {
                    Self::customer_sub_queue_setup(teller,user, service_location);
                    match teller.users.len() != usize::MAX {
                        true => {

                            info!("User: {:?}", user);
                            teller.users.push(user.clone());
                            Ok(())
                        }
                        false => Err("Unable to add customer"),
                    }
                }
                else {
                    Err("Cannot add user")
                }
            }
        }

    }
    pub fn customer_remove(&mut self, user: UserQueuePos) -> UserQueuePos {
        let mut teller = &mut self.tellers[user.service_location];
        let remaining_time = teller.users[0].startup_timer;
        let removed_user = teller.users.remove(user.sub_queue_position);
        Self::sub_queue_realign(&mut teller, user.service_location,  remaining_time);
        removed_user
    }

}
