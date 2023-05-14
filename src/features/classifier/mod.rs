use chrono::{DateTime, TimeZone, Utc};

pub struct Teller {
    pub name: String,
    pub action: String,
    pub average_time: usize,
    pub best_time: usize,
}

impl Teller {
    fn new(name: String, action: String, average_time: usize, best_time: usize) -> Self {
        Self {
            name,
            average_time,
            best_time,
            action
        }
    }
    fn
}

trait TellerActions {
}