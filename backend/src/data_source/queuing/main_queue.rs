use crate::prelude::*;
use log::*;
use crate::prelude::Users::national_id;

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
    ) -> Result<UserQueuePos, &str> {
        match self.queue.iter().find(|user| user.national_id == added_user.national_id) {
            None => {
                let user_position = self.queue.len();
                if user_position < CUSTOMER_COUNT && sub_queue.tellers_count() > 0 {
                    added_user.setup_main(user_position, user_position % sub_queue.tellers_count());
                    sub_queue.add_customer(&mut added_user).unwrap();
                    self.queue.push(added_user.clone());
                    Ok(added_user)
                } else {
                    Err("Unable to add user")
                }
            }
            Some(_) => {
                Err("User already in queue")
            }
        }
    }
    pub fn remove_user<'a>(
        &'a mut self,
        user_queue: UserQueuePos,
        servers: &'a mut SubQueues,
    ) {
        let removed_user = self.queue.remove(user_queue.position);
        self.main_queue_realign(removed_user.position);
        servers.remove_customer(removed_user);

    }
    /*Live Changes*/
    pub fn queue_change(&mut self) {
        for (pos, user) in self.queue.iter_mut().enumerate() {
            user.change_queue_pos(pos + 1);
        }
    }

    fn main_queue_realign(&mut self, old_queue_position: usize) {
        //TODO: Change the sub_queue_position of all users after the removed user

        for (position, user) in self.queue.iter_mut().enumerate() {
            if user.position > old_queue_position {
                user.position = position
            }
        }
    }
}
