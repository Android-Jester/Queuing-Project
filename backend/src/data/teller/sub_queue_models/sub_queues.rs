use std::thread::sleep;

use crate::prelude::*;

#[derive(Default, Debug, Deserialize, Serialize, Clone)]
pub struct SubQueues {
    pub tellers: Vec<ServerQueue>,
}

/// Teller Activities
impl SubQueues {
    pub fn teller_count(&self) -> usize {
        self.tellers.len()
    }
    pub fn teller_add(&mut self, teller: TellerQuery) -> Result<(), &'static str> {
        match self.teller_count() < SERVER_COUNT {
            true => {
                self.tellers.push(ServerQueue::new(teller));
                info!("{:?}", self.tellers);
                Ok(())
            }
            false => Err("Server List full"),
        }
    }
    pub fn teller_remove(&mut self, index: usize) -> Result<ServerQueue, &'static str> {
        if index >= 1 && index <= self.teller_count() {
            Ok(self.tellers.remove(index))
        } else {
            Err("Teller Not Available")
        }
    }
    pub fn teller_search(&self, station: usize) -> Result<&ServerQueue, &'static str> {
        if station < self.tellers.len() {
            Ok(&self.tellers[station])
        } else {
            Err("No Available Teller")
        }
    }
    pub fn teller_show_queue(&self, service_location: usize) -> Vec<UserQueuePos> {
        if self.teller_count() > 0 {
            let teller = &self.tellers[service_location];
            if !teller.users.is_empty() {
                teller.users.clone()
            } else {
                vec![]
            }
        } else {
            vec![]
        }
        // let mut teller_view: Vec<UserQueuePos> = Vec::new();
        // for user in teller.users.clone() {
        //     teller_view.push(user);
        // }
    }
    pub fn teller_queue_hold(&mut self, teller_position: usize) -> TellerState {
        let teller_form = &mut self.tellers[teller_position];
        if teller_form.users.is_empty() {
            set_teller_status(false, teller_form.teller.server_id.clone()).unwrap();
            self.teller_remove(teller_position)
                .expect("Unable to remove teller");

            TellerState::InActive
        } else {
            TellerState::PendingRelease
        }
    }
    pub fn teller_check_state(&mut self, teller_position: usize) -> TellerState {
        let teller_form = &mut self.tellers[teller_position];
        if teller_form.users.is_empty() {
            teller_form.teller.active = false;
            TellerState::InActive
        } else {
            teller_form.teller.active = true;
            TellerState::PendingRelease
        }
    }
}

/// User Activities
impl SubQueues {
    fn customer_sub_queue_setup(teller: &ServerQueue, user_queue_data: &mut UserQueuePos) {
        let sub_queue_position = teller.users.len();

        let timer = match sub_queue_position {
            0 => 0usize,
            1..=CUSTOMER_COUNT => {
                let remaining_time = teller.users.first().unwrap().startup_timer;
                warn!("Remaining Time: {}", remaining_time);
                warn!("Teller Time: {}", teller.teller.service_time);
                ((teller.teller.service_time) as usize * (sub_queue_position + 1)) + remaining_time
            }
            _ => 0,
        };
        warn!("Timer: {}", timer);

        user_queue_data.setup_sub(sub_queue_position, /*service_location,*/ timer);
        // Self::timer_countdown(user_queue_data);
    }

    pub async fn timer_countdown<'a>(
        &mut self,
        ip: String,
        index: usize,
        teller_station: usize,
        broadcast: Data<BroadcasterUser>,
    ) {
        let (tx, rx) = std::sync::mpsc::channel::<UserQueuePos>();
        let (tx_thread, rx_thread) = std::sync::mpsc::channel::<UserQueuePos>();
        let teller_queue = &mut self.tellers[teller_station].users;
        let user = &mut teller_queue[index];
        let temp_user = UserQueuePos::new_fill(user.clone());
        tx.send(temp_user).unwrap();
        tokio::spawn(async move {
            info!("SPAWNED IN");
            let mut user_data = rx.recv().unwrap();
            while user_data.startup_timer != 0 {
                sleep(Duration::from_secs(1));
                user_data.startup_timer -= 1;
                info!("Index: {}", user_data.startup_timer);
                broadcast.broadcast_countdown(&user_data, ip.clone()).await;
                match tx_thread.send(user_data.clone()) {
                    Ok(_) => {
                        info!("Successful");
                    }
                    Err(err) => {
                        error!("ERROR: {:?}", err)
                    }
                }
            }
        });
        match rx_thread.recv() {
            Ok(dd) => {
                let temp_data = dd;
                info!("SS: {:?}", temp_data);
                user.replace_fill(temp_data);
            }
            Err(err) => {
                error!("ERROR: {:?}", err);
            }
        }
    }

    fn sub_queue_realign(
        teller_data: &mut ServerQueue,
        old_sub_queue_position: usize,
        remaining_time: usize,
    ) {
        // self.tellers[service_location].users.iter_mut()
        // let teller_data = &mut self.tellers[service_location];
        //TODO: Change the sub_queue_position of all users after the removed user
        for (position, user) in teller_data.users.iter_mut().enumerate() {
            if user.sub_queue_position > old_sub_queue_position {
                let timer = (teller_data.teller.service_time * (position as f32 + 1.0)) as usize
                    + remaining_time;
                user.startup_timer = timer;
                user.sub_queue_position = position;
            }
        }
    }
    pub fn customer_add(&mut self, mut user: UserQueuePos) -> Result<UserQueuePos, String> {
        let teller = &mut self.tellers[user.service_location];

        match teller.teller.active {
            true => {
                Self::customer_sub_queue_setup(teller, &mut user /*service_loc*/);

                match teller.users.len() < usize::MAX {
                    true => {
                        teller.users.push(user.clone());
                        info!("Users: {:?}", teller.users);
                        Ok(user)
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
                            Ok(user)
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
    pub fn customer_remove(&mut self, user: UserQueuePos) -> UserQueuePos {
        let teller = &mut self.tellers[user.service_location];
        let remaining_time = teller.users[0].startup_timer;
        let removed_user = teller.users.remove(user.sub_queue_position - 1);
        Self::sub_queue_realign(teller, user.service_location, remaining_time);
        removed_user
    }
}
