use crate::prelude::*;
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ServerQueue {
    pub teller: ServerQuery,
    pub users: Vec<ClientQueueData>,
}

impl ServerQueue {
    pub fn new(teller: ServerQuery) -> Self {
        Self {
            teller,
            users: vec![],
        }
    }
}
impl SubQueues {
    pub fn teller_count(&self) -> usize {
        self.tellers.len()
    }
    pub fn teller_add(&mut self, teller: ServerQuery) -> Result<(), &'static str> {
        match self.teller_count() < SERVER_COUNT as usize {
            true => {
                self.tellers.push(ServerQueue::new(teller));
                info!("{:?}", self.tellers);
                Ok(())
            }
            false => Err("Server List full"),
        }
    }
    pub fn teller_remove(&mut self, index: usize) -> Result<ServerQueue, &'static str> {
        if index <= self.teller_count() {
            Ok(self.tellers.remove(index))
        } else {
            Err("Teller Not Available")
        }
    }
    pub fn teller_search(&self, station: usize) -> Result<&ServerQueue, &'static str> {
        if station < self.tellers.len() {
            Ok(&self.tellers[station])
        } else {
            Err("No Available Teller")
        }
    }
    pub fn teller_show_queue(&self, service_location: usize) -> Vec<ClientQueueData> {
        let teller = &self.tellers[service_location];
        if !teller.users.is_empty() {
            let mut results: Vec<ClientQueueData> = Vec::new();
            for client_data in teller.users.iter() {
                let unfocused_client_data = client_data.clone();
                results.push(unfocused_client_data);
            }
            results
        } else {
            vec![]
        }
    }
    pub fn teller_check_state(&self, service_location: usize) -> bool {
        self.tellers[service_location].teller.active
    }
}
