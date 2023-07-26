use crate::prelude::*;
impl SubQueues {
    pub fn customer_sub_queue_setup(servers: SubQueues, client: &mut ClientQueueData, position: i32) {
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
        client.setup(position, sub_queue_position, timer);
    }
    
    fn sub_queue_realign(&mut self, old_sub_queue_position: i32, server_loc: i32) {
        let teller_info = &mut self.tellers[server_loc as usize];
        let teller_queue = &mut teller_info.users;
        let startup_time = teller_queue[1].clone().time_duration;
        //TODO: Change the sub_queue_position of all users after the removed user
        for (position, user) in teller_queue.iter_mut().enumerate() {
            // let mut user = user;
            if user.sub_queue_position > old_sub_queue_position {
                let remaining_time = startup_time;
                let timer =
                    (teller_info.teller.service_time * (position as i32 + 1)) + remaining_time;
                user.time_duration = timer;
                user.sub_queue_position = position as i32;
            }
        }
    }
    pub fn customer_add(&mut self, mut user: ClientQueueData, queue_len: usize) -> Result<ClientQueueData, String> {
        let queue_clone = self.clone();
        let position = queue_len as i32;
        let teller = &mut self.tellers[user.server_location as usize];
        match teller.teller.active {
            true => {
                Self::customer_sub_queue_setup(queue_clone, &mut user, position);
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
    pub fn customer_remove(&mut self, national_id: String, service_location: usize, main_queue: &mut Queue) -> Result<ClientQueueData, String> {
        let found_user = self.search_user(national_id.clone());
        match found_user {
            Err(err) => {
                error!("ERROR: {}", err);
                Err(err)
            }
            Ok(found_user) => {
                let mut queue = main_queue;
                let sub_queue = &mut self.tellers[service_location].users;
                let removed_user = sub_queue.remove(found_user.sub_queue_position as usize);
                ClientQueueData::remove_user(national_id);
                queue.queue = ClientQueueData::list_users();
                dbg!(queue.queue.clone());
                self.drain_queue(removed_user.server_location, removed_user.position, removed_user.sub_queue_position, &mut queue);
                let mut prev_pos = removed_user.sub_queue_position;
                for queued_user in queue.queue.iter_mut() {
                    if queued_user.position > removed_user.position  {
                        ClientQueueData::remove_user(queued_user.national_id.clone());
                        ClientQueueData::order();
                        let assigned_teller_loc = queued_user.position as usize % self.teller_count();
                        let temp_pos = queued_user.sub_queue_position;
                        let server_queue = &mut self.tellers[assigned_teller_loc];
                        server_queue.users.push(queued_user.clone());
                        queued_user.position = queued_user.position - 1;
                        queued_user.server_location = assigned_teller_loc as i32;
                        queued_user.sub_queue_position = prev_pos;
                        queued_user.assigned_server = server_queue.teller.server_id.clone();
                        dbg!(server_queue.teller.service_time );
                        dbg!(queued_user.sub_queue_position);
                        dbg!(server_queue.users[0].time_duration);
                        dbg!(server_queue.teller.service_time * queued_user.sub_queue_position + server_queue.users[0].time_duration);
                        queued_user.time_duration = server_queue.teller.service_time * queued_user.sub_queue_position + server_queue.users[0].time_duration;
                        dbg!(queued_user.clone());
                        queued_user.add_user();
                        prev_pos = temp_pos;
                    }
                }
                queue.queue = ClientQueueData::list_users();
                dbg!(queue.queue.clone());
                Ok(removed_user)
            }
        }

    }

    fn drain_queue(&mut self, server_loc: i32, user_position: i32, user_sub_pos: i32, queue: &mut Queue) {
        for user in queue.queue.iter_mut() {
            if user.position > user_position {
                for teller in self.tellers.iter_mut() {
                    if teller.teller.station == user.server_location {
                        teller.users.remove(user.sub_queue_position as usize - 1);
                        break;
                    }
                }
            }
        }

    }
    fn search_user(
        &mut self,
        national_id: String,
    ) -> Result<ClientQueueData, String> {
        ClientQueueData::find_user(national_id)
    }
}
