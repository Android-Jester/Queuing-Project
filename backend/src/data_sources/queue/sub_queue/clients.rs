

use crate::prelude::*;

use super::servers::ServerQueue;
impl SubQueues {
   pub fn customer_sub_queue_setup(servers: &SubQueues, client: &mut ClientQueueData) {
    let server = &servers.tellers[client.service_location];
        let sub_queue = &server.users;
        let sub_queue_position = sub_queue.len();
        let timer = match sub_queue_position {
            0 => 0,
            1 => server.teller.service_time as usize * (sub_queue_position),
            1..=CUSTOMER_COUNT => {
                let first_user = sub_queue[1].lock();
                let remaining_time = first_user.startup_timer;
                (server.teller.service_time as usize * (sub_queue_position)) + remaining_time
            }
            _ => server.teller.service_time as usize,
        };
        client.setup_sub(sub_queue_position, timer);
    }
   pub async fn timer_countdown(
        &mut self,
        client_ip: String,
        index: usize,
        teller_station: usize,
        broadcast: Arc<ClientBroadcaster>,
    ) {
        let user = &mut self.tellers[teller_station].users[index];
        let user = user.clone();
        tokio::spawn(async move {
            info!("SPAWNED IN");
           let mut user_data = user.lock();
            while user_data.startup_timer != 0 {
                std::thread::sleep(Duration::from_secs(1));
                user_data.startup_timer -= 1;
                info!("Index: {}", user_data.startup_timer);
                broadcast.broadcast_countdown(user_data.clone(), client_ip.clone()).await;
            }
        });
        dbg!(&mut self.tellers[teller_station].users);
     
    }

    fn sub_queue_realign(
        &mut self,
        old_sub_queue_position: usize,
        server_loc: usize
    ) {
        let teller_info = &mut self.tellers[server_loc];
        let teller_queue = &mut teller_info.users;
        let startup_time = teller_queue[1].lock().clone().startup_timer;
        //TODO: Change the sub_queue_position of all users after the removed user
        for (position, user) in teller_queue.iter_mut().enumerate() {
            let mut user = user.lock();
            if user.sub_queue_position > old_sub_queue_position {

                let remaining_time = startup_time;
                let timer =
                    (teller_info.teller.service_time as usize * (position + 1)) + remaining_time;
                user.startup_timer = timer;
                user.sub_queue_position = position;
            }
        }
    }
    pub fn customer_add(&mut self, user: Arc<Mutex<ClientQueueData>>) -> Result<(), String> {
        let mut_user = user.lock();
        let teller = &mut self.tellers[mut_user.service_location];
        match teller.teller.active {
            true => {
                        teller.users.push(user.clone());
                        info!("User After ADD: {:?}", user);
                        Ok(())
            }
            false => {

                let service_location = mut_user.position % self.teller_count() + 1;
                let teller = &mut self.tellers[service_location];
                let teller_state = &mut teller.teller.active;
                // FIXME: Check for available tellers and show the available
                if *teller_state {
                            teller.users.push(user.clone());
                            info!("User ADDED: {:?}", user);
                            Ok(())
                } else {
                    info!("ERROR: Cannot add user");
                    Err("Cannot add user".to_string())
                }
            }
        }
    }
    //FIXME: Reassign users to queue
    pub fn customer_remove(&mut self, national_id: String, service_location: usize) -> usize {
        let found_user = self.search_user(national_id, service_location).unwrap();
        let found_user = found_user.lock();
        let user_queue = &mut self.tellers[service_location].users;
        let _ = user_queue.remove(found_user.sub_queue_position);
        self.sub_queue_realign(found_user.sub_queue_position, found_user.service_location);
        found_user.service_location
    }
   fn search_user(&mut self, national_id: String, service_location: usize) -> Option<Arc<Mutex<ClientQueueData>>> {
        let teller = &mut self.tellers[service_location];
        teller.users.iter().find(|user|user.lock().national_id == national_id).cloned()
    }
}
