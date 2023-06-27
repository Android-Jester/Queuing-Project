
use crate::prelude::*;
use log::*;
#[derive(Debug)]
pub struct QueueStruct {
    pub queue: Vec<UserQueuePos>,
}

impl Default for QueueStruct {
    fn default() -> Self {
        QueueStruct {
            queue: Vec::with_capacity(CUSTOMER_COUNT),
        }
    }
}

impl QueueStruct {
    /*Main Queue Events*/

    fn set_up_timer(
        &mut self,
        service_period: f64,
        first_counter: Option<f64>,
        server_queue_index: usize,
    ) -> f64 {
        match server_queue_index {
            1 => 0.0,
            2..=CUSTOMER_COUNT => (service_period * server_queue_index as f64) + first_counter.unwrap_or(0.0),
            _ => (service_period * server_queue_index as f64) + first_counter.unwrap_or(0.0)
        }
    }


    pub fn add_user<'a>(
        &'a mut self,
        user: JoinedUserOutput,
        servers: &'a mut TellersQueue,
    ) -> Result<UserQueuePos, &str> {
     if self.queue.len() < CUSTOMER_COUNT {   
        let teller_pos = self.queue.len() % servers.tellers_num();
        let teller = servers.search_teller(teller_pos).unwrap();
        let user_query = find_user(user.user_query.national_id.clone())?;
        let timer = self.set_up_timer(0.0, Some(0.0), teller.users.len() + 1);
        let user_queue_pos = UserQueuePos::new(
            user_query.name,
            user.user_query.national_id, 
            user.action, 
            self.queue.len(), 
            teller.users.len(),
            teller_pos,
            timer,
        );
        self.queue.push(user_queue_pos.clone());
        servers.add_customer(teller_pos, user_queue_pos.clone())?;
        Ok(user_queue_pos)
        } else {
            Err("Unable to add user")
        }                
    }
    pub fn remove_user<'a>(
        &'a mut self,
        user_queue: UserQueuePos,
        servers: &'a mut TellersQueue,
    ) -> Result<(), &'a str> {
        
        match servers.remove_customer(user_queue.clone()) {
            Ok(_) => {
                self.queue.remove(user_queue.pos);

                Ok(())
            },
            Err(err) => {
                error!("Unable to remove customer");
                Err(err)
            },
        }
        
        
        
        
        
        
        // match user_queue_pos < CUSTOMER_COUNT {
        //     true => {
                
        //         match  {
        //             Ok(_) => {
        //                 for (user_pos, user_data) in self.queue.iter_mut().enumerate() {
        //                             user_data.change_queue_pos(user_pos);
        //                             user_data.change_assigned_teller(user_pos % servers.tellers_num());
        //                             let _ = servers.add_customer(
        //                                 user_pos % servers.tellers_num(),
        //                                 user_data.clone()
        //                             );
        //                 }
        //                 Ok(())
        //             }
        //             Err(_) => Err("Unable to remove user to teller"),
        //         }
        //     }
        //     false => Err("Too Many Users"),
        // }
    }
    /*Timer Events*/
    
 

    /*Live Changes*/
    pub fn queue_change(&mut self) {
        for (pos, user) in self.queue.iter_mut().enumerate() {
            user.change_queue_pos(pos + 1);
        }
    }
}
