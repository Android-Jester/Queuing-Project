use actix_web_lab::__reexports::tracing::info;
use diesel::prelude::*;
use diesel::result::Error;
use log::error;

use super::super::prelude::establish_connection;
use crate::data::schema::{self, MainQueue, MainQueue::dsl::MainQueue as main_queue};
use crate::prelude::{ClientBroadcaster, ClientQueueData, SubQueues};

impl crate::prelude::ClientQueueData {
    pub fn first_user() -> Result<ClientQueueData, String> {
        let connection = &mut establish_connection();
        let user = connection.transaction(|conn| {
            MainQueue::dsl::MainQueue
                .filter(MainQueue::position.eq(0))
                .select(ClientQueueData::as_select())
                .first(conn)
        });
        match user {
            Ok(user_data) => Ok(user_data),
            Err(_) => Err("Unable to find user".to_string()),
        }
    }
    pub fn add_user(&self) -> Result<(), String> {
        let connection = &mut establish_connection();
        let insertion = connection.transaction(|conn| {
            info!("USER: {:?}", self);
            diesel::insert_into(main_queue)
                .values(self.clone())
                .execute(conn)
        });
        match insertion {
            Ok(_) => Ok(()),
            Err(e) => Err(format!("ERROR: {}", e)),
        }
    }

    pub async fn show_countdown(
        id: String,
        broadcast: std::sync::Arc<ClientBroadcaster>,
        time: i32,
    ) -> Result<usize, Error> {
        let conn = &mut establish_connection();
        let res = conn.transaction(|connection| {
            let user = ClientQueueData::find_user(id.clone());
            match user {
                Ok(_) => {
                    let update = diesel::update(MainQueue::dsl::MainQueue.find(id.clone()))
                        .set(MainQueue::time_duration.eq(time))
                        .execute(connection);

                    update
                }
                Err(_) => Err(Error::NotFound),
            }
        });
        match res {
            Ok(data) => {
                let user = ClientQueueData::find_user(id.clone()).unwrap();
                broadcast.joining(user, id).await;
                Ok(data)
            }
            Err(err) => {
                error!("ERROR: NO RECORD FOUND");
                Err(err)
            }
        }
    }
    pub fn find_user(id: String) -> Result<ClientQueueData, String> {
        let connection = &mut establish_connection();
        match connection.transaction(|conn| {
            let data = MainQueue::dsl::MainQueue
                .find(id)
                .select(ClientQueueData::as_select())
                .first(conn);
            data
        }) {
            Ok(data) => Ok(data),
            Err(err) => Err(format!("ERROR: {}", err)),
        }
    }
    pub fn remove_user(id: String) {
        let connection = &mut establish_connection();
        let res = connection.transaction(|conn| {
            let delete = diesel::delete(MainQueue::table)
                .filter(MainQueue::national_id.eq(id))
                .execute(conn);
            delete
        });
        match res {
            Ok(_) => {}
            Err(_) => todo!(),
        }
    }
    pub fn list_users() -> Vec<ClientQueueData> {
        let conn = &mut establish_connection();
        let users = MainQueue::dsl::MainQueue
            .select(ClientQueueData::as_select())
            .load(conn)
            .expect("Cannot load all users");
        users
    }
}
