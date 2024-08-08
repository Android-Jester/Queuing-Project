use crate::prelude::*;
use diesel::pg::Pg;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone)]
pub struct ServerLogin {
    pub server_id: String,
    pub password: String,
}

#[derive(Selectable, Queryable, Deserialize, Serialize, Clone, PartialEq, Debug)]
#[diesel(table_name = Servers)]
#[diesel(check_for_backend(Pg))]
pub struct ServerQuery {
    pub server_id: String,
    pub station: i32,
    // #[serde(skip_serializing)]
    pub password: String,
    pub service_time: i32,
    pub active: bool,
}
