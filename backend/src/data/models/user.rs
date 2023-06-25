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

#[derive(Default, Deserialize, Serialize, Clone, Debug)]
pub struct UserQueuePos {
    pub national_id: String,
    pub action: String,
    pub pos: usize,
    pub server_pos: Option<usize>,
    pub service_loc: Option<usize>,
    pub timer: f64,
}

impl UserQueuePos {
    pub fn new(
        national_id: String,
        action: String,
        pos: usize,
        server_pos: usize,
        service_loc: usize,
        timer: f64,
    ) -> Self {
        Self {
            national_id,
            action,
            pos,
            server_pos: Some(server_pos),
            service_loc: Some(service_loc),
            timer,
        }
    }

    pub fn change_queue_pos(&mut self, pos: usize) {
        self.pos = pos;
    }
    pub fn change_assigned_teller(&mut self, new_teller: usize) {
        self.service_loc = Some(new_teller);
    }
    pub fn change_teller_queue_pos(&mut self, new_server_pos: usize) {
        self.server_pos = Some(new_server_pos);
    }
}

#[derive(Deserialize, Clone)]
pub struct UserQueryData {
    pub national_id: String,
    pub action: String,
}

#[derive(Queryable, Insertable, Selectable, Serialize, Deserialize, Clone)]
#[diesel(table_name = crate::data::schema::Users)]
#[diesel(check_for_backend(diesel::mysql::Mysql))]
pub struct UserInsert {
    pub account_number: String,
    pub national_id: String,
    pub password: String,
}

#[derive(Selectable, Queryable, Serialize, Deserialize, Clone)]
#[diesel(table_name = crate::data::schema::Users)]
#[diesel(check_for_backend(diesel::mysql::Mysql))]
pub struct UserLoginQuery {
    pub account_number: String,
    pub password: String,
    pub national_id: String,
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

#[derive(Insertable, Serialize, Deserialize)]
#[diesel(table_name = crate::data::schema::Guests)]
#[diesel(check_for_backend(diesel::mysql::Mysql))]
pub struct GuestInsert {
    pub national_id: String,
    pub name: String,
    pub transaction_type: String,
    pub telephone_num: String,
}
