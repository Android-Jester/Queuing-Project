use crate::prelude::*;
use chrono::{NaiveDateTime, Utc};

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

#[derive(
    Queryable, Selectable, Insertable, Default, Deserialize, Serialize, Clone, Debug, PartialEq,
)]
#[diesel(table_name = MainQueue)]
#[diesel(check_for_backend(diesel::mysql::Mysql))]
pub struct ClientQueueData {
    pub name: String,
    pub national_id: String,
    pub activity: String,
    pub position: i32,
    pub sub_queue_position: i32,
    #[serde(skip_serializing)]
    pub assigned_server: String,
    pub server_location: i32,
    pub time_duration: i32,
    pub time_joined: NaiveDateTime,
}

impl ClientQueueData {
    pub fn order() {
        let conn = &mut establish_connection();
        MainQueue::dsl::MainQueue
            .order(MainQueue::position.asc())
            .execute(conn)
            .unwrap();
    }

    pub fn new(
        user_input: ClientInputData,
        assigned_server: String,
        name: String,
        service_location: i32,
    ) -> Self {
        Self {
            name,
            position: 0,
            sub_queue_position: 0,
            server_location: service_location,
            assigned_server,
            time_duration: 0,
            activity: user_input.activity,
            national_id: user_input.national_id,
            time_joined: Utc::now().naive_utc(),
        }
    }
    pub fn new_fill(user: ClientQueueData) -> Self {
        Self { ..user }
    }

    pub async fn timer_countdown(national_id: String, broadcaster: Arc<ClientBroadcaster>) {
        tokio::spawn(async move {
            let national_id = national_id.clone();
            dbg!(national_id.clone());
            let user = ClientQueueData::find_user(national_id.clone());
            match user {
                Ok(data_user) => {
                    let mut time = db_teller_service_time(data_user.assigned_server)
                        * data_user.sub_queue_position
                        + ClientQueueData::first_user().unwrap().position;
                    let mut interval = tokio::time::interval(Duration::from_secs(2));
                    while time != 0 {
                        interval.tick().await;
                        time = time - 1;
                        dbg!(national_id.clone());
                        let data = ClientQueueData::show_countdown(
                            national_id.clone(),
                            broadcaster.clone(),
                            time,
                        )
                        .await;
                        match data {
                            Ok(_) => {
                                info!("DD")
                            }
                            Err(_) => break,
                        }
                    }
                }
                Err(_) => {
                    error!("ERROR")
                }
            }
        });
    }

    pub fn replace_fill(&mut self, user: ClientQueueData) {
        *self = user;
    }

    pub fn setup(&mut self, position: i32, sub_queue_position: i32, startup_timer: i32) {
        self.position = position;
        self.sub_queue_position = sub_queue_position;
        self.time_duration = startup_timer;
    }
}
