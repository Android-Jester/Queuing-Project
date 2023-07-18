use crate::prelude::*;

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
        // let teller = &self.tellers[service_location];
        let sub_queue_position = teller.users.len() + 1;

        let timer = match sub_queue_position {
            1 => Duration::from_secs(0),
            2..=CUSTOMER_COUNT => {
                let remaining_time = teller.users.first().unwrap().startup_timer;
                Duration::from_secs_f64(
                    ((teller.teller.service_time * 60.0) as f64
                        * (sub_queue_position as f64 + 1.0))
                        + remaining_time.as_secs() as f64,
                )
            }
            _ => Duration::from_secs_f64((teller.teller.service_time * 60.0) as f64),
        };

        user_queue_data.setup_sub(sub_queue_position, /*service_location,*/ timer);
        Self::timer_countdown(user_queue_data);
    }
    fn timer_countdown(user: &mut UserQueuePos) {
        let (tx, rx) = std::sync::mpsc::channel::<Duration>();
        let (tx_thread2, rx_thread2) = std::sync::mpsc::channel::<Duration>();
        tx.send(user.startup_timer).unwrap();
        std::thread::spawn(move || {
            let startup = match rx.recv() {
                Ok(timed_data) => timed_data.as_secs(),
                Err(_) => 0,
            };
            for t in (0..=startup).rev() {
                std::thread::sleep(Duration::from_secs(1));
                tx_thread2.send(Duration::from_secs(t)).unwrap();
            }
        });
        user.startup_timer = rx_thread2.recv().unwrap();
    }
    fn sub_queue_realign(
        teller_data: &mut ServerQueue,
        old_sub_queue_position: usize,
        remaining_time: Duration,
    ) {
        // self.tellers[service_location].users.iter_mut()
        // let teller_data = &mut self.tellers[service_location];
        //TODO: Change the sub_queue_position of all users after the removed user
        for (position, user) in teller_data.users.iter_mut().enumerate() {
            if user.sub_queue_position > old_sub_queue_position {
                let timer = Duration::from_secs_f32(
                    (teller_data.teller.service_time * (position as f32 + 1.0))
                        + remaining_time.as_secs_f32(),
                );
                user.startup_timer = timer;
                user.sub_queue_position = position;
            }
        }
    }
    pub fn customer_add(&mut self, mut user: UserQueuePos) -> Result<(), String> {
        let teller = &mut self.tellers[user.service_location];
        match teller.teller.active {
            true => {
                Self::customer_sub_queue_setup(teller, &mut user /*service_loc*/);

                match teller.users.len() != usize::MAX {
                    true => {
                        info!("User: {:?}", user);
                        teller.users.push(user.clone());
                        // Self::count_down_timer(user);
                        Ok(())
                    }
                    false => Err("Unable to add customer".to_string()),
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
