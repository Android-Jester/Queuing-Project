pub mod clients;
pub mod servers;

pub mod prelude {
    pub use super::clients::*;
    use super::servers::ServerQueue;
    pub use super::servers::*;
    use crate::prelude::*;
    #[derive(Default, Debug, Deserialize, Serialize, Clone)]
    pub struct SubQueues {
        pub tellers: Vec<ServerQueue>,
    }
}
