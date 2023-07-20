use crate::prelude::*;
use diesel::mysql::Mysql;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone)]
pub struct TellerLogin {
    pub server_id: String,
    pub password: String,
}

#[derive(Selectable, Queryable, Deserialize, Serialize, Clone, PartialEq, Debug)]
#[diesel(table_name = Tellers)]
#[diesel(check_for_backend(Mysql))]
pub struct TellerQuery {
    pub server_id: String,
    pub server_station: i32,
    #[serde(skip_serializing)]
    pub password: String,
    pub service_time: f32,
    pub active: bool,
}
