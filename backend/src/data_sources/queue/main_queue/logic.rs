use crate::{
    data_sources::{
        queue::sub_queue::prelude::SubQueues,
    },
    prelude::*,
};

#[derive(Debug, Default)]
pub struct Queue {
    pub queue: Vec<ClientQueueData>,
}

impl Queue {
    pub fn user_add(&mut self, queue_client: ClientQueueData, sub_queue: &mut SubQueues) -> Result<ClientQueueData, String> {
                match sub_queue.customer_add(queue_client) {
                    Ok(user_data) => {
                        let user = user_data.add_user();
                        match user {
                            Ok(_) => {
                                info!("User Added");
                                Ok(user_data)
                            },
                            Err(err) => {
                                error!("ERROR: {err}");
                                Err(format!("ERROR: {err}"))
                            },
                        }
                    },
                    Err(err) => { error!("ERROR: {err}"); Err(format!("ERROR: {err}")) },
                }
    }
    // fn main_queue_realign(&mut self, old_queue_position: usize) {
    //     //TODO: Change the sub_queue_position of all users after the removed user
    //
    //     for (position, user) in self.queue.iter_mut().enumerate() {
    //         let mut user = user;
    //         if user.position > old_queue_position {
    //             user.position = position
    //         }
    //     }
    // }
    pub fn user_remove<'a>(
        &'a mut self,
        national_id: String,
        servers: &'a mut SubQueues,
    ) -> Result<usize, String> {
        if !self.queue.is_empty() {
            let user = self.search_user(national_id).unwrap();
            let removed_user = self.queue.remove(user.position as usize);
            // self.main_queue_realign(removed_user_mut.position);
            Ok(servers.customer_remove(
                removed_user.national_id.clone(),
                removed_user.server_location as usize,
            ))
        } else {
            Err("User Removal Not Possible".to_string())
        }
    }
    pub fn user_dismiss<'a>(
        &'a mut self,
        national_id: String,
        servers: &'a mut SubQueues,
    ) -> Result<(), String> {
        let user_found = self.search_user(national_id);
        if let Ok(user) = user_found {
            let user = user;
            let user_national_id = user.national_id.clone();
            match self.user_remove(user_national_id, servers) {
                Ok(_) => Ok(()),
                Err(_) => Err("User Not Found".to_string()),
            }
        } else {
            Err("User Not Found".to_string())
        }
    }
    pub fn search_user(&self, national_id: String) -> Result<ClientQueueData, String> {
        ClientQueueData::find_user(national_id)
    }
}
