use actix_web_lab::__reexports::tracing::info;
use diesel::prelude::*;
use schema::Clients::national_id;

use super::super::prelude::establish_connection;
use crate::data::schema::{self, MainQueue, MainQueue::dsl::MainQueue as main_queue};
use crate::prelude::ClientQueueData;
use crate::prelude::MainQueue::dsl::MainQueue as OtherMainQueue;



impl crate::prelude::ClientQueueData {
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
            Err(e) => Err(format!("ERROR: {}",e )),
        }
    }
    pub fn update_queue(id: String, time: i32) -> Result<(), String> {
        let conn = &mut establish_connection();
        match conn.transaction(|connection| {
            let update = diesel::update(MainQueue::dsl::MainQueue.filter(MainQueue::dsl::national_id.eq(id)))
                .set(MainQueue::dsl::time_duration.eq(time))
                .execute(connection);
            update
        }) {
            Ok(_) => Ok(()),
            Err(err) => Err(format!("ERROR: {}", err))
        }
    }
    pub fn show_countdown(id: String, time: i32) -> Result<usize, diesel::result::Error> {
        let conn = &mut establish_connection();
        let res = conn.transaction(|connection| {
            let table = main_queue.filter(national_id.eq(id.clone()));
            let updatable = diesel::update(MainQueue::table.find(id))
                .set(MainQueue::time_duration.eq(time))
                .execute(connection);
            updatable
        });
        res
    }
    pub fn find_user(id: String) -> Result<ClientQueueData, String> {
        let connection = &mut establish_connection();
        match connection.transaction(|conn| {
            let data = MainQueue::dsl::MainQueue
                .select(ClientQueueData::as_select())
                .filter(MainQueue::dsl::national_id.eq(id))
                .first(conn);
            data
        }) {
            Ok(data) => Ok(data),
            Err(err) => Err(format!("ERROR: {}", err))
        }
    }
    pub fn remove_user(id: String) {
        let connection = &mut establish_connection();
        let res = connection.transaction(|conn| {
            diesel::delete(MainQueue::table)
                .filter(MainQueue::national_id.eq(id))
                .execute(conn)
        });
        match res {
            Ok(data) => {}
            Err(_) => todo!(),
        }
    }
}
