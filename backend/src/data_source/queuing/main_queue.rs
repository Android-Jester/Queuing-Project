use crate::prelude::*;
use log::*;
#[derive(Debug)]
pub struct MainQueue {
    pub queue: Vec<UserQueuePos>,
}

impl Default for MainQueue {
    fn default() -> Self {
        MainQueue {
            queue: Vec::with_capacity(CUSTOMER_COUNT),
        }
    }
}

impl MainQueue {
    /*Main Queue Events*/

    // adding user
    // 1. Get user's details
    // 2. Assign them to available teller
    // 3. Give them the time
    // 4. Include them into the queue
    // 5. Include them to sub queues

    pub fn add_user(
        &mut self,
        mut added_user: UserQueuePos,
        sub_queue: &mut SubQueues,
    ) -> Result<(), &str> {
        let user_position = self.queue.len();
        let service_location = user_position % sub_queue.tellers_count();
        if user_position < CUSTOMER_COUNT {
            added_user.setup_main(user_position, service_location);
            self.queue.push(added_user.clone());
            sub_queue.add_customer(added_user).unwrap();
            Ok(())
        } else {
            Err("Unable to add user")
        }
        // if self.queue.len() < CUSTOMER_COUNT {
        //     let teller_pos = self.queue.len() % servers.tellers_num();
        //     let teller = servers.search_teller(teller_pos).unwrap();
        //     let user_query = db_find_user(user.user_query.national_id.clone())?;
        //     let timer = self.set_up_timer(0.0, Some(0.0), teller.users.len() + 1);
        //     let user_queue_pos = UserQueuePos::new(
        //         user_query.name,
        //         user.user_query.national_id,
        //         user.action,
        //         self.queue.len(),
        //         teller.users.len(),
        //         teller_pos,
        //         timer,
        //     );
        //     self.queue.push(user_queue_pos.clone());
        //     servers.add_customer(teller_pos, user_queue_pos.clone())?;
        //     Ok(user_queue_pos)
        // } else {
        //     Err("Unable to add user")
        // }
    }
    pub fn remove_user<'a>(
        &'a mut self,
        user_queue: UserQueuePos,
        servers: &'a mut SubQueues,
    ) -> Result<(), &'a str> {
        match servers.remove_customer(user_queue.clone()) {
            Ok(_) => {
                self.queue.remove(user_queue.position.unwrap_or(0));
                Ok(())
            }
            Err(err) => {
                error!("Unable to remove customer");
                Err(err)
            }
        }
    }
    /*Live Changes*/
    // pub fn queue_change(&mut self) {
    //     for (pos, user) in self.queue.iter_mut().enumerate() {
    //         user.change_queue_pos(pos + 1);
    //     }
    // }
}
