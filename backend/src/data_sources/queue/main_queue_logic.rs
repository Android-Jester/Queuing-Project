use crate::prelude::*;

#[derive(Debug, Default)]
pub struct MainQueue {
    pub queue: Vec<UserQueuePos>,
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
        mut added_user: UserQueuePos,
        sub_queue: &mut SubQueues,
    ) -> Result<UserQueuePos, String> {
        let existing_user = self
            .queue
            .iter()
            .find(|user| user.national_id == added_user.national_id);

        if existing_user.is_none() {
            // TODO: Make it such that it starts from 1 not 0
            let user_position = self.queue.len();
            info!("User Position: {}", user_position);
            if user_position < CUSTOMER_COUNT && sub_queue.teller_count() > 0 {
                added_user.setup_main(user_position);
                self.queue.push(added_user.clone());
                match sub_queue.customer_add(added_user.clone()) {
                    Ok(_) => Ok(added_user),
                    Err(_) => Err("Unable to Assign User to a teller".to_string()),
                }
            } else {
                Err("Either No Teller Available or Queue is full".to_string())
            }
        } else {
            Err("User already In Queue".to_string())
        }
    }
    pub fn user_remove<'a>(
        &'a mut self,
        user: UserQueuePos,
        servers: &'a mut SubQueues,
    ) -> Result<UserQueuePos, String> {
        if self.queue.is_empty() {
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
            self.user_remove(user.clone(), servers);
            Ok(())
        } else {
            Err("User Not Found".to_string())
        }
    }
}
