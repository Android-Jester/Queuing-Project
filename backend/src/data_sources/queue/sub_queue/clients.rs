use crate::prelude::*;

impl SubQueues {
    pub fn customer_sub_queue_setup(servers: SubQueues, client: &mut ClientQueueData) {
        let server = &servers.tellers[client.server_location as usize];
        let sub_queue = &server.users;
        let sub_queue_position = sub_queue.len() as i32;
        let timer = match sub_queue_position {
            0 => 0,
            1 => server.teller.service_time * (sub_queue_position),
            1..=CUSTOMER_COUNT => {
                let first_user = &sub_queue[1];
                let remaining_time = first_user.time_duration;
                (server.teller.service_time  * (sub_queue_position)) + remaining_time
            }
            _ => server.teller.service_time,
        };
        client.setup_sub(sub_queue_position, timer);
    }
    
    fn sub_queue_realign(&mut self, old_sub_queue_position: i32, server_loc: i32) {
        let teller_info = &mut self.tellers[server_loc as usize];
        let teller_queue = &mut teller_info.users;
        let startup_time = teller_queue[1].clone().time_duration;
        //TODO: Change the sub_queue_position of all users after the removed user
        for (position, user) in teller_queue.iter_mut().enumerate() {
            let mut user = user;
            if user.sub_queue_position > old_sub_queue_position {
                let remaining_time = startup_time;
                let timer =
                    (teller_info.teller.service_time * (position as i32 + 1)) + remaining_time;
                user.time_duration = timer;
                user.sub_queue_position = position as i32;
            }
        }
    }
    pub fn customer_add(&mut self, mut user: ClientQueueData) -> Result<ClientQueueData, String> {
        let queue_clone = self.clone();
        let teller = &mut self.tellers[user.server_location as usize];
        match teller.teller.active {
            true => {
                Self::customer_sub_queue_setup(queue_clone, &mut user);
                teller.users.push(user.clone());
                Ok(user)
            }
            false => {
                user.server_location = user.server_location + 1;
                let res = loop {
                
                let teller = &mut self.tellers[user.server_location as usize];
                let teller_state = &mut teller.teller.active;
                // FIXME: Check for available tellers and show the available
                    if user.server_location < SERVER_COUNT {
                        if *teller_state {
                            teller.users.push(user.clone());
                            info!("User ADDED: {:?}", user);
                            break Ok(user)
                        } else {
                            info!("ERROR: Cannot add user");
                            user.server_location += 1;
                            // Err("Cannot add user".to_string())
                        }
                    } else {
                        break Err("Unable to assign user".to_string())
                    }
                };
                res
            }
        }
    }
    //FIXME: Reassign users to queue
    pub fn customer_remove(&mut self, national_id: String, service_location: usize) -> usize {
        let found_user = self.search_user(national_id).unwrap();
        // let found_user = found_user;
        let user_queue = &mut self.tellers[service_location].users;
        let _ = user_queue.remove(found_user.sub_queue_position as usize);
        self.sub_queue_realign(found_user.sub_queue_position, found_user.server_location);
        found_user.server_location as usize
    }
    fn search_user(
        &mut self,
        national_id: String,
    ) -> Result<ClientQueueData, String> {
        ClientQueueData::find_user(national_id)
    }
}
