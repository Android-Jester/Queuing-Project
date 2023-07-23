use crate::{data_sources::queue::sub_queue::prelude::SubQueues, prelude::*};

#[derive(Debug, Default)]
pub struct MainQueue {
    pub queue: Vec<Arc<Mutex<ClientQueueData>>>,
}

impl MainQueue {
    fn main_queue_realign(&mut self, old_queue_position: usize) {
        //TODO: Change the sub_queue_position of all users after the removed user

        for (position, user) in self.queue.iter_mut().enumerate() {
            let mut user = user.lock();
            if user.position > old_queue_position {
                user.position = position
            }
        }
    }
    pub fn user_add(
        &mut self,
        mut added_user: ClientQueueData,
        sub_queue: &mut SubQueues,
    ) -> Result<Arc<Mutex<ClientQueueData>>, String> {
        let existing_user = self.search_user(added_user.national_id.clone());
        if existing_user.is_none() {
            let user_position = self.queue.len();
            if user_position < CUSTOMER_COUNT && sub_queue.teller_count() > 0 {
                added_user.setup_main(user_position, &sub_queue);
                self.queue.push(Arc::new(Mutex::new(added_user.clone())));
                let new_user = self.search_user(added_user.national_id.clone()).unwrap();
                match sub_queue.customer_add(new_user) {
                    Ok(complete_user) => {
                        info!("USER FULL DATA: {:?}", complete_user);
                        Ok(complete_user)
                    }
                    Err(_) => {
                        error!("Unable to Assign User to a teller");
                        Err("Unable to Assign User to a teller".to_string())
                    }
                }
            } else {
                error!("Either No Teller Available or Queue is full");
                Err("Either No Teller Available or Queue is full".to_string())
            }
        } else {
            error!("User already In Queue");
            Err("User already In Queue".to_string())
        }
    }
    pub fn user_remove<'a>(
        &'a mut self,
        user: Arc<Mutex<ClientQueueData>>,
        servers: &'a mut SubQueues,
    ) -> Result<Arc<Mutex<ClientQueueData>>, String> {
        if !self.queue.is_empty() {
            let removed_user = self.queue.remove(user.lock().position);
            let removed_user_mut = removed_user.lock();
            self.main_queue_realign(removed_user_mut.position);
            servers.customer_remove(removed_user.clone());
            Ok(servers.customer_remove(removed_user.clone()))
            // Ok(())
        } else {
            Err("User Removal Not Possible".to_string())
        }
    }
    pub fn user_dismiss<'a>(
        &'a mut self,
        national_id: String,
        servers: &'a mut SubQueues,
    ) -> Result<(), String> {
        let user_found = self
            .queue
            .iter()
            .find(|user| user.lock().national_id == national_id);
        if let Some(user) = user_found {
            match self.user_remove(user.clone(), servers) {
                Ok(_) => Ok(()),
                Err(_) => Err("User Not Found".to_string()),
            }
        } else {
            Err("User Not Found".to_string())
        }
    }

    pub fn search_user(&self, national_id: String) -> Option<Arc<Mutex<ClientQueueData>>> {
        self.queue
            .iter()
            .find(|user| national_id == user.lock().national_id).cloned()
            
    }
}
