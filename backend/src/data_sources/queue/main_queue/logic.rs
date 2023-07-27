use crate::{data_sources::queue::sub_queue::prelude::SubQueues, prelude::*};

#[derive(Debug, Default)]
pub struct Queue {
    pub queue: Vec<ClientQueueData>,
}

impl Queue {
    pub fn user_add(
        &mut self,
        queue_client: ClientQueueData,
        sub_queue: &mut SubQueues,
    ) -> Result<ClientQueueData, String> {
        self.queue = ClientQueueData::list_users();
        match sub_queue.customer_add(queue_client, self.queue.len()) {
            Ok(user_data) => {
                let user = user_data.add_user();
                match user {
                    Ok(_) => {
                        info!("User Added");
                        Ok(user_data)
                    }
                    Err(err) => {
                        error!("ERROR: {err}");
                        Err(format!("ERROR: {err}"))
                    }
                }
            }
            Err(err) => {
                error!("ERROR: {err}");
                Err(format!("ERROR: {err}"))
            }
        }
    }
    pub async fn user_remove<'a>(
        &'a mut self,
        national_id: String,
        servers: &'a mut SubQueues,
        broadcast: Arc<ClientBroadcaster>,
    ) -> Result<ClientQueueData, String> {
        if !self.queue.is_empty() {
            let removed_user = self.search_user(national_id.clone());
            match removed_user {
                Ok(removed_user) => {
                    servers
                        .customer_remove(
                            removed_user.national_id.clone(),
                            removed_user.server_location as usize,
                            self,
                            broadcast,
                        )
                        .await
                }
                Err(err) => {
                    error!("No User Found");
                    Err("No User Found".to_string())
                }
            }
        } else {
            Err("Queue is Empty".to_string())
        }
    }
    // pub fn user_dismiss<'a>(
    //     &'a mut self,
    //     national_id: String,
    //     servers: &'a mut SubQueues,
    // ) -> Result<(), String> {
    //     let user_found = self.search_user(national_id);
    //     if let Ok(user) = user_found {
    //         let user = user;
    //         let user_national_id = user.national_id.clone();
    //         match self.user_remove(user_national_id, servers).await {
    //             Ok(_) => Ok(()),
    //             Err(_) => Err("User Not Found".to_string()),
    //         }
    //     } else {
    //         Err("User Not Found".to_string())
    //     }
    // }
    pub fn search_user(&self, national_id: String) -> Result<ClientQueueData, String> {
        ClientQueueData::find_user(national_id)
    }
}
