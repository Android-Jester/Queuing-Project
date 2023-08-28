use crate::prelude::*;

#[derive(Deserialize, Serialize)]
pub struct Analytics {
    average_service_time: f32,
    average_waiting_time: usize,
    average_queue_length: f64,
}

impl Analytics {
    pub fn new() -> Self {
        Self {
            average_service_time: get_service_time().unwrap(),
            average_waiting_time: waiting_queue(
                CUSTOMER_COUNT as f64,
                SERVER_COUNT as usize,
                combined_service_rate().unwrap(),
            ),
            average_queue_length: average_number_queue_customers(
                CUSTOMER_COUNT as f64,
                SERVER_COUNT as usize,
                combined_service_rate().unwrap(),
            ),
        }
    }
}
