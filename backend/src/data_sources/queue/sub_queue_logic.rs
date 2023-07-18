use crate::prelude::*;
#[derive(Default, Debug, Deserialize, Serialize, Clone)]
pub struct SubQueues {
    pub tellers: Vec<ServerQueue>,
}
