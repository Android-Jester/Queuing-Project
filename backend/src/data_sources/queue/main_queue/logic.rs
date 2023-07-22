use crate::{data_sources::queue::sub_queue::prelude::SubQueues, prelude::*};

#[derive(Debug, Default)]
pub struct MainQueue {
    pub queue: Vec<ClientQueueData>,
}

impl MainQueue {
    fn main_queue_realign(&mut self, old_queue_position: usize) {
        //TODO: Change the sub_queue_position of all users after the removed user

        for (position, user) in self.queue.iter_mut().enumerate() {
            if user.position > old_queue_position {
                user.position = position
            }
        }
    }
    pub fn user_add(
        &mut self,
        mut added_user: ClientQueueData,
        sub_queue: &mut SubQueues,
    ) -> Result<ClientQueueData, String> {
        let existing_user = self
            .queue
            .iter()
            .find(|user| user.national_id == added_user.national_id);

        if existing_user.is_none() {
            let user_position = self.queue.len();
            if user_position < CUSTOMER_COUNT && sub_queue.teller_count() > 0 {
                added_user.setup_main(user_position);
                match sub_queue.customer_add(added_user.clone()) {
                    Ok(complete_user) => {
                        info!("USER FULL DATA: {:?}", complete_user);
                        self.queue.push(complete_user.clone());
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
        user: ClientQueueData,
        servers: &'a mut SubQueues,
    ) -> Result<ClientQueueData, String> {
        if !self.queue.is_empty() {
            let removed_user = self.queue.remove(user.position);
            self.main_queue_realign(removed_user.position);
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
            .find(|user| user.national_id == national_id);
        if let Some(user) = user_found {
            match self.user_remove(user.clone(), servers) {
                Ok(_) => Ok(()),
                Err(_) => Err("User Not Found".to_string()),
            }
        } else {
            Err("User Not Found".to_string())
        }
    }

    pub fn search_user(&self, national_id: String) -> ClientQueueData {
        self.queue
            .iter()
            .find(|user| national_id == user.national_id)
            .unwrap()
            .clone()
    }
}
