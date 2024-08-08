pub mod clients;
pub mod servers;

pub mod prelude {
    
    use super::servers::ServerQueue;
    
    use crate::prelude::*;
    #[derive(Default, Debug, Deserialize, Serialize, Clone)]
    pub struct SubQueues {
        pub tellers: Vec<ServerQueue>,
    }
}
