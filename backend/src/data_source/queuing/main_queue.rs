use log::info;

use crate::prelude::*;
#[derive(Debug)]
pub struct QueueStruct {
    pub queue: Vec<UserQueuePos>,
}

impl Default for QueueStruct {
    fn default() -> Self {
        QueueStruct {
            queue: Vec::with_capacity(CUSTOMER_COUNT),
        }
    }
}

impl QueueStruct {
    /*Main Queue Events*/

    


    pub fn add_user<'a>(
        &'a mut self,
        user: JoinedUserOutput,
        servers: &'a mut TellersQueue,
    ) -> Result<UserQueuePos, &str> {
        match self.queue.len() < CUSTOMER_COUNT {
            true => {
                let pos: usize = self.queue.len() % servers.tellers_num();
                match servers.add_customer(pos, user.clone()) {
                    Ok((teller_pos, user_pos)) => match servers.search_teller(teller_pos) {
                        Ok(server) => {
                            let timer = show_user_waiting_time(
                                0.0,
                                server.teller.server_id.clone(),
                                self,
                                user_pos,
                            );
                            let user_pos: UserQueuePos = UserQueuePos::new(
                                user.user_query.name,
                                user.user_query.national_id.clone(),
                                user.action,
                                self.queue.len() + 1,
                                user_pos,
                                teller_pos,
                                timer,
                            );
                            
                            self.queue.push(user_pos.clone());
                            info!("Queue: {:?}", self.queue);
                            Ok(user_pos)
                        }
                        Err(d) => Err(d),
                    },
                    Err(_) => Err("Unable to assign user to teller"),
                }
            }
            false => Err("Queue Length exceeds Maximum"),
        }
    }
    pub fn remove_user(
        &mut self,
        user_queue_pos: usize,
        servers: &mut TellersQueue,
    ) -> Result<(), &str> {
        match user_queue_pos < CUSTOMER_COUNT {
            true => {
                let user: UserQueuePos = self.queue.remove(user_queue_pos);
                match servers.remove_customer(user) {
                    Ok(_) => {
                        for (user_pos, user_data) in self.queue.iter_mut().enumerate() {
                             if let Ok(user_query) = find_user(user_data.national_id.clone()) {
                                    user_data.change_queue_pos(user_pos);
                                    user_data.change_assigned_teller(user_pos % servers.tellers_num());
                                    let _ = servers.add_customer(
                                        user_pos % servers.tellers_num(),
                                        JoinedUserOutput::new(user_query, user_data.action.clone())
                                    );
                            }; 
                        }
                        Ok(())
                    }
                    Err(_) => Err("Unable to remove user to teller"),
                }
            }
            false => Err("Too Many Users"),
        }
    }
    /*Timer Events*/
    pub fn set_up_timer(
        &mut self,
        prev_remaining_time: f64,
        service_period: f64,
        server_index: usize,
    ) -> f64 {
        let mut timer: f64 = 0.0;
        if server_index > 2 {
            timer = (service_period * server_index as f64) + prev_remaining_time;
        } else if server_index <= 2 {
            timer = prev_remaining_time;
        }
        timer
    }
 

    /*Live Changes*/
    pub fn queue_change(&mut self) {
        for (pos, user) in self.queue.iter_mut().enumerate() {
            user.change_queue_pos(pos + 1);
        }
    }
}
