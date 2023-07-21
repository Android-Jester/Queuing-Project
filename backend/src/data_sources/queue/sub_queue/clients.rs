use crate::prelude::*;

use super::servers::ServerQueue;
impl super::prelude::SubQueues {
    fn customer_sub_queue_setup(server: &ServerQueue, user_queue_data: &mut ClientQueueData) {
        let sub_queue_position = server.users.len() + 1;
        let timer = match sub_queue_position {
            1 => 0,
            2..=CUSTOMER_COUNT => {
                let remaining_time = server.users.first().unwrap().startup_timer;

                (server.teller.service_time as usize * (sub_queue_position + 1)) + remaining_time
            }
            _ => server.teller.service_time as usize,
        };

        user_queue_data.setup_sub(sub_queue_position, timer);
    }

    pub async fn timer_countdown<'a>(
        &mut self,
        client_ip: String,
        index: usize,
        teller_station: usize,
        broadcast: Data<ClientBroadcaster>,
    ) {
        let (tx, rx) = std::sync::mpsc::channel::<ClientQueueData>();
        info!("SPAWNED: TELLER_STATION: {}", teller_station);
        info!("SPAWNED: USER POSITION: {}", index);
        let teller_queue = &mut self.tellers[teller_station].users;
        let user = &mut teller_queue[index];
        info!("USSSERRR: {:?}", user);
        let user_time = user.startup_timer;
        tx.send(user.clone()).unwrap();
        tokio::spawn(async move {
            info!("SPAWNED IN");
            warn!("User Time: {:?}", user_time);
            let mut user_data = rx.recv().unwrap();
            for index in (0..=user_time).rev() {
                std::thread::sleep(Duration::from_secs(1));
                user_data.startup_timer = index;
                info!("Index: {}", index);
                broadcast
                    .broadcast_countdown(&user_data, client_ip.clone())
                    .await;
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
            if user.sub_queue_position > old_sub_queue_position {
                let timer =
                    (teller_data.teller.service_time as usize * (position + 1)) + remaining_time;
                user.startup_timer = timer;
                user.sub_queue_position = position;
            }
        }
    }
    pub fn customer_add(&mut self, mut user: ClientQueueData) -> Result<(), String> {
        let teller = &mut self.tellers[user.service_location];

        match teller.teller.active {
            true => {
                Self::customer_sub_queue_setup(teller, &mut user /*service_loc*/);

                match teller.users.len() < usize::MAX {
                    true => {
                        info!("User: {:?}", user);
                        teller.users.push(user.clone());

                        // Self::count_down_timer(user);
                        Ok(())
                    }
                    false => {
                        info!("Teller: {:?}", teller);
                        Err("Unable to add customer".to_string())
                    }
                }
            }
            false => {
                let service_location = user.position % self.teller_count() + 1;
                let teller = &mut self.tellers[service_location];
                let teller_state = &mut teller.teller.active;
                if *teller_state {
                    Self::customer_sub_queue_setup(teller, &mut user);
                    match teller.users.len() != usize::MAX {
                        true => {
                            info!("User: {:?}", user);
                            teller.users.push(user.clone());
                            // Self::count_down_timer(user);
                            Ok(())
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
    pub fn customer_remove(&mut self, queue_client: ClientQueueData) -> ClientQueueData {
        let teller = &mut self.tellers[queue_client.service_location];
        let remaining_time = teller.users[0].startup_timer;
        let removed_user = teller.users.remove(queue_client.sub_queue_position - 1);
        Self::sub_queue_realign(teller, queue_client.service_location, remaining_time);
        removed_user
    }
}
