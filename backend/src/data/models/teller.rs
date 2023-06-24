use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};

#[derive(Selectable, Queryable, Insertable, Serialize, Deserialize)]
#[diesel(table_name = crate::data::schema::Tellers)]
#[diesel(check_for_backend(diesel::mysql::Mysql))]
pub struct Teller {
    pub server_id: String,
    pub server_station: i32,
    pub service_time: f32,
    pub active: bool,
}

#[derive(Insertable, Serialize, Deserialize)]
#[diesel(table_name = crate::data::schema::Tellers)]
#[diesel(check_for_backend(diesel::mysql::Mysql))]
pub struct TellerInsert {
    pub server_id: String,
    pub server_station: i32,
    pub service_time: f32,
    pub active: bool,
    pub password: String,
}

#[derive(Selectable, Queryable, Deserialize, Serialize, Clone)]
#[diesel(table_name = crate::data::schema::Tellers)]
#[diesel(check_for_backend(diesel::mysql::Mysql))]
pub struct TellerLogin {
    pub server_id: String,
    pub password: String,
}

#[derive(Selectable, Queryable, Deserialize, Serialize, Clone)]
#[diesel(table_name = crate::data::schema::Tellers)]
#[diesel(check_for_backend(diesel::mysql::Mysql))]
pub struct TellerQuery {
    pub server_id: String,
    pub server_station: i32,
    pub password: String,
}
#[derive(Selectable, Queryable, Deserialize, Serialize, Clone, Debug)]
#[diesel(table_name = crate::data::schema::Tellers)]
#[diesel(check_for_backend(diesel::mysql::Mysql))]
pub struct TellerQueueQuery {
    pub server_id: String,
    pub server_station: i32,
}

impl Teller {
    pub fn change_teller_status(&mut self, status: bool) -> &mut Teller {
        self.active = status;
        self
    }
}

impl Display for Teller {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "{}:{}:{}:{}",
            self.server_id, self.server_station, self.service_time, self.active,
        ))
    }
}
