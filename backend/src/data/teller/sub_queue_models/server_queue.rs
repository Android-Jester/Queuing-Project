use crate::prelude::*;
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq, Eq)]
pub enum TellerState {
    Active,
    InActive,
    PendingRelease,
}
// #[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
// pub struct TellerQueueQuery {
//     pub server_id: String,
//     pub server_station: i32,
//     pub service_time: std::time::Duration,
//     pub teller_state: TellerState,
// }

#[derive(PartialEq, Debug, Deserialize, Serialize, Clone)]
pub struct ServerQueue {
    pub teller: TellerQuery,
    pub users: Vec<UserQueuePos>,
}

impl ServerQueue {
    pub fn new(teller: TellerQuery) -> Self {
        Self {
            teller,
            users: vec![],
        }
    }
}
