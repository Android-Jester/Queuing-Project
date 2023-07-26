use std::thread;
use async_std::task::JoinHandle;

use crate::prelude::*;

#[derive(Serialize, Deserialize, Clone)]
pub struct ClientLoginData {
    pub account_number: String,
    pub password: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct ClientInputData {
    pub national_id: String,
    pub activity: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ClientDataQuery {
    pub name: String,
    pub account_number: String,
    pub national_id: String,
}

impl ClientDataQuery {
    pub fn new(name: String, account_number: String, national_id: String) -> Self {
        Self {
            name,
            account_number,
            national_id,
        }
    }
}

#[derive(Queryable, Selectable, Insertable, Default, Deserialize, Serialize, Clone, Debug, PartialEq)]
#[diesel(table_name = MainQueue)]
#[diesel(check_for_backend(diesel::mysql::Mysql))]
pub struct ClientQueueData {
    pub name: String,
    pub national_id: String,
    pub activity: String,
    pub position: i32,
    pub sub_queue_position: i32,
    pub assigned_server: String,
    pub server_location: i32,
    pub time_duration: i32,
}

impl ClientQueueData {
    pub fn new(user_input: ClientInputData, assigned_server: String, name: String, service_location: i32) -> Self {
        Self {
            name,
            position: 0,
            sub_queue_position: 0,
            server_location: service_location,
            assigned_server,
            time_duration: 0,
            activity: user_input.activity,
            national_id: user_input.national_id,
        }
    }
    pub fn new_fill(user: ClientQueueData) -> Self {
        Self { ..user }
    }

    pub async fn timer_countdown(
        &mut self,
        client_ip: String,
        broadcaster: Arc<ClientBroadcaster>,
    ) {
        let data = Arc::new(Mutex::new(self.clone()));
        tokio::spawn(async move {
            let mut user = data.lock();
            let mut interval = tokio::time::interval(Duration::from_secs(1));
            loop {
                if user.time_duration != 0 {
                    interval.tick().await;
                    user.time_duration -= 1;
                    match Self::show_countdown(user.national_id.clone(), user.time_duration) {
                        Ok(_) => {
                            info!("Index: {:?}", user.time_duration);
                            broadcaster.countdowning(user.clone(), client_ip.clone()).await;
                        },
                        Err(err) => {
                            error!("ERROR: {:?}", err);
                            continue
                        }
                    };
                    continue;
                }
            }
        });
    }

    pub fn replace_fill(&mut self, user: ClientQueueData) {
        *self = user;
    }

    pub fn setup_main(&mut self, position: i32, servers: SubQueues) {
        self.position = position;
        SubQueues::customer_sub_queue_setup(servers, self);
    }

    pub fn setup_sub(&mut self, sub_queue_position: i32, startup_timer: i32) {
        self.sub_queue_position = sub_queue_position;
        self.time_duration = startup_timer;
    }
}
