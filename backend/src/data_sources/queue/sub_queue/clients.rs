
use std::task::Poll;

use crate::prelude::*;

use super::servers::ServerQueue;
impl super::prelude::SubQueues {
   pub fn customer_sub_queue_setup(servers: &SubQueues, client: &mut ClientQueueData) {
    let server = &servers.tellers[client.service_location];
        let sub_queue = &server.users;
        let sub_queue_position = sub_queue.len();
        let timer = match sub_queue_position {
            0 => 0,
            1..=CUSTOMER_COUNT => {
                let first_user = sub_queue.first().unwrap();
                let remaining_time = first_user.lock().startup_timer;

                (server.teller.service_time as usize * (sub_queue_position + 1)) + remaining_time
            }
            _ => server.teller.service_time as usize,
        };
        client.setup_sub(sub_queue_position, timer);
    }

    pub async fn timer_countdown<'a>(
        &mut self,
        client_ip: String,
        index: usize,
        teller_station: usize,
        broadcast: Arc<ClientBroadcaster>,
    ) {
        let user = &mut self.tellers[teller_station].users[index];
        // let (tx, rx) = std::sync::mpsc::channel::<ClientQueueData>();
        let user = user.clone();

        tokio::spawn(async move {
            info!("SPAWNED IN");
        // let user = Arc::clone(user);
           let mut user_data = user.lock();
            while user_data.startup_timer != 0 {
                std::thread::sleep(Duration::from_secs(1));
                user_data.startup_timer -= 1;
                info!("Index: {}", user_data.startup_timer);
                broadcast.broadcast_countdown(user_data.clone(), client_ip.clone()).await;
            }
        });
     
    }

    fn sub_queue_realign(
        teller_data: &mut ServerQueue,
        old_sub_queue_position: usize,
        remaining_time: usize,
    ) {
        //TODO: Change the sub_queue_position of all users after the removed user
        for (position, user) in teller_data.users.iter_mut().enumerate() {
            let mut user = user.lock(); 
            if user.sub_queue_position > old_sub_queue_position {
                let timer =
                    (teller_data.teller.service_time as usize * (position + 1)) + remaining_time;
                user.startup_timer = timer;
                user.sub_queue_position = position;
            }
        }
    }
    pub fn customer_add(&mut self, user: Arc<Mutex<ClientQueueData>>) -> Result<Arc<Mutex<ClientQueueData>>, String> {
        let mut mut_user = user.lock();
        let teller = &mut self.tellers[mut_user.service_location];

        match teller.teller.active {
            true => {
                match teller.users.len() < usize::MAX {
                    true => {
                        teller.users.push(user.clone());
                        info!("User After ADD: {:?}", user);
                        Ok(user.clone())
                    }
                    false => {
                        info!("Teller: {:?}", teller);
                        Err("Unable to add customer".to_string())
                    }
                }
            }
            false => {
                let service_location = mut_user.position % self.teller_count() + 1;
                let teller = &mut self.tellers[service_location];
                let teller_state = &mut teller.teller.active;
                if *teller_state {
                    // Self::customer_sub_queue_setup(self, &mut mut_user);
                    match teller.users.len() != usize::MAX {
                        true => {
                            teller.users.push(user.clone());
                            info!("User ADDED: {:?}", user);
                            Ok(user.clone())
                        }
                        false => Err("Unable to add customer".to_string()),
                    }
                } else {
                    info!("ERROR");

                    Err("Cannot add user".to_string())
                }
            }
        }
    }
    pub fn customer_remove(&mut self, queue_client: Arc<Mutex<ClientQueueData>>) -> Arc<Mutex<ClientQueueData>> {
        let queue_client = queue_client.lock();
        let teller = &mut self.tellers[queue_client.service_location];
        let remaining_time = teller.users.first().unwrap().lock().startup_timer;
        let removed_user = teller.users.remove(queue_client.sub_queue_position - 1);
        Self::sub_queue_realign(teller, queue_client.service_location, remaining_time);
        removed_user
    }
}
