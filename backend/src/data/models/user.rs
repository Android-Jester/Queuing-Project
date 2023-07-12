use actix_web::web;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Selectable, Queryable, Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[diesel(table_name = crate::data::schema::Users)]
#[diesel(check_for_backend(diesel::mysql::Mysql))]
pub struct UserQuery {
    pub name: String,
    pub account_number: String,
    pub national_id: String,
}

#[derive(Default, Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct UserQueuePos {
    pub name: String,
    pub national_id: String,
    pub activity: String,
    pub position: usize,
    pub sub_queue_position: usize,
    pub service_location: usize,
    pub startup_timer: f64,
}
#[derive(Default, Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct UserQueueInput {
    pub national_id: String,
    pub activity: String,
}

impl UserQueuePos {
    pub fn new(user_input: web::Json<UserInputData>, name: String) -> Self {
        Self {
            name,
            national_id: user_input.national_id.clone(),
            activity: user_input.activity.clone(),
            position: 0,
            sub_queue_position: 0,
            service_location: 0,
            startup_timer: 0.0,
        }
    }

    pub fn setup_main(&mut self, position: usize) {
        self.position = position;
    }

    pub fn setup_sub(&mut self, sub_queue_position: usize, teller_location: usize, startup_timer: f64) {
        self.sub_queue_position = sub_queue_position;
        self.startup_timer = startup_timer;
        self.service_location = teller_location;
    }

    pub fn change_queue_pos(&mut self, pos: usize) {
        self.position = pos;
    }
    pub fn change_assigned_teller(&mut self, new_teller: usize) {
        self.service_location = new_teller;
    }
    pub fn change_teller_queue_pos(&mut self, new_server_pos: usize) {
        self.sub_queue_position = new_server_pos;
    }
}

#[derive(Deserialize, Clone, Debug)]
pub struct UserInputData {
    pub national_id: String,
    pub activity: String,
}

#[derive(Queryable, Insertable, Selectable, Serialize, Deserialize, Clone)]
#[diesel(table_name = crate::data::schema::Users)]
#[diesel(check_for_backend(diesel::mysql::Mysql))]
pub struct UserInsert {
    pub name: String,
    pub account_number: String,
    pub national_id: String,
    pub password: String,
}

#[derive(Selectable, Queryable, Serialize, Deserialize, Clone, Debug)]
#[diesel(table_name = crate::data::schema::Users)]
#[diesel(check_for_backend(diesel::mysql::Mysql))]
pub struct UserLoginQuery {
    pub name: String,
    pub account_number: String,
    pub password: String,
    pub national_id: String,
}

#[derive(/*Selectable, Queryable */ Serialize, Deserialize, Clone, Debug)]
// #[diesel(table_name = crate::data::schema::Users)]
// #[diesel(check_for_backend(diesel::mysql::Mysql))]
pub struct UserDataQuery {
    pub name: String,
    pub account_number: String,
    pub national_id: String,
}

impl UserDataQuery {
    pub fn new(name: String, account_number: String, national_id: String) -> Self {
        Self {
            name,
            account_number,
            national_id,
        }
    }
}

#[derive(Selectable, Queryable, Serialize, Deserialize, Clone)]
#[diesel(table_name = crate::data::schema::Users)]
#[diesel(check_for_backend(diesel::mysql::Mysql))]
pub struct UserLogin {
    pub account_number: String,
    pub password: String,
}

#[derive(Selectable, Queryable, Insertable, Serialize, Deserialize, Clone)]
#[diesel(table_name = crate::data::schema::Guests)]
#[diesel(check_for_backend(diesel::mysql::Mysql))]
pub struct Guest {
    pub national_id: String,
    pub name: String,
    pub transaction_type: String,
    pub telephone_num: String,
}

#[derive(Insertable, Serialize, Deserialize, Clone)]
#[diesel(table_name = crate::data::schema::Guests)]
#[diesel(check_for_backend(diesel::mysql::Mysql))]
pub struct GuestInsert {
    pub national_id: String,
    pub name: String,
    pub transaction_type: String,
    pub telephone_num: String,
}
